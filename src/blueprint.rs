use crate::context::Context;
use crate::request::HttpMethod;
use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, RwLock};

pub type HttpHandler = Arc<dyn Fn(&mut Context) + Send + Sync + 'static>;

pub struct Blueprint {
    name: String,
    prefix: String,
    // 使用 HashMap 存储不同方法的路由表
    routes: RwLock<HashMap<HttpMethod, HashMap<String, HttpHandler>>>,
}

impl fmt::Debug for Blueprint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let routes = self.routes.read().unwrap();

        // 创建一个更结构化的输出
        let mut debug_map = f.debug_map();
        debug_map.entry(&"name", &self.name);
        debug_map.entry(&"prefix", &self.prefix);

        // 为每个方法添加路由信息
        for method in HttpMethod::all() {
            if let Some(handlers) = routes.get(&method) {
                let path_count = handlers.len();
                let paths = handlers.keys().map(|s| s.as_str()).collect::<Vec<_>>().join(", ");
                debug_map.entry(&format!("{method}"), &format!("{path_count} routes: {paths}"));
            } else {
                debug_map.entry(&format!("{method}"), &"0 routes");
            }
        }

        debug_map.finish()
    }
}

impl Blueprint {
    pub fn new(name: &str, prefix: &str) -> Self {
        if !prefix.starts_with("/") {
            panic!("prefix should start with '/'")
        }

        let mut routes = HashMap::new();
        for method in HttpMethod::all() {
            routes.insert(method, HashMap::new());
        }

        Self {
            name: name.to_string(),
            prefix: prefix.to_string(),
            routes: RwLock::new(routes),
        }
    }

    /// 注册子路由组.
    pub fn register_blueprint(&mut self, blueprint: &Self) {
        for method in HttpMethod::all() {
            if let Some(method_routes) = blueprint.routes.read().unwrap().get(&method) {
                for (path, handler) in method_routes.iter() {
                    self.add_handler(method, path, handler.clone());
                }
            } else {
                panic!("cannot find {method} routes");
            }
        }
    }

    pub fn get(&mut self, path: &str, handler: HttpHandler) {
        self.add_handler(HttpMethod::GET, path, handler);
    }

    pub fn post(&mut self, path: &str, handler: HttpHandler) {
        self.add_handler(HttpMethod::POST, path, handler);
    }

    fn jon_http_path(&self, p1: &str, p2: &str) -> String {
        let pattern = format!("{p1}{p2}");
        pattern.replacen("//", "/", 99)
    }

    fn add_handler(&self, method: HttpMethod, path: &str, handler: HttpHandler) {
        let mut routes = self.routes.write().unwrap();

        if let Some(method_routes) = routes.get_mut(&method) {
            let mut pattern = path.to_string();
            if !pattern.starts_with("/") {
                pattern = format!("/{pattern}");
            }
            pattern = self.jon_http_path(&self.prefix, &pattern);

            method_routes.insert(pattern, handler);
        } else {
            panic!("cannot find {method} routes");
        }
    }

    pub fn find_handler(&self, method: HttpMethod, path: &str) -> Option<HttpHandler> {
        let routes = self.routes.read().unwrap();
        if let Some(method_routes) = routes.get(&method) {
            method_routes.get(path).map(Arc::clone)
        } else {
            println!("cannot find {method} for ({path})");
            None
        }
    }
}
