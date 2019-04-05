extern crate clap;

use std::path::Path;

use lisp_vm::{compiler, vm};

use clap::{App, Arg};

fn main() {
    let app = App::new("Uncommon Lisp")
        .version("0.1.0")
        .author("Lee Keitel")
        .about("Execute a lisp file")
        .arg(Arg::with_name("INPUT").required(true))
        .get_matches();

    compile_file(app.value_of("INPUT").unwrap());
}

fn compile_file(path: &str) {
    println!("Compiling {}", path);
    let src_path = Path::new(path);
    let code = compiler::compile_file(src_path).unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(1);
    });
    println!("{:?}", code);

    let mut vm = vm::VM::new();
    if let Err(e) = vm.run(&code) {
        println!("Error: {}", e);
    }
}
