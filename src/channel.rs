use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn channel_base() {
  // 创建一个消息通道, 返回一个元组：(发送者，接收者)
  let (tx, rx) = mpsc::channel();

  // 创建线程，并发送消息
  thread::spawn(move || {
      // 发送一个数字1, send方法返回Result<T,E>，通过unwrap进行快速错误处理
      tx.send(1).unwrap();

      // 下面代码将报错，因为编译器自动推导出通道传递的值是i32类型，那么Option<i32>类型将产生不匹配错误
      // tx.send(Some(1)).unwrap()
  });


  // 在主线程中接收子线程发送的消息并输出
  println!("receive {:?}", rx.try_recv());
  println!("receive {:?}", rx.try_recv());
  println!("receive {:?}", rx.try_recv());
}

pub fn channel_send_string(){
  let (tx, rx) = mpsc::channel();

  // 创建线程，并发送消息
  thread::spawn(move || {
      // 发送一个数字1, send方法返回Result<T,E>，通过unwrap进行快速错误处理
      let s = String::from("hello");
      tx.send(s).unwrap();

      //println!("send string {}", s);
      // 下面代码将报错，因为编译器自动推导出通道传递的值是i32类型，那么Option<i32>类型将产生不匹配错误
      // tx.send(Some(1)).unwrap()
  });


  // 在主线程中接收子线程发送的消息并输出
  let recieved_string = rx.recv().unwrap();
  println!("receive string {}", recieved_string);
}

pub fn channel_send_multiple_values() {
  let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

pub fn channel_mul_sender(){
  let (tx, rx) = mpsc::channel();
  let tx1 = tx.clone();
  thread::spawn(move || {
      tx.send(String::from("hi from raw tx")).unwrap();
  });

  thread::spawn(move || {
      tx1.send(String::from("hi from cloned tx")).unwrap();
  });

  for received in rx {
      println!("Got: {}", received);
  }
}

pub fn channel_sync_send(){  
  let (tx, rx)= mpsc::sync_channel(1);

  let handle = thread::spawn(move || {
      println!("发送之前");
      tx.send(1).unwrap();
      println!("发送之后");
      tx.send(1).unwrap();
      println!("再次发送之后");
  });

  println!("睡眠之前");
  thread::sleep(Duration::from_secs(3));
  println!("睡眠之后");

  println!("receive {}", rx.recv().unwrap());
  handle.join().unwrap();
}