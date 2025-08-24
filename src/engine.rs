use std::{
    collections::HashMap,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use crate::request::Request;
use crate::response::Response;

pub struct Engine {
    addr: String,
}

impl Engine {
    pub fn default(addr: &str) -> Self {
        Engine {
            addr: addr.to_string(),
        }
    }

    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(&self.addr)?;
        println!("Server running on http://127.0.0.1:8000");

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    std::thread::spawn(|| {
                        if let Err(e) = Self::handle_connection(stream) {
                            eprintln!("Error handling connection: {e}");
                        }
                    });
                }
                Err(e) => {
                    eprintln!("Connection failed: {e}");
                }
            }
        }
        Ok(())
    }

    fn handle_connection(mut stream: TcpStream) -> Result<(), std::io::Error> {
        let mut buffer = [0; 2048];
        let bytes_read = stream.read(&mut buffer)?;

        if bytes_read == 0 {
            return Ok(());
        }

        let request_str = String::from_utf8_lossy(&buffer[..bytes_read]);

        match Request::from_string(&request_str) {
            Ok(request) => {
                println!(
                    "Received {:?} request for {}\nargs: {:?}\nbody: {}",
                    request.method, request.path, request.args, request.body
                );

                // 简单的路由
                let response = if request.path == "/" {
                    Response::text(200, "OK".to_string(), "hello gua".to_string())
                } else if request.path == "/json" {
                    let mut data = HashMap::new();
                    data.insert("name", "bob");
                    data.insert("age", "18");
                    Response::json(200, "OK".to_string(), data)
                } else if request.path == "/html" {
                    Response::html(200, "OK".to_string(), "<h1>hello gua</h1>")
                } else {
                    Response::text(404, "Not Found".to_string(), "Not Found".to_string())
                };

                stream.write_all(response.make_response().as_bytes())?;
            }
            Err(e) => {
                eprintln!("Failed to parse request: {e}");
                let response = Response::text(400, "Bad Request".to_string(), "Bad Request".to_string());
                stream.write_all(response.make_response().as_bytes())?;
            }
        }

        stream.flush()?;
        Ok(())
    }
}
