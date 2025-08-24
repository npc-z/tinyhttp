use std::{collections::HashMap, fmt::Display};

#[derive(Debug)]
enum ContentType {
    Html,
    Json,
    Text,
}

impl Display for ContentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let charset = "charset=utf-8";
        match self {
            Self::Html => write!(f, "text/html; {charset}"),
            Self::Text => write!(f, "text/plain; {charset}"),
            Self::Json => write!(f, "application/json; {charset}"),
        }
    }
}

#[derive(Debug)]
pub struct Response {
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
            content_type: ContentType::Html,
            body: html.to_string(),
        }
    }

    pub fn json(status_code: u16, status: String, data: HashMap<&str, &str>) -> Self {
        let body = serde_json::to_string(&data).unwrap();
        Self {
            status_code,
            status,
            content_type: ContentType::Json,
            body,
        }
    }

    pub fn text(status_code: u16, status: String, text: String) -> Self {
        Self {
            status_code,
            status,
            content_type: ContentType::Text,
            body: text,
        }
    }

    pub fn make_response(&self) -> String {
        // "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 11\r\n\r\nHello World"
        format!(
            "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
            self.status_code,
            self.status,
            self.content_type,
            self.body.len(),
            self.body,
        )
    }
}
