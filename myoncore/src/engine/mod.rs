use std::rc::Rc;
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowAttributes},
};

use crate::logger::Logger;

#[derive(Default)]
pub struct EngineConfig {
    title: String,
    width: u16,
    height: u16,
    resizable: bool,
}

impl EngineConfig {
    pub fn new(title: String, width: u16, height: u16, resizable: bool) -> Self {
        Self {
            title,
            width,
            height,
            resizable,
        }
    }
}

pub trait AppHandler {
    fn on_event(&mut self, event_loop: &ActiveEventLoop, event: &WindowEvent);
    fn on_update(&mut self);
}

#[derive(Default)]
pub struct Engine<A: AppHandler> {
    config: Rc<EngineConfig>,
    logger: Rc<Logger>,
    window: Option<Rc<Window>>,
    app: A,
}

impl<A: AppHandler> Engine<A> {
    pub fn new(config: Rc<EngineConfig>, app: A) -> Self {
        let logger = Rc::new(Logger::new());

        Self {
            config,
            logger,
            window: None,
            app,
        }
    }
}

impl<A: AppHandler> ApplicationHandler for Engine<A> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = WindowAttributes::default()
            .with_title(&self.config.title)
            .with_inner_size(LogicalSize::new(self.config.width, self.config.height))
            .with_resizable(self.config.resizable);

        let window = event_loop
            .create_window(window_attributes)
            .expect("Failed to create window");

        self.window = Some(Rc::new(window));

        self.app.on_update();

        tracing::info!("Window created!");
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        self.app.on_event(event_loop, &event);
    }
}
