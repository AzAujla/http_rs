#![allow(dead_code)]

pub mod database;
pub mod http;
pub mod request;
pub mod response;
pub mod router;

use std::str::FromStr;
use std::sync::Arc;

use http::version::HttpVersion;
use request::Request;
use router::Router;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

pub struct App {
    address: &'static str,
    port: u32,
    router: Arc<Router>,
    version: HttpVersion,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LogLevel {
    All,
    Severe,
    ErrorsOnly,
    None,
}

impl App {
    pub fn new(address: &'static str, port: u32, router: Router) -> Self {
        Self {
            address,
            port,
            router: Arc::new(router),
            version: HttpVersion::HTTP_1_1,
        }
    }

    pub async fn listen(&mut self, log_level: bool) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(format!("{}:{}", self.address, self.port)).await?;

        if log_level {
            println!("Listening on {}:{}", self.address, self.port);
        }

        let router = &self.router;
        loop {
            let (mut socket, addr) = listener.accept().await?;
            if log_level {
                println!("From: {}", addr);
            }
            let router_clone = Arc::clone(router);
            let version = self.version;

            tokio::spawn(async move {
                let mut buf = [0u8; 1024];

                loop {
                    let n = match socket.read(&mut buf).await {
                        Ok(0) => break,
                        Ok(n) => n,
                        Err(e) => {
                            if log_level {
                                eprintln!("Read error: {}", e);
                            }
                            break;
                        }
                    };

                    let request =
                        Request::from_str(std::str::from_utf8(&buf[0..n]).unwrap()).unwrap();
                    let response = router_clone.handle(request).await;

                    match socket
                        .write_all(format!("{} {}", version, response).as_bytes())
                        .await
                    {
                        Ok(_) => {}
                        Err(e) => {
                            if log_level {
                                println!("{}", e);
                            }
                        }
                    };
                }
            });
        }
    }

    pub fn version(&mut self, version: HttpVersion) -> &mut Self {
        self.version = version;
        self
    }
}
