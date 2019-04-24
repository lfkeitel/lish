mod terminal;

use terminal::Terminal;

use lazuli_vm::compiler::lexer::{ByteIter, Lexer};
use lazuli_vm::compiler::parser::Parser;
use lazuli_vm::object::cons_list::ConsList;
use lazuli_vm::object::{Callable, Node, Symbol};
use lazuli_vm::vm::VM;
use lazuli_vm::{args_setup, args_setup_error};

use path_absolutize::*;
use std::collections::HashMap;
use std::env;
use std::path;
use std::process::Command;

const PROMPT: &str = "lish$ ";

fn main() {
    let mut term = Terminal::new();

    let mut vm = VM::new();
    vm.add_symbol(Symbol::new_with_builtin("exit", shell_exit).into_ref());
    vm.add_symbol(Symbol::new_with_builtin("pwd", shell_pwd).into_ref());
    vm.add_symbol(Symbol::new_with_builtin("cd", shell_cd).into_ref());
    vm.add_symbol(Symbol::new_with_builtin("call", shell_call).into_ref());

    vm.set_cmd_not_found(Callable::Builtin(shell_call));

    loop {
        let mut line = term.readline(&PROMPT);
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
                println!("{}", e);
                continue;
            }
        };

        // println!("{:?}", tree);
        match vm.run(&tree) {
            Ok(v) => println!("{}", v),
            Err(e) => println!("{}", e),
        }
    }
}

fn shell_exit(_vm: &mut VM, _args: ConsList<Node>) -> Result<Node, String> {
    ::std::process::exit(0);
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
        Node::String(s) => s.to_owned(),
        Node::Symbol(sym) => match sym.borrow().value() {
            Node::String(s) => s.to_owned(),
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
        Ok(_) => Ok(Node::empty_list()),
        Err(e) => Err(format!("{}", e)),
    }
}

fn shell_call(vm: &mut VM, args: ConsList<Node>) -> Result<Node, String> {
    let args = args_setup!(args, "call", >=, 1);

    let mut cmd = Command::new(format!("{}", vm.eval(&args[0])?));

    for arg in args.iter().skip(1) {
        cmd.arg(format!("{}", vm.eval(arg)?));
    }

    let res = cmd.output();

    let mut map = HashMap::new();

    match res {
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
