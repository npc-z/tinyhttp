use crate::context::Context;
use crate::request::HttpMethod;
use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, RwLock};

pub type HttpHandler = Arc<dyn Fn(&mut Context) + Send + Sync + 'static>;

pub struct Blueprint {
    name: String,
    prefix: String,
    get_routes: RwLock<HashMap<String, HttpHandler>>,
    post_routes: RwLock<HashMap<String, HttpHandler>>,
}

impl fmt::Debug for Blueprint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let get_routes_keys: Vec<String> = self.get_routes.read().unwrap().keys().cloned().collect();
        let post_routes_keys: Vec<String> = self.post_routes.read().unwrap().keys().cloned().collect();
        f.debug_struct("Blueprint")
            .field("name", &self.name)
            .field("prefix", &self.prefix)
            .field("get_routes", &get_routes_keys)
            .field("post_routes", &post_routes_keys)
            .finish()
    }
}

impl Blueprint {
    pub fn new(name: &str, prefix: &str) -> Self {
        if !prefix.starts_with("/") {
            panic!("prefix should start with '/'")
        }

        Self {
            name: name.to_string(),
            prefix: prefix.to_string(),
            get_routes: RwLock::new(HashMap::new()),
            post_routes: RwLock::new(HashMap::new()),
        }
    }

    fn add_handler(&self, method: HttpMethod, path: &str, handler: HttpHandler) {
        let mut pattern = path.to_string();

        if !pattern.starts_with("/") {
            pattern = format!("/{pattern}");
        }

        if self.prefix != "/" {
            pattern = format!("{}{pattern}", self.prefix);
        }

        match method {
            HttpMethod::GET => {
                let mut r = self.get_routes.write().unwrap();
                r.insert(pattern, handler);
            }
            HttpMethod::POST => {
                let mut r = self.post_routes.write().unwrap();
                r.insert(pattern, handler);
            }
        };
    }

    pub fn get(&mut self, path: &str, handler: HttpHandler) {
        self.add_handler(HttpMethod::GET, path, handler);
    }

    pub fn post(&mut self, path: &str, handler: HttpHandler) {
        self.add_handler(HttpMethod::POST, path, handler);
    }

    pub fn find_handler(&self, method: &HttpMethod, path: &str) -> Option<HttpHandler> {
        match method {
            HttpMethod::GET => {
                let routes = self.get_routes.read().unwrap();
                routes.get(path).map(Arc::clone)
            }
            HttpMethod::POST => {
                let routes = self.get_routes.read().unwrap();
                routes.get(path).map(Arc::clone)
            }
        }
    }
}
