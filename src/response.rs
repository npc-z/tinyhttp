use std::collections::HashMap;

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
