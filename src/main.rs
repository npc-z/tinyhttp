use std::{collections::HashMap, sync::Arc};
use tinyhttp::blueprint::Blueprint;
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

    app.post("/json", Arc::new(foo));

    let mut user_bp = Blueprint::new("user_bp", "/user");
    user_bp.post(
        "/add",
        Arc::new(|ctx: &mut Context| {
            ctx.resp = Response::text(200, "OK".to_string(), "added a new user".to_string());
        }),
    );

    let mut user_post_bp = Blueprint::new("user_post_bp", "/post");
    user_post_bp.get(
        "/list",
        Arc::new(|ctx: &mut Context| {
            ctx.resp = Response::text(200, "OK".to_string(), "list user's posts".to_string());
        }),
    );
    user_post_bp.post(
        "/new",
        Arc::new(|ctx: &mut Context| {
            ctx.resp = Response::text(200, "OK".to_string(), "post a new post".to_string());
        }),
    );
    // dbg!(&user_post_bp);
    user_bp.register_blueprint(&user_post_bp);
    // dbg!(&user_bp);
    app.register_blueprint(&user_bp);

    dbg!(&app);

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
