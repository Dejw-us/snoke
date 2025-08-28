use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use snake_http::{Request, Response, StatusCode};

#[tokio::main]
async fn main() {
  let listener = TcpListener::bind("localhost:8080").await.unwrap();
  
  loop {
    let (mut stream, _) = listener.accept().await.unwrap();
    tokio::spawn(async move {
      let mut buf = [0u8; 1024];
      let n = stream.read(&mut buf).await.unwrap();
      if n > 0 {
        println!("Received: {:?}", Request::from_bytes(&buf[..n]).unwrap());

        let response = Response::new(StatusCode::OK, "OK", "fdsf");

        stream.write_all(response.to_string().as_bytes()).await.unwrap();
      }
    });
  }
}
