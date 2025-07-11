use myoncore::{AppHandler, Engine, EngineConfig};
use std::rc::Rc;
use winit::{
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
};

struct Sandbox;

impl AppHandler for Sandbox {
    fn on_event(&mut self, event_loop: &ActiveEventLoop, event: &WindowEvent) {}

    fn on_update(&mut self) {}
}

fn main() -> anyhow::Result<()> {
    let event_loop = EventLoop::new()?;
    let engineconfig = Rc::new(EngineConfig::new(
        String::from("MyonSandbox"),
        800,
        600,
        false,
    ));
    let mut engine = Engine::new(Rc::clone(&engineconfig), Sandbox);
    event_loop.run_app(&mut engine)?;

    Ok(())
}
