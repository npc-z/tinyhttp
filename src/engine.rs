use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, RwLock},
};

use crate::blueprint::{Blueprint, HttpHandler};
use crate::context::Context;
use crate::response::Response;

#[derive(Debug)]
pub struct Engine {
    addr: String,
    bp: Arc<RwLock<Blueprint>>,
}

impl Engine {
    pub fn default(addr: &str) -> Self {
        Engine {
            addr: addr.to_string(),
            // bp: Arc::new(Blueprint::new("root", "/")),
            bp: Arc::new(RwLock::new(Blueprint::new("root", "/"))),
        }
    }

    pub fn post(&self, path: &str, handler: HttpHandler) {
        // 需要获取可变引用或使用内部可变性
        let mut bp = self.bp.write().unwrap(); // 获取写锁
        bp.post(path, handler);
    }

    pub fn get(&self, path: &str, handler: HttpHandler) {
        let mut bp = self.bp.write().unwrap(); // 获取写锁
        bp.get(path, handler);
    }

    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(&self.addr)?;
        println!("Server running on {}", &self.addr);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let bp = Arc::clone(&self.bp);

                    std::thread::spawn(move || {
                        // 在线程内部获取读锁
                        let bp_read = bp.read().unwrap();

                        if let Err(e) = Self::handle_connection(&*bp_read, stream) {
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

    fn dispatch(bp: &Blueprint, ctx: &mut Context) {
        let path = &ctx.req.path;
        let method = &ctx.req.method;
        println!("======= get {method} req for ({path})");

        // 尝试使用路由系统查找处理器
        if let Some(handler) = bp.find_handler(method, path) {
            println!("Find handler for {path}");

            handler(ctx);
            return;
        }

        println!("Not find handler for {path}");
        let response = Response::text(404, "Not Found".to_string(), "Not Found".to_string());
        ctx.resp = response;
    }

    fn handle_connection(bp: &Blueprint, mut stream: TcpStream) -> Result<(), std::io::Error> {
        // TODO: readd all request data
        let mut buffer = [0; 2048];
        let bytes_read = stream.read(&mut buffer)?;

        if bytes_read == 0 {
            return Ok(());
        }

        let request = String::from_utf8_lossy(&buffer[..bytes_read]);
        let mut ctx = Context::from_string(&request);

        Self::dispatch(bp, &mut ctx);

        stream.write_all(ctx.resp.make_response().as_bytes())?;
        stream.flush()?;
        Ok(())
    }
}
