use tinyhttp::engine::Engine;

fn main() -> std::io::Result<()> {
    let engine = Engine::default("localhost:8000");
    let _ = engine.run();
    Ok(())
}
