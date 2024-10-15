use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;

// 主人
#[derive(Debug)]
pub struct Owner {
  name: String,
  pub gadgets: RefCell<Vec<Weak<Gadget>>>,
}

#[derive(Debug)]
pub struct Gadget {
  id: i32,
  owner: Rc<Owner>,
}

impl Owner {
  pub fn new(name: &str) -> Self {
    Self {
      name: name.to_string(),
      gadgets: RefCell::new(vec![]),
    }
  }

  pub fn add_gadget(&self, gadget: &Rc<Gadget>) {
    self.gadgets.borrow_mut().push(Rc::downgrade(gadget));
  }
}

impl Gadget {
  pub fn new(id: i32, owner: Rc<Owner>) -> Self {
    Self { id, owner }
  }
}

