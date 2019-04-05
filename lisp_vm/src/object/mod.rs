pub mod cons_list;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use cons_list::ConsList;

pub type Program = ConsList<NodeRef>;

impl ::std::fmt::Debug for Program {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let mut s = String::new();
        self.iter()
            .for_each(|item| s.push_str(&format!("{} ", item.borrow())));
        write!(f, "({})", s.trim_end())
    }
}

pub type NodeRef = Rc<RefCell<Node>>;

pub enum Node {
    Symbol(SymbolRef),
    Number(u64),
    String(String),
    List(ConsList<NodeRef>),
    Function(Function),
}

impl Node {
    pub fn empty_list() -> NodeRef {
        Rc::new(RefCell::new(Node::List(ConsList::new())))
    }

    pub fn into_ref(self) -> NodeRef {
        Rc::new(RefCell::new(self))
    }
}

impl ::std::fmt::Display for Node {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            Node::Symbol(v) => write!(f, "{}", v.borrow()),
            Node::Number(v) => write!(f, "{}", v),
            Node::String(v) => write!(f, "\"{}\"", v),
            Node::List(v) => {
                let mut s = String::new();
                v.iter()
                    .for_each(|item| s.push_str(&format!("{} ", item.borrow())));
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

type SymbolProps = HashMap<Symbol, Node>;
pub type SymbolRef = Rc<RefCell<Symbol>>;

pub fn symbolref_to_noderef(sym: SymbolRef) -> NodeRef {
    Node::Symbol(sym).into_ref()
}

pub struct Symbol {
    name: String,
    pub value: Option<NodeRef>, // Used when this symbol is evaulated outside a callable context
    function: Option<Callable>, // Used when this symbol is evaluated as a callable object
    properties: Option<SymbolProps>, // Only created when needed
}

impl Symbol {
    pub fn new(name: &str) -> Self {
        Symbol {
            name: name.to_uppercase().to_owned(),
            value: None,
            function: None,
            properties: None,
        }
    }

    pub fn into_ref(self) -> SymbolRef {
        Rc::new(RefCell::new(self))
    }

    pub fn into_noderef(self) -> NodeRef {
        Node::Symbol(self.into_ref()).into_ref()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> NodeRef {
        if let Some(val) = &self.value {
            val.clone()
        } else {
            Node::String(self.name.clone()).into_ref()
        }
    }
}

impl ::std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match &*self.value().borrow() {
            Node::String(s) => write!(f, "{}", s),
            n => write!(f, "{}", n),
        }
    }
}

impl ::std::fmt::Debug for Symbol {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", self)
    }
}

pub struct Function {
    is_macro: bool,
    params: Vec<String>,
    body: ConsList<NodeRef>,
}

pub enum Callable {
    Builtin,
    User(Function),
}
