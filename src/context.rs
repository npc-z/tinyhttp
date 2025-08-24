use crate::{request::Request, response::Response};

pub struct Context {
    pub req: Request,
    pub resp: Response,
}

impl Context {
    pub fn from_string(request: &str) -> Self {
        Self {
            req: Request::from_string(request).unwrap(),
            resp: Response::default(),
        }
    }
}
