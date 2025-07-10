use myoncore::{AppHandler, Engine, EngineConfig};
use std::rc::Rc;
use winit::{
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
};

struct App;

impl AppHandler for App {
    fn on_event(&mut self, event_loop: &ActiveEventLoop, event: &WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                tracing::info!("Closing...");
                event_loop.exit();
            },
            _ => {}
        }
    }

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
    let mut engine = Engine::new(Rc::clone(&engineconfig), App);
    event_loop.run_app(&mut engine)?;

    Ok(())
}
