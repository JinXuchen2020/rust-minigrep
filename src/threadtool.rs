use std::{sync::{mpsc, Arc, Mutex}, thread};

type Job = Box<dyn FnOnce() + Send +'static>;

pub struct ThreadPool {
  workers: Vec<Worker>,
  sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
  // --snip--
  pub fn new(size: usize) -> ThreadPool {
    assert!(size > 0);

    let (sender, receiver) = mpsc::channel();

    let mut workers = Vec::with_capacity(size);

    let receiver = Arc::new(Mutex::new(receiver));

    for id in 0..size {
      workers.push(Worker::new(id, receiver.clone()));
    }

    ThreadPool { workers, sender:Some(sender) }
  }

  pub fn execute<F>(&self, f: F)
    where
      F: FnOnce() + Send + 'static,
  {
    let job = Box::new(f);

    self.sender.as_ref().unwrap().send(job).unwrap();
  }
  // --snip--
}

impl Drop for ThreadPool {
  fn drop(&mut self) {
    drop(self.sender.take());
    for worker in &mut self.workers {
      println!("Shutting down worker {}", worker.id);

      if let Some(thread) = worker.thread.take() {
        thread.join().unwrap();
      }
    }
  }
}

struct Worker {
  id: usize,
  thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
  fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker 
  {
    let thread = thread::spawn(move || loop {
      let message = receiver.lock().unwrap().recv();

      match message {
        Ok(job) => {
          println!("Worker {id} got a job; executing.");

          job();
        }
        Err(_) => {
          println!("Worker {id} shutting down");
          break;
        }
      }
    });

    // 每个 `Worker` 都拥有自己的唯一 id
    Worker { id, thread: Some(thread) }
  }
}