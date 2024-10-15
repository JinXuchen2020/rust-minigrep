use std::cell::RefCell;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::ptr::NonNull;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct Node {
  value: i32,
  parent: RefCell<Weak<Node>>,
  children: RefCell<Vec<Rc<Node>>>,
}

impl Node {
  pub fn new(value: i32) -> Node {
    Node {
      value,
      parent: RefCell::new(Weak::new()),
      children: RefCell::new(vec![]),
    }
  }

  pub fn add_child(&self, child: Rc<Node>) {
    self.children.borrow_mut().push(child.clone());
  }

  pub fn set_parent(&self, parent: Weak<Node>) {
    *self.parent.borrow_mut() = parent;
  }

  pub fn get_value(&self) -> i32 {
    self.value
  }

  pub fn get_parent(&self) -> Option<Rc<Node>> {
    self.parent.borrow().upgrade()
  }

  pub fn get_children(&self) -> Vec<Rc<Node>> {
    self.children.borrow().clone()
  }
}

#[derive(Debug)]
pub struct SelfRef {
  value: String,
  pointer_to_value: *const String,
}

impl SelfRef {
  pub fn new(txt: &str) -> Self {
      SelfRef {
        value: String::from(txt),
        pointer_to_value: std::ptr::null(),
      }
  }

  pub fn init(&mut self) {
      let self_ref: *const String = &self.value;
      self.pointer_to_value = self_ref;
  }

  pub fn value(&self) -> &str {
      &self.value
  }

  pub fn pointer_to_value(&self) -> &String {
      assert!(!self.pointer_to_value.is_null(),
          "Test::b called without Test::init being called first");
      unsafe { &*(self.pointer_to_value) }
  }
}

// 下面是一个自引用数据结构体，因为 slice 字段是一个指针，指向了 data 字段
// 我们无法使用普通引用来实现，因为违背了 Rust 的编译规则
// 因此，这里我们使用了一个裸指针，通过 NonNull 来确保它不会为 null
#[derive(Debug)]
pub struct Unmovable {
  pub data: String,
  pub slice: NonNull<String>,
  _pin: PhantomPinned,
}

impl Unmovable {
  // 为了确保函数返回时数据的所有权不会被转移，我们将它放在堆上，唯一的访问方式就是通过指针
  pub fn new(data: String) -> Pin<Box<Self>> {
    let res = Unmovable {
      data,
      // 只有在数据到位时，才创建指针，否则数据会在开始之前就被转移所有权
      slice: NonNull::dangling(),
      _pin: PhantomPinned,
    };
    let mut boxed = Box::pin(res);

    let slice = NonNull::from(&boxed.data);
    // 这里其实安全的，因为修改一个字段不会转移整个结构体的所有权
    unsafe {
      let mut_ref: Pin<&mut Self> = Pin::as_mut(&mut boxed);
      Pin::get_unchecked_mut(mut_ref).slice = slice;
    }
    boxed
  }
}