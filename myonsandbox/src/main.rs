use myoncore::Engine;
use winit::event_loop::EventLoop;

fn main() -> anyhow::Result<()> {
    let event_loop = EventLoop::new()?;
    let mut engine = Engine::new();
    event_loop.run_app(&mut engine)?;

    Ok(())
}
