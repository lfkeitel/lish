use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::object;

pub(crate) type EnvRef = Rc<RefCell<Env>>;

#[derive(Default)]
pub(crate) struct Env {
    syms: HashMap<String, object::SymbolRef>,
    parent: Option<EnvRef>,
}

impl Env {
    pub(crate) fn new() -> Self {
        Env {
            syms: HashMap::new(),
            parent: None,
        }
    }

    pub(crate) fn new_with_parent(parent: EnvRef) -> Self {
        Env {
            syms: HashMap::new(),
            parent: Some(parent),
        }
    }

    pub(crate) fn set_symbol(&mut self, sym: object::SymbolRef) {
        let sym_name = {
            let sym_ref = sym.borrow();
            sym_ref.name().to_owned()
        };
        self.syms.insert(sym_name, sym);
    }

    pub(crate) fn contains(&mut self, name: &str) -> bool {
        self.syms.contains_key(name)
    }

    pub(crate) fn get_symbol(&mut self, name: &str) -> Option<object::SymbolRef> {
        if let Some(sym) = self.syms.get(name) {
            return Some(sym.clone());
        }

        if let Some(p) = &self.parent {
            p.borrow_mut().get_symbol(name)
        } else {
            Some(object::Symbol::new(name).into_ref())
        }
    }
}

impl ::std::fmt::Debug for Env {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        for sym in self.syms.values() {
            let sym_ref = sym.borrow();
            writeln!(f, "{} = {:?}", sym_ref.name(), sym_ref)?
        }

        if let Some(p) = &self.parent {
            write!(f, "Parent:\n{:?}", p.borrow())
        } else {
            Ok(())
        }
    }
}
