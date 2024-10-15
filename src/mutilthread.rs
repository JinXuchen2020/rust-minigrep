use std::{sync::{Arc, Barrier, Condvar, Mutex}, thread, time::Duration};
use std::cell::RefCell;

use std::thread::LocalKey;
use std::sync::Once;

static mut VAL: usize = 0;
static INIT: Once = Once::new();

thread_local! {
    static FOO: RefCell<usize> = RefCell::new(0);
}
struct Bar {
    foo: &'static LocalKey<RefCell<usize>>,
}
impl Bar {
    fn constructor() -> Self {
        Self {
            foo: &FOO,
        }
    }
}


pub fn create_threads() {
  let v = vec![1, 2, 3];

  let handle = thread::spawn(move || {
    for i in 1..10 {
      println!("hi number {} from the spawned thread!", i);
      thread::sleep(Duration::from_millis(1));
      println!("Here's a vector: {:?}", v);
    }
  });
  
  for i in 1..5 {
    println!("hi number {} from the main thread!", i);
    thread::sleep(Duration::from_millis(1));
  }

  handle.join().unwrap();
}

pub fn loop_in_thread() {
  let new_thread = thread::spawn(move || {
    // 再创建一个线程B
    thread::spawn(move || {
      loop {
        println!("I am a new thread.");
      }
    })
  });

  // 等待新创建的线程执行完成
  new_thread.join().unwrap();
  println!("Child thread is finish!");

  // 睡眠一段时间，看子线程创建的子线程是否还在运行
  thread::sleep(Duration::from_millis(100));
}

pub fn barrier_in_thread() {
  let mut handles = Vec::with_capacity(6);
  let barrier = Arc::new(Barrier::new(6));

  for _ in 0..6 {
    let b = barrier.clone();
    handles.push(thread::spawn(move|| {
        println!("before wait");
        b.wait();
        println!("after wait");
    }));
  }

  for handle in handles {
      handle.join().unwrap();
  }
}

pub fn thread_local_in_thread() {
  thread_local!(static FOO: RefCell<u32> = RefCell::new(1));

  FOO.with(|f| {
      assert_eq!(*f.borrow(), 1);
      *f.borrow_mut() = 2;
  });

  // 每个线程开始时都会拿到线程局部变量的FOO的初始值
  let t = thread::spawn(move|| {
      FOO.with(|f| {
          assert_eq!(*f.borrow(), 1);
          *f.borrow_mut() = 3;
      });
  });

  // 等待线程完成
  t.join().unwrap();

  // 尽管子线程中修改为了3，我们在这里依然拥有main线程中的局部值：2
  FOO.with(|f| {
    assert_eq!(*f.borrow(), 2);
  });
}

pub fn mutex_in_thread() {
  let pair = Arc::new((Mutex::new(false), Condvar::new()));
  let pair2 = pair.clone();

  thread::spawn(move|| {
      let (lock, cvar) = &*pair2;
      let mut started = lock.lock().unwrap();
      println!("changing started");
      *started = true;
      cvar.notify_one();
  });

  let (lock, cvar) = &*pair;
  let mut started = lock.lock().unwrap();
  while !*started {
      started = cvar.wait(started).unwrap();
  }

  println!("started changed");
}

pub fn once_in_thread() {
  let handle1 = thread::spawn(move || {
    INIT.call_once(|| {
        unsafe {
            VAL = 1;
        }
    });
  });

  let handle2 = thread::spawn(move || {
      INIT.call_once(|| {
          unsafe {
              VAL = 2;
          }
      });
  });

  handle1.join().unwrap();
  handle2.join().unwrap();

  println!("{}", unsafe { VAL });
}