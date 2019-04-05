mod env;

use std::collections::HashMap;

use crate::object::cons_list::ConsList;
use crate::object::{self, Node, NodeRef, Program};

type BuiltinFn = fn(&mut VM, ConsList<NodeRef>) -> Result<NodeRef, String>;

#[derive(Default)]
pub struct VM {
    symbols: env::Env,
    builtins: HashMap<String, BuiltinFn>,
}

impl VM {
    pub fn new() -> Self {
        let mut vm = VM {
            symbols: env::Env::new(),
            builtins: HashMap::new(),
        };

        vm.builtins.insert("DEFVAR".to_owned(), builtin_defvar);
        vm.builtins.insert("PRINT".to_owned(), builtin_print);

        vm
    }

    pub fn run(&mut self, program: &Program) -> Result<(), String> {
        for form in program.iter() {
            self.eval(&form).map(|_| ())?
        }
        // println!("{:?}", self.symbols);
        Ok(())
    }

    fn eval(&mut self, form: &NodeRef) -> Result<NodeRef, String> {
        match &*form.borrow() {
            Node::Symbol(sym_ref) => {
                let sym = sym_ref.borrow();
                let sym_name = sym.name();
                match self.symbols.get_symbol(&sym_name) {
                    Some(v) => Ok(object::symbolref_to_noderef(v)), // TODO: Return value of symbol has it
                    None => Ok(Node::empty_list()),
                }
            }
            Node::Number(num) => Ok(Node::Number(*num).into_ref()),
            Node::String(string) => Ok(Node::String(string.to_owned()).into_ref()),
            Node::List(list) => self.eval_list(list),
            // Node::Function(Function),
            _ => Err("Not supported".to_owned()),
        }
    }

    fn eval_list(&mut self, form: &ConsList<NodeRef>) -> Result<NodeRef, String> {
        if let Some(h) = form.head() {
            match &*h.borrow() {
                Node::Symbol(sym_ref) => {
                    let sym = sym_ref.borrow();
                    if let Some(func) = self.builtins.get(sym.name()) {
                        func(self, form.tail())
                    } else {
                        Err(format!("Undefined function {}", sym.name()))
                    }
                }
                _ => Err("Cannot evaluate non-symbol object".to_owned()),
            }
        } else {
            Ok(Node::empty_list())
        }
    }
}

fn builtin_defvar(vm: &mut VM, args_list: ConsList<NodeRef>) -> Result<NodeRef, String> {
    // Collect into a vector to make it easier to work with args
    let args: Vec<NodeRef> = args_list.iter().cloned().collect();

    if args.len() != 2 {
        return Err(format!("defvar expected 2 args, got {}", args.len()));
    }

    let arg1 = args[0].borrow(); // Possibly a symbol reference

    let arg1_sym = match &*arg1 {
        Node::Symbol(sym) => sym,
        _ => return Err("defvar expected a symbol as arg 1".to_owned()),
    }; // Definitly a symbol reference

    let val = vm.eval(&args[1])?; // Evalute new value for symbol

    // Mutate symbol in a block so we can use it later
    {
        let mut arg1_sym_mut = arg1_sym.borrow_mut();
        arg1_sym_mut.value = Some(val);
    }

    // Store updated symbol in table
    vm.symbols.set_symbol(arg1_sym.clone());
    Ok(args[0].clone()) // Return symbol
}

fn builtin_print(vm: &mut VM, args_list: ConsList<NodeRef>) -> Result<NodeRef, String> {
    for arg in args_list.iter() {
        print!("{} ", vm.eval(arg)?.borrow());
    }
    println!("");
    Ok(object::Node::empty_list()) // Return nil
}
