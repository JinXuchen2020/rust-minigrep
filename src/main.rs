use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::hash::Hash;
use std::env;
use std::process;
use std::rc::Rc;
use std::thread;
use apperror::render;
use apperror::run;
use apperror::AppError;
use async_concept::async_base;
use atomic::atomic_base;
use atomic::atomic_with_arc;
use atomic::atomic_with_once;
use channel::channel_base;
use channel::channel_mul_sender;
use channel::channel_send_multiple_values;
use channel::channel_send_string;
use channel::channel_sync_send;
use futures::executor::block_on;
use gadget::Gadget;
use minigrep::Config;
use mutilthread::barrier_in_thread;
use mutilthread::create_threads;
use mutilthread::loop_in_thread;
use mutilthread::mutex_in_thread;
use mutilthread::once_in_thread;
use mutilthread::thread_local_in_thread;
use node::SelfRef;
use node::Unmovable;
use pin_concept::pin_runner;
use sharememory::share_memory_base;
use sharememory::share_memory_with_condvar;
use sharememory::share_memory_with_deadlock;
use sharememory::share_memory_with_thread;
use threadtool::ThreadPool;
use timer_future::runner;
use unsafe_concept::unsafe_raw_pointer;
use crate::List::{Cons, Nil};
use std::cell::RefCell;

mod point;
mod cacher;
mod list;
mod gadget;
mod node; // 新增的模块
mod mutilthread;
mod channel;
mod sharememory;
mod atomic;
mod apperror;
mod unsafe_concept;
mod async_concept;
mod timer_future;
mod pin_concept;
mod web_server;
mod threadtool;

use node::Node;
use gadget::Owner;
use point::Point;
use cacher::Cacher;
use list::List;

fn main()  {
  let args = env::args();

  let config = Config::build(args).unwrap_or_else(|err| {
    eprintln!("Problem parsing arguments: {err}");
    process::exit(1);
  });

  config.print();

  if let Err(err) = config.run() {
    eprintln!("Application error: {err}");
    process::exit(1);
  }

  let mut p = Point { x: 0,  y: 0 };
  let r = &mut p;

  let rr: &mut Point = &mut *r;
  rr.move_to(2, 3);
  println!("{:p}", rr);
  println!("{:?}", rr);

  let rrr: &mut Point = &mut *rr;
  rrr.move_to(3, 4);
  println!("{:p}", rrr);
  println!("{:?}", rrr);

  println!("{:p}", r);
  println!("{:?}", r);

  let x = vec![1, 2, 3];
  fn_once(|z|{z == x.len()});

  let mut s = String::new();

  let mut update_string =  |str| s.push_str(str);
  update_string("hello");

  let s = String::new();

  let update_string =  move || println!("{}",s.len());

  update_string();
  update_string();

  let temp = "str".to_string();
  do4(temp);

  //println!("{}", temp);

  let mut temp1 = "str".to_string();

  do3(&mut temp1);

  println!("{}", temp1);

  let mut temp2 = "str".to_string();

  do5(&mut temp2);

  println!("{}", temp2);

  let values = vec![1, 2, 3];

  {
      let result = match values.into_iter() {
          mut iter => loop {
              match iter.next() {
                  Some(x) => { println!("{}", x); },
                  None => break,
              }
          },
      };
      result
  }

  let values = vec![1, 2, 3];
  for v in values {
      println!("{}", v)
  }

  list_test();
  gadget_test();
  node_test();
  self_ref_test();

  create_threads();

  //loop_in_thread();

  barrier_in_thread();

  thread_local_in_thread();

  mutex_in_thread();

  once_in_thread();

  // channel_base();
  // channel_send_string();
  // channel_send_multiple_values();
  // channel_mul_sender();
  // channel_sync_send();

  // share_memory_base();
  // share_memory_with_thread();
  // share_memory_with_deadlock();
  // share_memory_with_condvar();

  // atomic_base();
  // atomic_with_arc();
  // atomic_with_once();

  //unsafe_raw_pointer();
  // let future = async_base();
  // block_on(future);

  // runner();

  // pin_runner();
  // block_on(web_server::runner());

  let pool = ThreadPool::new(4);
  pool.execute(|| {thread::sleep(std::time::Duration::from_secs(5));});
  pool.execute(|| {thread::sleep(std::time::Duration::from_secs(5));});
  pool.execute(|| {thread::sleep(std::time::Duration::from_secs(5));});
  pool.execute(|| {thread::sleep(std::time::Duration::from_secs(5));});
  pool.execute(|| {thread::sleep(std::time::Duration::from_secs(5));});
  pool.execute(|| {thread::sleep(std::time::Duration::from_secs(5));});
  pool.execute(|| {thread::sleep(std::time::Duration::from_secs(5));});

  // thread::sleep(std::time::Duration::from_secs(10));

  // if let Err(err) = run() {
  //   return Err(Box::new(err));
  // }

  // Ok(())

  //render()

}

fn fn_once<F>(func: F)
where
    F: FnOnce(usize) -> bool + Copy,
{
    println!("{}", func(3));
    println!("{}", func(4));
}

#[allow(dead_code)]
fn factory(x:i32) -> Box<dyn Fn(i32) -> i32> {
  let num = 5;

  if x > 1{
      Box::new(move |x| x + num)
  } else {
      Box::new(move |x| x - num)
  }
}

fn do3(c: &mut String) {
  c.push_str("4");
  println!("{}", c);
}

fn do4(mut c: String) {
  c.push_str("4");
  println!("{}", c);
}

fn do5(c: &mut String) {
  c.push_str("4");
  println!("{}", c);
}

// fn my_function(n: usize) {
//   let array = [123; n];
//   let s1: Box<str> = "Hello there!".into();
//   let a = Rc::new(String::from("test ref counting"));
//   println!("count after creating a = {}", Rc::strong_count(&a));
//   let b =  Rc::clone(&a);
// }

fn list_test() {
  let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

  println!("a的初始化rc计数 = {}", Rc::strong_count(&a));
  println!("a指向的节点 = {:?}", a.tail());

  // 创建`b`到`a`的引用
  let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

  println!("在b创建后，a的rc计数 = {}", Rc::strong_count(&a));
  println!("b的初始化rc计数 = {}", Rc::strong_count(&b));
  println!("b指向的节点 = {:?}", b.tail());

  // 利用RefCell的可变性，创建了`a`到`b`的引用
  if let Some(link) = a.tail() {
      *link.borrow_mut() = Rc::clone(&b);
  }

  println!("在更改a后，b的rc计数 = {}", Rc::strong_count(&b));
  println!("在更改a后，a的rc计数 = {}", Rc::strong_count(&a));

  // 下面一行println!将导致循环引用
  // 我们可怜的8MB大小的main线程栈空间将被它冲垮，最终造成栈溢出
  //println!("a next item = {:?}", a.tail());
}

fn gadget_test() {
  // 创建一个 Owner
    // 需要注意，该 Owner 也拥有多个 `gadgets`
    let gadget_owner : Rc<Owner> = Rc::new(
      Owner::new("Gadget Man") 
    );

  // 创建工具，同时与主人进行关联：创建两个 gadget，他们分别持有 gadget_owner 的一个引用。
  let gadget1 = Rc::new(
    Gadget::new(1, gadget_owner.clone())
  );
  let gadget2 = Rc::new(Gadget::new(2, gadget_owner.clone()));

  // 为主人更新它所拥有的工具
  // 因为之前使用了 `Rc`，现在必须要使用 `Weak`，否则就会循环引用
  gadget_owner.add_gadget(&gadget1);
  gadget_owner.add_gadget(&gadget2);

  // 遍历 gadget_owner 的 gadgets 字段
  for gadget_opt in gadget_owner.gadgets.borrow().iter() {

      // gadget_opt 是一个 Weak<Gadget> 。 因为 weak 指针不能保证他所引用的对象
      // 仍然存在。所以我们需要显式的调用 upgrade() 来通过其返回值(Option<_>)来判
      // 断其所指向的对象是否存在。
      // 当然，Option 为 None 的时候这个引用原对象就不存在了。
      let gadget = gadget_opt.upgrade().unwrap();
      println!("Gadget {:?}", gadget);
  }

  // 在 main 函数的最后，gadget_owner，gadget1 和 gadget2 都被销毁。
  // 具体是，因为这几个结构体之间没有了强引用（`Rc<T>`），所以，当他们销毁的时候。
  // 首先 gadget2 和 gadget1 被销毁。
  // 然后因为 gadget_owner 的引用数量为 0，所以这个对象可以被销毁了。
  // 循环引用问题也就避免了
}

fn node_test(){
  let leaf = Rc::new(Node::new(3));

  println!(
      "leaf strong = {}, weak = {}",
      Rc::strong_count(&leaf),
      Rc::weak_count(&leaf),
  );

  {
    let branch = Rc::new(Node::new(3));
    branch.add_child(Rc::clone(&leaf));

    leaf.set_parent(Rc::downgrade(&branch));

    println!(
      "branch strong = {}, weak = {}",
      Rc::strong_count(&branch),
      Rc::weak_count(&branch),
    );

    println!(
      "leaf strong = {}, weak = {}",
      Rc::strong_count(&leaf),
      Rc::weak_count(&leaf),
    );
  }

  println!("leaf parent = {:?}", leaf.get_parent());
  println!(
      "leaf strong = {}, weak = {}",
      Rc::strong_count(&leaf),
      Rc::weak_count(&leaf),
  );
}

fn self_ref_test() {
  let mut t = SelfRef::new("hello");
  t.init();
  // 打印值和指针地址
  println!("{}, {:?}", t.value(), t.pointer_to_value());

  let unmovable: std::pin::Pin<Box<Unmovable>> = Unmovable::new("hello".to_string());
  println!("{:?}", unmovable);
}

#[allow(dead_code)]
fn get_default<'m, K, V>(map: &'m mut HashMap<K, V>, key: K) -> &'m mut V
where
    K: Clone + Eq + Hash,
    V: Default,
{
    // match map.get_mut(&key) {
    //     Some(value) => value,
    //     None => {
    //         map.insert(key.clone(), V::default());
    //         map.get_mut(&key).unwrap()
    //     }
    // }

    if let None = map.get_mut(&key) {
      map.insert(key.clone(), V::default());
    }

    map.get_mut(&key).unwrap()
}
