use std::{collections::HashMap, fmt::Display, str::FromStr};

#[derive(Debug)]
pub enum HttpMethod {
    GET,
    POST,
    // 添加其他方法...
}

impl Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GET => write!(f, "GET"),
            Self::POST => write!(f, "POST"),
        }
    }
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

#[derive(Debug)]
pub struct Request {
    pub method: HttpMethod,
    pub path: String,
    pub args: HashMap<String, String>,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl Request {
    pub fn from_string(request: &str) -> Result<Self, String> {
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
