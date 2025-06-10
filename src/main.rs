use request::HttpRequest;
use routes::handle_request;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

mod http;
mod models;
mod request;
mod response;
mod routes;
mod views;

#[tokio::main]
async fn main() {
    let _ = dotenv::dotenv().unwrap();

    let server_addr = dotenv::var("SERVER_ADDR").unwrap();
    let port = dotenv::var("PORT").unwrap();

    let listener = TcpListener::bind(format!("{}:{}", server_addr, port))
        .await
        .unwrap();

    println!("Server listening on {}:{}", server_addr, port);

    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            let mut buffer = [0; 2048];

            match socket.read(&mut buffer).await {
                Ok(0) => {
                    println!("Connection closed.");
                }
                Ok(n) => {
                    let request =
                        HttpRequest::parse(String::from_utf8_lossy(&buffer[..n]).to_string());

                    if let Ok(req) = request {
                        let log = format!("{} {} {}", req.version(), req.method(), req.uri_raw());
                        let response = handle_request(req);
                        println!("{} {} from {}", log, response.status(), addr);
                        let _ = socket.write_all(format!("{}", response).as_bytes()).await;
                    } else {
                        let _ = socket.write_all(String::from(
                            "HTTP/1.1 400 Bad Request\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
                        ).as_bytes()).await;
                    }
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        });
    }
}
