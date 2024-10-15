use std::{thread, time::{self, Duration}};

use std::pin::Pin;
use futures::{channel::mpsc, SinkExt, Stream, StreamExt, TryStreamExt};

use crate::timer_future::TimerFuture;

pub async fn async_base() {
  println!("go go go !");

  hello_cat().await;

  let future_sing = learn_sing_song();
  let future_dance = dance();

  futures::join!(future_sing, future_dance);

  blocks().await;

  send_recv().await;
}

async fn hello_cat() {
  println!("hello, kitty!");
}

struct Song {
  author: String,
  name: String,
}

async fn learn_song() -> Song {
  Song {
    author: "周杰伦".to_string(),
    name: String::from("《菊花台》"),
  }
}

async fn sing_song(song: Song) {
  println!(
    "给大家献上一首{}的{} ~ {}",
    song.author, song.name, "菊花残，满地伤~ ~"
  );

  TimerFuture::new(Duration::new(2, 0)).await;
}

async fn learn_sing_song() {  
  println!(
    "Start"
  );
  let song = learn_song().await;
  println!(
    "A song is learned: {} by {}",
    song.name, song.author
  );
  sing_song(song).await;
  
  println!("歌曲结束，大家欢度假~ ~");
}

async fn dance() {
  println!("唱到情深处，身体不由自主的动了起来~ ~");
}

async fn blocks() {
  let my_string = "foo".to_string();

  let future_one = async {
      // ...
      println!("{my_string}");
  };

  let future_two = async {
      // ...
      println!("{my_string}");
  };

  // 运行两个 Future 直到完成
  let ((), ()) = futures::join!(future_one, future_two);
}

async fn send_recv() {
  const BUFFER_SIZE: usize = 10;
  let (mut tx, mut rx) = mpsc::channel::<i32>(BUFFER_SIZE);

  tx.send(1).await.unwrap();
  tx.send(2).await.unwrap();
  drop(tx);

  // `StreamExt::next` 类似于 `Iterator::next`, 但是前者返回的不是值，而是一个 `Future<Output = Option<T>>`，
  // 因此还需要使用`.await`来获取具体的值
  println!("{:?}", rx.next().await);
  println!("{:?}", rx.next().await);
  println!("{:?}", rx.next().await);
}

async fn jump_around(
  mut stream: Pin<&mut dyn Stream<Item = Result<u8, std::io::Error>>>,
) -> Result<(), std::io::Error> {
  const MAX_CONCURRENT_JUMPERS: usize = 100;

  stream.try_for_each_concurrent(MAX_CONCURRENT_JUMPERS, |num| async move {
      jump_n_times(num).await?;
      report_n_jumps(num).await?;
      Ok(())
  }).await?;

  Ok(())
}

async fn jump_n_times(n: u8) -> Result<(), std::io::Error> {
  for _ in 0..n {
    println!("jumping");
    TimerFuture::new(Duration::new(1, 0)).await;
  }
  Ok(())
}

async fn report_n_jumps(n: u8) -> Result<(), std::io::Error> {
  println!("jumped {} times", n);
  Ok(())
}

