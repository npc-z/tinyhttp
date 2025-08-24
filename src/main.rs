use std::{
    collections::HashMap,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    str::FromStr,
};

#[derive(Debug)]
enum HttpMethod {
    GET,
    POST,
    // 添加其他方法...
}

impl FromStr for HttpMethod {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "GET" => Ok(HttpMethod::GET),
            "POST" => Ok(HttpMethod::POST),
            _ => Err(format!("Unsupported HTTP method: {s}")),
        }
    }
}

// #[derive(Debug)]
struct Request {
    pub method: HttpMethod,
    pub path: String,
    pub args: HashMap<String, String>,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl Request {
    fn from_string(request: &str) -> Result<Self, String> {
        let mut lines = request.lines();

        // 解析起始行
        let start_line = lines.next().ok_or("Empty request".to_string())?;
        let mut start_line_parts = start_line.split_whitespace();

        let method_str = start_line_parts
            .next()
            .ok_or("No method in request".to_string())?;
        let method = HttpMethod::from_str(method_str)?;

        let mut path_part = start_line_parts.next().ok_or("Not found path")?.split("?");
        let path = path_part
            .next()
            .ok_or("Not found path".to_string())?
            .to_string();

        // 查询参数
        let mut args = HashMap::new();
        for kv in path_part {
            if let Some((key, value)) = kv.split_once("=") {
                args.insert(key.to_string(), value.to_string());
            }
        }

        // 解析头部和body
        let mut headers = HashMap::new();
        let mut body = String::new();
        let mut in_body = false;

        for line in lines {
            if line.is_empty() {
                in_body = true;
                continue;
            }

            if in_body {
                body.push_str(line);
                body.push('\n');
            } else if let Some((key, value)) = line.split_once(": ") {
                headers.insert(key.to_string(), value.to_string());
            }
        }

        Ok(Self {
            method,
            path,
            args,
            headers,
            body: body.trim().to_string(),
        })
    }
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
                // "Received {:?} request for {}\nargs: {:?}\nheaders {:?}\nbody: {}",
                // request.method, request.path, request.args, request.headers, request.body
                "Received {:?} request for {}\nargs: {:?}\nbody: {}",
                request.method, request.path, request.args, request.body
            );

            // 简单的路由
            let response = if request.path == "/" {
                "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 11\r\n\r\nHello World"
            } else {
                "HTTP/1.1 404 Not Found\r\nContent-Type: text/plain\r\nContent-Length: 9\r\n\r\nNot Found"
            };

            stream.write_all(response.as_bytes())?;
        }
        Err(e) => {
            eprintln!("Failed to parse request: {e}");
            let response =
                "HTTP/1.1 400 Bad Request\r\nContent-Type: text/plain\r\nContent-Length: 0\r\n\r\n";
            stream.write_all(response.as_bytes())?;
        }
    }

    stream.flush()?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000")?;
    println!("Server running on http://127.0.0.1:8000");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(|| {
                    if let Err(e) = handle_connection(stream) {
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
