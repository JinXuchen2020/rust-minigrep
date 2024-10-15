use std::{fs, io::{Read, Write}, net::TcpStream};
use std::net::TcpListener;

async fn handle_connection(mut stream: TcpStream) {
    // 从连接中顺序读取 1024 字节数据
  let mut buffer = [0; 1024];
  stream.read(&mut buffer).unwrap();

  let get = b"GET / HTTP/1.1\r\n";


  // 处理HTTP协议头，若不符合则返回404和对应的 `html` 文件
  let (status_line, filename) = if buffer.starts_with(get) {
      ("HTTP/1.1 200 OK\r\n\r\n", "poem.txt")
  } else {
      ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "poem.txt")
  };

  println!("{filename}");
  let contents = fs::read_to_string(filename).unwrap();

  // 将回复内容写入连接缓存中
  let response = format!("{status_line}{contents}");
  stream.write_all(response.as_bytes()).unwrap();
  // 使用 flush 将缓存中的内容发送到客户端
  stream.flush().unwrap();
}

//#[async_std::main]
pub async fn runner() {
  let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

  for stream in listener.incoming() {
    let stream = stream.unwrap();

    handle_connection(stream).await;
  }
}