use std::{
    collections::HashMap,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use crate::context::Context;
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
        println!("Server running on {}", &self.addr);

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

    fn dispatch(ctx: &mut Context) {
        // 简单的路由
        let path = &ctx.req.path;

        let response = if path == "/" {
            Response::text(200, "OK".to_string(), "hello gua".to_string())
        } else if path == "/json" {
            let mut data = HashMap::new();
            data.insert("name", "bob");
            data.insert("age", "18");
            Response::json(200, "OK".to_string(), data)
        } else if path == "/html" {
            Response::html(200, "OK".to_string(), "<h1>hello gua</h1>")
        } else {
            Response::text(404, "Not Found".to_string(), "Not Found".to_string())
        };

        ctx.resp = response;
    }

    fn handle_connection(mut stream: TcpStream) -> Result<(), std::io::Error> {
        // TODO: readd all request data
        let mut buffer = [0; 2048];
        let bytes_read = stream.read(&mut buffer)?;

        if bytes_read == 0 {
            return Ok(());
        }

        let request = String::from_utf8_lossy(&buffer[..bytes_read]);
        let mut ctx = Context::from_string(&request);

        Self::dispatch(&mut ctx);

        stream.write_all(ctx.resp.make_response().as_bytes())?;
        stream.flush()?;
        Ok(())
    }
}
