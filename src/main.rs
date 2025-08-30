use std::{collections::HashMap, sync::Arc};
use tinyhttp::context::Context;

use tinyhttp::{engine::Engine, response::Response};

fn main() -> std::io::Result<()> {
    let app = Engine::default("localhost:8000");

    app.get(
        "/",
        Arc::new(|ctx: &mut Context| {
            ctx.resp = Response::text(200, "OK".to_string(), "Hello World".to_string());
        }),
    );

    app.get("/json", Arc::new(foo));

    let _ = app.run();
    Ok(())
}

fn foo(ctx: &mut Context) {
    let mut data = HashMap::new();
    data.insert("name", "bob");
    data.insert("age", "18");
    let resp = Response::json(200, "OK".to_string(), data);

    ctx.resp = resp;
}
