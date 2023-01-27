use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::object::Object;

#[derive(Default, Debug, PartialEq)]
pub struct Meta {
    parent: Option<Rc<RefCell<Meta>>>,
    vars: HashMap<String, Object>,
}

impl Meta {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn extend(parent: Rc<RefCell<Self>>) -> Meta {
        Self {
            vars: HashMap::new(),
            parent: Some(parent),
        }
    }

    pub fn get(&self, name: &str) -> Option<Object> {
        match self.vars.get(name) {
            Some(val) => Some(val.clone()),
            None => self
                .parent
                .as_ref()
                .and_then(|o| o.borrow()
                    .get(name)
                    .clone()),
        }
    }

    pub fn set(&mut self, name: &str, val: Object) {
        self.vars.insert(name.to_string(), val);
    }
}