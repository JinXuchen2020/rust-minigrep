use std::{marker::PhantomPinned, pin::Pin};

#[derive(Debug)]
struct Test {
    a: String,
    b: *const String,
    _marker: PhantomPinned,
}

impl Test {
  fn _new(txt: &str) -> Self {
      Test {
          a: String::from(txt),
          b: std::ptr::null(),
          _marker: PhantomPinned, // 这个标记可以让我们的类型自动实现特征`!Unpin`
      }
  }

  // fn init(&mut self) {
  //     let self_ref: *const String = &self.a;
  //     self.b = self_ref;
  // }

  fn init(self: Pin<&mut Self>) {
    let self_ptr: *const String = &self.a;
    let this = unsafe { self.get_unchecked_mut() };
    this.b = self_ptr;
  }

  fn new(txt: &str) -> Pin<Box<Self>> {
    let t = Test {
        a: String::from(txt),
        b: std::ptr::null(),
        _marker: PhantomPinned,
    };
    let mut boxed = Box::pin(t);
    let self_ptr: *const String = &boxed.as_ref().a;
    unsafe { boxed.as_mut().get_unchecked_mut().b = self_ptr };

    boxed
  }

  fn a(self: Pin<&Self>) -> &str {
      &self.get_ref().a
  }

  fn b(self: Pin<&Self>) -> &String {
      assert!(!self.b.is_null(), "Test::b called without Test::init being called first");
      unsafe { &*(self.b) }
  }
}

pub fn pin_runner(){
  let mut test1 = Test::_new("test1");
  let mut test1 = unsafe { Pin::new_unchecked(&mut test1) };
  test1.as_mut().init();
  let mut test2 = Test::_new("test2");
  let mut test2 = unsafe { Pin::new_unchecked(&mut test2) };
  test2.as_mut().init();

  println!("a: {}, b: {}", test1.as_ref().a(), test1.as_ref().b());

  pin_drop();
  pin_box();
}

fn pin_drop() {
  let mut test1 = Test::_new("test1");
  let mut test1_pin = unsafe { Pin::new_unchecked(&mut test1) };
  Test::init(test1_pin.as_mut());

  drop(test1_pin);
  println!(r#"test1.b points to "test1": {:?}..."#, test1.b);

  let mut test2 = Test::_new("test2");
  std::mem::swap(&mut test1, &mut test2);
  println!("... and now it points nowhere: {:?}", test1.b);
}

fn pin_box() {
  let mut test1 = Test::new("test1");
  let mut test2 = Test::new("test2");

  println!("a: {}, b: {}", test1.as_ref().a(), test1.as_ref().b());
  println!("a: {}, b: {}", test2.as_ref().a(), test2.as_ref().b());

  std::mem::swap(&mut test1, &mut test2);

  println!("a: {}, b: {}", test1.as_ref().a(), test1.as_ref().b());
  println!("a: {}, b: {}", test2.as_ref().a(), test2.as_ref().b());
}