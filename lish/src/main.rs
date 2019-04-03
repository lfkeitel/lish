mod terminal;

use std::io::Write as IoWrite;

use terminal::Terminal;

use lisp::ast::{self, list};
use lisp::parser::lexer::{ByteIter, Lexer};
use lisp::parser::parser::Parser;
use path_absolutize::*;
use std::collections::HashMap;
use std::env;
use std::path;
use std::process::Command;

const PROMPT: &str = "lish$ ";

type AstList = list::ConsList<ast::Node>;
type SimpleFn = fn(&mut Shell, &AstList);

thread_local!(
    static FUNCS: HashMap<ast::Symbol, SimpleFn> = {
        let mut m = HashMap::new();
        m.insert(ast::Symbol::new("EXIT"), shell_exit as SimpleFn);
        m.insert(ast::Symbol::new("PWD"), shell_pwd as SimpleFn);
        m.insert(ast::Symbol::new("CD"), shell_cd as SimpleFn);
        m
    };
);

struct Shell {
    term: Terminal,
    pwd: path::PathBuf,
}

impl Shell {
    fn run(&mut self) {
        loop {
            let mut line = self.term.readline(&PROMPT);
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
            self.execute_tree(tree);
        }
    }

    fn execute_tree(&mut self, tree: ast::Node) {
        if let ast::Node::List(tree_list) = &tree {
            for form in tree_list.iter() {
                if let ast::Node::List(l) = &form {
                    let sym = ast::Symbol::new(match l.head() {
                        Some(h) => {
                            if let ast::Node::Symbol(s) = h {
                                s.name()
                            } else {
                                continue;
                            }
                        }
                        None => continue,
                    });

                    let some_func = FUNCS.with(|m| m.get(&sym).cloned());

                    if let Some(f) = some_func {
                        if let ast::Node::List(tree) = &form {
                            f(self, &tree);
                        }
                    } else {
                        let cmd_path = sym.name().to_lowercase();
                        let args: Vec<String> = l.tail().iter().map(|i| format!("{}", i)).collect();

                        match Command::new(cmd_path)
                            .args(&args)
                            .current_dir(self.pwd.as_path())
                            .spawn()
                        {
                            Ok(mut child) => {
                                child.wait().expect("Child wait failed");
                            }
                            Err(e) => println!("{}", e),
                        }
                    }
                }
            }
        }
    }

    fn print(&mut self, s: &str) {
        write!(self.term, "{}", s).unwrap();
    }

    fn println(&mut self, s: &str) {
        writeln!(self.term, "{}", s).unwrap();
    }
}

fn main() {
    let mut shell = Shell {
        term: Terminal::new(),
        pwd: env::current_dir().unwrap_or_default(),
    };
    shell.run();
}

fn shell_exit(_: &mut Shell, _: &AstList) {
    ::std::process::exit(0);
}

fn shell_pwd(shell: &mut Shell, _: &AstList) {
    shell.println(&format!("{}", shell.pwd.display()));
}

fn shell_cd(shell: &mut Shell, args: &AstList) {
    if let Some(p) = args.tail().head() {
        let new_path_str = match p {
            ast::Node::String(s) => s.to_owned(),
            ast::Node::Symbol(sym) => sym.to_val_string(),
            _ => "".to_owned(),
        };

        let new_path_segment = path::Path::new(&new_path_str);

        if new_path_segment.is_absolute() {
            shell.pwd = new_path_segment.absolutize().unwrap().to_path_buf();
        } else {
            let new_path = shell.pwd.join(new_path_segment);
            shell.pwd = new_path.absolutize().unwrap().to_path_buf();
        }
    }
}
