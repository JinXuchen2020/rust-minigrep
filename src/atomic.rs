use std::hint;
use std::ops::Sub;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Instant;
use std::sync::OnceLock;

const N_TIMES: u64 = 10000000;
const N_THREADS: usize = 10;

static R: AtomicU64 = AtomicU64::new(0);

#[derive(Debug)]
struct MyBox(*const u8);
unsafe impl Send for MyBox {}
unsafe impl Sync for MyBox {}

fn add_n_times(n: u64) -> JoinHandle<()> {
  thread::spawn(move || {
      for _ in 0..n {
          R.fetch_add(1, Ordering::Relaxed);
      }
  })
}

pub fn atomic_base() {
  let s = Instant::now();
  let mut threads = Vec::with_capacity(N_THREADS);

  for _ in 0..N_THREADS {
      threads.push(add_n_times(N_TIMES));
  }

  for thread in threads {
      thread.join().unwrap();
  }

  assert_eq!(N_TIMES * N_THREADS as u64, R.load(Ordering::Relaxed));
  println!("{:?}",R.load(Ordering::Relaxed));
  println!("{:?}",Instant::now().sub(s));
}

pub fn atomic_with_arc() {
  let spinlock = Arc::new(AtomicUsize::new(1));

  let spinlock_clone = Arc::clone(&spinlock);
  let thread = thread::spawn(move|| {
      spinlock_clone.store(0, Ordering::SeqCst);
  });

  // 等待其它线程释放锁
  while spinlock.load(Ordering::SeqCst) != 0 {
      hint::spin_loop();
  }

  if let Err(panic) = thread.join() {
      println!("Thread had an error: {:?}", panic);
  }

  let b = &MyBox(5 as *const u8);
  let v = Arc::new(Mutex::new(b));
  let t = thread::spawn(move || {
      let _v1 =  v.lock().unwrap();
  });

  t.join().unwrap();

  println!("{:?}",  b.0);
}

pub fn atomic_with_once() {
  let handle = thread::spawn(|| {
    let logger = Logger::global();
    logger.log("thread message".to_string());
  });

  // 主线程调用
  let logger = Logger::global();
  logger.log("some message".to_string());

  let logger2 = Logger::global();
  logger2.log("other message".to_string());

  handle.join().unwrap();
}

#[derive(Debug)]
struct Logger;

// Rust 1.70版本以上
static LOGGER: OnceLock<Logger> = OnceLock::new();

impl Logger {
    fn global() -> &'static Logger {
        // 获取或初始化 Logger
        LOGGER.get_or_init(|| {
            println!("Logger is being created..."); // 初始化打印
            Logger
        })
    }

    fn log(&self, message: String) {
        println!("{}", message)
    }
}