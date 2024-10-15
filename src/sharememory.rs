use std::{sync::{Arc, Condvar, Mutex, MutexGuard}, thread::{self, sleep}, time::Duration};

use lazy_static::lazy_static;

pub fn share_memory_base() {
  let m = Mutex::new(5);
  {
      // 获取锁，然后deref为`m`的引用
      // lock返回的是Result
      let mut num = m.lock().unwrap();
      *num = 6;
      // 锁自动被drop
  }

  println!("m = {:?}", m);
}

pub fn share_memory_with_thread() {
  let counter = Arc::new(Mutex::new(0));
  let mut handles = vec![];

  for _ in 0..10 {
      let counter = Arc::clone(&counter);
      // 创建子线程，并将`Mutex`的所有权拷贝传入到子线程中
      let handle = thread::spawn(move || {
          let mut num = counter.lock().unwrap();

          *num += 1;
      });
      handles.push(handle);
  }

  // 等待所有子线程完成
  for handle in handles {
      handle.join().unwrap();
  }

  // 输出最终的计数结果
  println!("Result: {}", *counter.lock().unwrap());
}
lazy_static! {
  static ref MUTEX1: Mutex<i64> = Mutex::new(0);
  static ref MUTEX2: Mutex<i64> = Mutex::new(0);
}

pub fn share_memory_with_deadlock() {
  // 存放子线程的句柄
  let mut children = vec![];
  for i_thread in 0..2 {
      children.push(thread::spawn(move || {
          for _ in 0..1 {
              // 线程1
              if i_thread % 2 == 0 {
                  // 锁住MUTEX1
                  let _guard = MUTEX1.lock().unwrap();

                  println!("线程 {} 锁住了MUTEX1，接着准备去锁MUTEX2 !", i_thread);

                  // 当前线程睡眠一小会儿，等待线程2锁住MUTEX2
                  sleep(Duration::from_millis(10));

                  // 去锁MUTEX2
                  let guard = MUTEX2.try_lock();
                  println!("线程 {} 获取 MUTEX2 锁的结果: {:?}", i_thread, guard);
              // 线程2
              } else {
                  // 锁住MUTEX2
                  let _guard = MUTEX2.lock().unwrap();

                  println!("线程 {} 锁住了MUTEX2, 准备去锁MUTEX1", i_thread);

                  // 当前线程睡眠一小会儿，等待线程2锁住MUTEX2
                  sleep(Duration::from_millis(10));

                  let guard = MUTEX1.try_lock();
                  println!("线程 {} 获取 MUTEX1 锁的结果: {:?}", i_thread, guard);
              }
          }
      }));
  }

  // 等子线程完成
  for child in children {
      let _ = child.join();
  }

  println!("死锁没有发生");
}

pub fn share_memory_with_condvar() {
  let flag = Arc::new(Mutex::new(false));
  let cond = Arc::new(Condvar::new());
  let cflag = flag.clone();
  let ccond = cond.clone();

  let hdl = thread::spawn(move || {
      let mut lock = cflag.lock().unwrap();
      let mut counter = 0;

      while counter < 3 {
          while !*lock {
              // wait方法会接收一个MutexGuard<'a, T>，且它会自动地暂时释放这个锁，使其他线程可以拿到锁并进行数据更新。
              // 同时当前线程在此处会被阻塞，直到被其他地方notify后，它会将原本的MutexGuard<'a, T>还给我们，即重新获取到了锁，同时唤醒了此线程。
              lock = ccond.wait(lock).unwrap();
          }
          
          *lock = false;

          counter += 1;
          println!("inner counter: {}", counter);
      }
  });

  let mut counter = 0;
  loop {
      sleep(Duration::from_millis(1000));
      *flag.lock().unwrap() = true;
      counter += 1;
      if counter > 3 {
          break;
      }
      println!("outside counter: {}", counter);
      cond.notify_one();
  }
  hdl.join().unwrap();
  println!("{:?}", flag);
}