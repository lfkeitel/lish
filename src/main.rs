mod terminal;

use clap::{App, Arg};
use dirs::config_dir;
use path_absolutize::*;
use shellexpand::tilde;
use terminal::Terminal;

use lazuli_vm::args_setup;
use lazuli_vm::compiler;
use lazuli_vm::compiler::lexer::{ByteIter, Lexer};
use lazuli_vm::compiler::parser::Parser;
use lazuli_vm::object::cons_list::ConsList;
use lazuli_vm::object::{Callable, Node, Symbol, FALSE_KW, TRUE_KW};
use lazuli_vm::vm::VM;

use std::collections::HashMap;
use std::env;
use std::ffi::OsString;
use std::io::ErrorKind as io_error_kind;
use std::path;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

const DEFAULT_PROMPT: &str = "lish$ ";

fn main() {
    let app = App::new("Lish")
        .version("0.1.0")
        .author("Lee Keitel")
        .about("Lazuli Lisp Shell")
        .arg(
            Arg::with_name("startup-file")
                .short("s")
                .value_name("FILE")
                .takes_value(true)
                .help("Startup file to source before starting interactive shell"),
        )
        .arg(Arg::with_name("FILE"))
        .get_matches();

    match app.value_of("FILE") {
        Some(f) => compile_file(f),
        None => interactive_shell(app.value_of("startup-file")),
    }
}

fn compile_file(path: &str) {
    let src_path = Path::new(path);
    let code = compiler::compile_file(src_path).unwrap_or_else(|e| {
        eprintln!("{}", e);
        ::std::process::exit(1);
    });
    let abs_file_path = src_path
        .absolutize()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default()
        .to_owned();

    let mut vm = setup_vm(false);
    vm.add_symbol(
        Symbol::with_value("curr-script-path", Node::from_string(abs_file_path.clone())).into_ref(),
    );
    vm.add_filename(&abs_file_path);
    if let Err(e) = vm.run(&code) {
        eprintln!("Error: {}", e);
    }
}

fn setup_vm(interactive: bool) -> VM {
    let mut vm = VM::new();
    // Builtin Functions
    vm.add_symbol(Symbol::with_builtin("exit", shell_exit).into_ref());
    vm.add_symbol(Symbol::with_builtin("pwd", shell_pwd).into_ref());
    vm.add_symbol(Symbol::with_builtin("cd", shell_cd).into_ref());
    vm.add_symbol(Symbol::with_builtin("capc", shell_captured_call).into_ref());
    vm.add_symbol(Symbol::with_builtin("call", shell_call).into_ref());
    vm.add_symbol(Symbol::with_builtin("pipe", shell_pipe).into_ref());
    vm.add_symbol(Symbol::with_builtin("|", shell_pipe).into_ref());
    vm.add_symbol(Symbol::with_builtin("export", shell_export).into_ref());
    vm.add_symbol(Symbol::with_builtin("unexport", shell_unexport).into_ref());
    vm.add_symbol(Symbol::with_builtin("prompt", shell_default_prompt).into_ref());
    vm.add_symbol(Symbol::with_builtin("glob", shell_glob).into_ref());

    // Predefined variables
    vm.add_symbol(Symbol::with_value("interactive", Node::bool_obj(interactive)).into_ref());
    vm.add_symbol(Symbol::with_value("last-status", Node::Number(0)).into_ref());

    for (key, value) in env::vars() {
        vm.add_symbol(Symbol::with_value(&key, Node::from_string(value)).into_ref());
    }

    // Override VM define so we can disable interactive shell during exec
    let vm_define = vm.symbols.borrow().get_symbol("define");
    let mut shadow_define = Symbol::new("_define");
    shadow_define.function = vm_define.borrow().function.clone();
    vm.add_symbol(shadow_define.into_ref());
    vm.add_symbol(Symbol::with_builtin("define", shell_define).into_ref());

    vm.set_cmd_not_found(Callable::Builtin(shell_call));
    vm
}

fn get_default_rc_filepath() -> Option<PathBuf> {
    match config_dir() {
        Some(p) => Some(p.join("lish").join("init.lisp")),
        None => None,
    }
}

fn get_default_history_filepath() -> PathBuf {
    match config_dir() {
        Some(p) => p.join("lish").join(".history"),
        None => PathBuf::from(".lish_history"),
    }
}

fn interactive_shell(startup_file: Option<&str>) {
    let mut term = Terminal::new(get_default_history_filepath());
    if let Err(e) = term.load_history() {
        eprintln!("{}", e);
    }

    let mut vm = setup_vm(true);

    let rc_path = match startup_file {
        Some(filename) => Some(Path::new(filename).to_path_buf()),
        None => get_default_rc_filepath(),
    };

    // Source startup file if one is given
    if let Some(filename) = rc_path {
        if filename.exists() {
            match compiler::compile_file(&filename) {
                Ok(code) => {
                    vm.add_filename(filename.absolutize().unwrap_or_default());
                    if let Err(e) = vm.run(&code) {
                        eprintln!("Error: {}", e);
                    }
                    vm.pop_filename();
                }
                Err(e) => eprintln!("{}", e),
            };
        }
    }

    let prompt_func =
        ConsList::new().append(Symbol::with_value("prompt", Node::empty_list()).into_node());

    loop {
        let mut line = match vm.eval_list(&prompt_func) {
            Ok(node) => match node {
                Node::String(s) => term.readline(&s),
                _ => {
                    println!(
                        "prompt didn't return a String, returned {}",
                        node.type_str()
                    );
                    term.readline(&DEFAULT_PROMPT)
                }
            },
            _ => term.readline(&DEFAULT_PROMPT),
        };

        if !line.starts_with('(') {
            line = format!("({})", line);
        }
        let mut line_iter = line.bytes();
        let mut str_iter = ByteIter::new(&mut line_iter);
        let mut lex = Lexer::new(&mut str_iter, "<shell>");
        let parser = Parser::new(&mut lex);
        let tree = match parser.parse() {
            Ok(t) => t,
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
        };

        match vm.run(&tree) {
            Ok(v) => match v {
                Node::Empty | Node::Symbol(_) => {}
                _ => println!("{}", v),
            },
            Err(e) => eprintln!("{}", e),
        }
    }
}

fn shell_default_prompt(_vm: &mut VM, _args: ConsList<Node>) -> Result<Node, String> {
    Ok(Node::from_string(DEFAULT_PROMPT.to_string()))
}

fn shell_exit(vm: &mut VM, args: ConsList<Node>) -> Result<Node, String> {
    let args = args_setup!(args);
    let status = if !args.is_empty() {
        match vm.eval(args[0])? {
            Node::Number(n) => {
                if n >= 0 && n <= 255 {
                    n as i32
                } else {
                    255 // Not between 0 - 255
                }
            }
            _ => 255, // Not a Number
        }
    } else {
        0 // No argument
    };

    ::std::process::exit(status);
}

fn shell_pwd(_vm: &mut VM, _args: ConsList<Node>) -> Result<Node, String> {
    let n: Node = env::current_dir()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default()
        .parse()
        .unwrap();
    Ok(n)
}

fn shell_cd(vmc: &mut VM, args: ConsList<Node>) -> Result<Node, String> {
    let args = args_setup!(args, "cd", ==, 1);

    let evaled_arg = vmc.eval(args[0])?;

    let new_path_str = match &evaled_arg {
        Node::String(s) => tilde(s).into_owned(),
        Node::Symbol(sym) => match sym.borrow().value() {
            Node::String(s) => tilde(&s).into_owned(),
            _ => return Err("cd expected a string".to_owned()),
        },
        _ => "".to_owned(),
    };

    let new_path_segment = path::Path::new(&new_path_str);

    let res = env::set_current_dir(if new_path_segment.is_absolute() {
        new_path_segment.absolutize().unwrap().to_path_buf()
    } else {
        let pwd = env::current_dir().unwrap_or_default();
        let new_path = pwd.join(new_path_segment);
        new_path.absolutize().unwrap().to_path_buf()
    });

    match res {
        Ok(_) => Ok(Node::Empty),
        Err(e) => Err(format!("{}", e)),
    }
}

fn shell_glob(vmc: &mut VM, args: ConsList<Node>) -> Result<Node, String> {
    let args = args_setup!(args, "glob", >=, 1);

    let evaled_arg = vmc.eval(args[0])?;

    let new_path_str = match &evaled_arg {
        Node::String(s) => tilde(s).into_owned(),
        Node::Symbol(sym) => match sym.borrow().value() {
            Node::String(s) => tilde(&s).into_owned(),
            _ => return Err("glob expected a string".to_owned()),
        },
        _ => "".to_owned(),
    };

    let max_depth = if args.len() > 1 {
        match &vmc.eval(args[1])? {
            Node::Number(i) => *i as usize,
            Node::Symbol(sym) => match sym.borrow().value() {
                Node::Number(i) => i as usize,
                _ => 1,
            },
            _ => 1,
        }
    } else {
        1
    };

    let mut list = ConsList::new();

    let walker = make_glob_walker(new_path_str, max_depth)
        .unwrap()
        .filter_map(Result::ok);

    for img in walker {
        list = list.append(Node::from_string(
            img.path()
                .as_os_str()
                .to_string_lossy()
                .to_owned()
                .to_string(),
        ));
    }

    Ok(Node::List(list))
}

fn make_glob_walker<S: AsRef<str>>(
    pattern: S,
    max_depth: usize,
) -> Result<globwalk::GlobWalker, globwalk::GlobError> {
    let path_pattern: PathBuf = pattern.as_ref().into();
    if path_pattern.is_absolute() {
        // If the pattern is an absolute path, split it into the longest base and a pattern.
        let mut base = PathBuf::new();
        let mut pattern = PathBuf::new();
        let mut globbing = false;

        // All `to_str().unwrap()` calls should be valid since the input is a string.
        for c in path_pattern.components() {
            let os = c.as_os_str().to_str().unwrap();
            for c in &["*", "{", "}"][..] {
                if os.contains(c) {
                    globbing = true;
                    break;
                }
            }

            if globbing {
                pattern.push(c);
            } else {
                base.push(c);
            }
        }

        let pat = pattern.to_str().unwrap();
        if cfg!(windows) {
            globwalk::GlobWalkerBuilder::new(base.to_str().unwrap(), pat.replace("\\", "/"))
                .max_depth(max_depth)
                .build()
        } else {
            globwalk::GlobWalkerBuilder::new(base.to_str().unwrap(), pat)
                .max_depth(max_depth)
                .build()
        }
    } else {
        // If the pattern is relative, start searching from the current directory.
        globwalk::GlobWalkerBuilder::new(".", pattern)
            .max_depth(max_depth)
            .build()
    }
}

fn is_interactive(vm: &mut VM) -> bool {
    let node = vm.symbols.borrow().get_symbol("interactive");
    let node_val = node.borrow().value();
    node_val.is_truthy()
}

fn set_interactive(vm: &mut VM, i: bool) {
    let node_ref = vm.symbols.borrow().get_symbol("interactive");
    let mut node = node_ref.borrow_mut();
    if i {
        TRUE_KW.with(|t| {
            node.value = Some(t.clone());
        });
    } else {
        FALSE_KW.with(|f| {
            node.value = Some(f.clone());
        });
    }
}

fn shell_captured_call(vm: &mut VM, args: ConsList<Node>) -> Result<Node, String> {
    set_interactive(vm, false);
    let res = shell_call(vm, args);
    set_interactive(vm, true);
    res
}

fn shell_call(vm: &mut VM, args: ConsList<Node>) -> Result<Node, String> {
    let args = args_setup!(args, "call", >=, 1);

    let command_name = vm.eval(&args[0])?;
    let mut cmd = Command::new(format!("{}", command_name));

    for arg in args.iter().skip(1) {
        cmd.arg(format!("{}", vm.eval(arg)?));
    }

    if is_interactive(vm) {
        match cmd.status() {
            Ok(out) => {
                vm.add_symbol(
                    Symbol::with_value(
                        "last-status",
                        Node::Number(i64::from(out.code().unwrap_or(255))),
                    )
                    .into_ref(),
                );
                Ok(Node::Empty)
            }
            Err(e) => {
                vm.add_symbol(Symbol::with_value("last_status", Node::Number(255)).into_ref());

                match e.kind() {
                    io_error_kind::NotFound => Err(format!("Command not found {}", command_name)),
                    _ => Err(format!("{}", e)),
                }
            }
        }
    } else {
        let mut map = HashMap::new();
        cmd.stdin(Stdio::null());

        match cmd.output() {
            Ok(out) => {
                map.insert(
                    ":stdout".to_owned(),
                    Node::String(
                        String::from_utf8(out.stdout)
                            .unwrap_or_default()
                            .trim()
                            .to_owned(),
                    ),
                );
                map.insert(
                    ":stderr".to_owned(),
                    Node::String(
                        String::from_utf8(out.stderr)
                            .unwrap_or_default()
                            .trim()
                            .to_owned(),
                    ),
                );
                map.insert(
                    ":status".to_owned(),
                    Node::Number(i64::from(out.status.code().unwrap_or(255))),
                );
            }
            Err(e) => {
                map.insert(":stdout".to_owned(), Node::String("".to_owned()));
                map.insert(":stderr".to_owned(), Node::String(format!("{}", e)));
                map.insert(":status".to_owned(), Node::Number(255));
            }
        }
        Ok(Node::from_hashmap(map))
    }
}

fn shell_pipe(vm: &mut VM, args: ConsList<Node>) -> Result<Node, String> {
    let args = args_setup!(args, "pipe", >=, 1);

    let mut parent_cmd = if let Node::List(l) = args[0] {
        let mut args: Vec<OsString> = Vec::new();
        for arg in l.iter().skip(1) {
            args.push(format!("{}", vm.eval(arg)?).into());
        }

        duct::cmd(
            format!("{}", vm.eval(l.head().unwrap_or(&Node::Empty))?),
            &args,
        )
    } else {
        return Err(format!(
            "pipe args must by lists, got {}",
            args[0].type_str()
        ));
    };

    for cmd in args.iter().skip(1) {
        parent_cmd = parent_cmd.pipe(if let Node::List(l) = cmd {
            let mut args: Vec<OsString> = Vec::new();
            for arg in l.iter().skip(1) {
                args.push(format!("{}", vm.eval(arg)?).into());
            }

            duct::cmd(
                format!("{}", vm.eval(l.head().unwrap_or(&Node::Empty))?),
                &args,
            )
        } else {
            return Err(format!("pipe args must by lists, got {}", cmd.type_str()));
        });
    }

    let mut map = HashMap::new();
    parent_cmd = parent_cmd.unchecked();

    if is_interactive(vm) {
        match parent_cmd.run() {
            Ok(out) => {
                vm.add_symbol(
                    Symbol::with_value(
                        "last-status",
                        Node::Number(i64::from(out.status.code().unwrap_or(255))),
                    )
                    .into_ref(),
                );
                Ok(Node::Empty)
            }
            Err(e) => {
                vm.add_symbol(Symbol::with_value("last-status", Node::Number(255)).into_ref());
                Err(format!("{}", e))
            }
        }
    } else {
        parent_cmd = parent_cmd.stdin_null();
        parent_cmd = parent_cmd.stdout_capture();
        parent_cmd = parent_cmd.stderr_capture();

        match parent_cmd.run() {
            Ok(out) => {
                map.insert(
                    ":stdout".to_owned(),
                    Node::String(String::from_utf8(out.stdout).unwrap_or_default()),
                );
                map.insert(
                    ":stderr".to_owned(),
                    Node::String(String::from_utf8(out.stderr).unwrap_or_default()),
                );
                map.insert(
                    ":status".to_owned(),
                    Node::Number(i64::from(out.status.code().unwrap_or(255))),
                );
            }
            Err(e) => {
                map.insert(":stdout".to_owned(), Node::String("".to_owned()));
                map.insert(":stderr".to_owned(), Node::String(format!("{}", e)));
                map.insert(":status".to_owned(), Node::Number(255));
            }
        }
        Ok(Node::from_hashmap(map))
    }
}

fn shell_export(vm: &mut VM, args: ConsList<Node>) -> Result<Node, String> {
    let args = args_setup!(args, "export", ==, 1);

    if let Node::Symbol(s) = vm.eval(&args[0])? {
        let sym_name = s.borrow().name().to_owned();
        let real_sym = vm.symbols.borrow().get_symbol(&sym_name);
        let real_sym_b = real_sym.borrow();
        let value = format!("{}", real_sym_b.value());
        env::set_var(sym_name, value);
        Ok(Node::Empty)
    } else {
        Err("export requires a Symbol as the first argument".to_owned())
    }
}

fn shell_unexport(vm: &mut VM, args: ConsList<Node>) -> Result<Node, String> {
    let args = args_setup!(args, "unexport", ==, 1);

    if let Node::Symbol(s) = vm.eval(&args[0])? {
        env::remove_var(s.borrow().name());
        Ok(Node::Empty)
    } else {
        Err("export requires a Symbol as the first argument".to_owned())
    }
}

fn shell_define(vm: &mut VM, args: ConsList<Node>) -> Result<Node, String> {
    set_interactive(vm, false);

    let new_args = args.append(Symbol::new("_define").into_node());
    let ret = vm.eval_list(&new_args);

    set_interactive(vm, true);

    ret
}
