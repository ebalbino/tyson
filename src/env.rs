use crate::value::ASTNode;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, PartialEq, Default)]
pub struct Env<'arena> {
    parent: Option<Rc<RefCell<Env<'arena>>>>,
    vars: HashMap<&'arena str, ASTNode<'arena>>,
}

impl<'arena> Env<'arena> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn update(&mut self, data: Rc<RefCell<Self>>) {
        self.vars.extend(
            data.borrow()
                .vars
                .iter()
                .map(|(k, v)| (k.clone(), v.clone())),
        );
    }

    pub fn extend(parent: Rc<RefCell<Self>>) -> Self {
        Env {
            vars: HashMap::new(),
            parent: Some(parent),
        }
    }

    pub fn get(&self, name: &str) -> Option<ASTNode<'arena>> {
        match self.vars.get(name) {
            Some(value) => Some(value.clone()),
            None => self
                .parent
                .as_ref()
                .and_then(|o| o.borrow().get(name).clone()),
        }
    }

    pub fn set(&mut self, name: &'arena str, val: ASTNode<'arena>) {
        self.vars.insert(name, val);
    }
}
