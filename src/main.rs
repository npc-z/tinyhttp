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
        let path = path_part.next().ok_or("Not found path".to_string())?.to_string();

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

#[derive(Debug)]
enum ContentType {
    HTML,
    JSON,
    TEXT,
}

impl ToString for ContentType {
    fn to_string(&self) -> String {
        match self {
            Self::HTML => "text/html".to_string(),
            Self::TEXT => "text/plain".to_string(),
            Self::JSON => "application/json".to_string(),
        }
    }
}

#[derive(Debug)]
struct Response {
    status_code: u16,
    status: String,
    content_type: ContentType,
    body: String,
}

impl Response {
    pub fn html(status_code: u16, status: String, html: &str) -> Self {
        Self {
            status_code,
            status,
            content_type: ContentType::HTML,
            body: html.to_string(),
        }
    }

    pub fn json(status_code: u16, status: String, data: HashMap<&str, &str>) -> Self {
        let body = serde_json::to_string(&data).unwrap();
        Self {
            status_code,
            status,
            content_type: ContentType::JSON,
            body,
        }
    }

    pub fn text(status_code: u16, status: String, text: String) -> Self {
        Self {
            status_code,
            status,
            content_type: ContentType::TEXT,
            body: text,
        }
    }

    pub fn make_response(&self) -> String {
        // "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 11\r\n\r\nHello World"
        format!(
            "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
            self.status_code,
            self.status,
            self.content_type.to_string(),
            self.body.len(),
            self.body,
        )
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
