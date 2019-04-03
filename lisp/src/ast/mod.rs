pub mod list;

use std::rc::Rc;

use list::List;

pub enum Node {
    Symbol(Rc<Symbol>),
    Number(u64),
    String(String),
    List(Rc<List<Node>>),
    Function(Rc<Function>),
}

impl ::std::fmt::Display for Node {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            Node::Symbol(v) => write!(f, "{}", &v.name),
            Node::Number(v) => write!(f, "{}", v),
            Node::String(v) => write!(f, "\"{}\"", v),
            Node::List(v) => {
                let mut s = String::new();
                v.iter().for_each(|item| s.push_str(&format!("{} ", item)));
                write!(f, "({})", s.trim_end())
            }
            Node::Function(_) => write!(f, "function"),
        }
    }
}

impl ::std::fmt::Debug for Node {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", self)
    }
}

pub struct Symbol {
    name: String,
    value: Option<Rc<Node>>,
    function: Option<Rc<Function>>,
}

impl Symbol {
    pub fn new(name: &str) -> Self {
        Symbol {
            name: name.to_uppercase().to_owned(),
            value: None,
            function: None,
        }
    }
}

pub struct Function {
    params: Rc<List<Node>>,
    body: Rc<List<Node>>,
}
