use tinyhttp::engine::Engine;

fn main() -> std::io::Result<()> {
    let app = Engine::default("localhost:8000");
    let _ = app.run();
    Ok(())
}
