pub mod list;

use std::hash::{Hash, Hasher};
use std::rc::Rc;

use list::ConsList;

pub enum Node {
    Symbol(Rc<Symbol>),
    Number(u64),
    String(String),
    List(Rc<ConsList<Node>>),
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
            value: Some(Rc::new(Node::String(name.to_owned()))),
            function: None,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn to_val_string(&self) -> String {
        if let Some(val) = &self.value {
            if let Node::String(s) = val.as_ref() {
                s.to_owned()
            } else {
                format!("{}", val)
            }
        } else {
            self.name.to_owned()
        }
    }
}

impl Hash for Symbol {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for Symbol {
    fn eq(&self, other: &Symbol) -> bool {
        self.name == other.name
    }
}

impl Eq for Symbol {}

pub struct Function {
    params: Rc<ConsList<Node>>,
    body: Rc<ConsList<Node>>,
}
