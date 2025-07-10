use myoncore::{Engine, EngineConfig};
use std::rc::Rc;
use winit::event_loop::EventLoop;

fn main() -> anyhow::Result<()> {
    let event_loop = EventLoop::new()?;
    let engineconfig = Rc::new(EngineConfig::new(
        String::from("MyonSandbox"),
        800,
        600,
        false,
    ));
    let mut engine = Engine::new(Rc::clone(&engineconfig));
    event_loop.run_app(&mut engine)?;

    Ok(())
}
