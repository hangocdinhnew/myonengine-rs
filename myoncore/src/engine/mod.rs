use std::{rc::Rc, sync::Arc};
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowAttributes},
};

use crate::{graphics::GraphicsAPI, logger::Logger, window::WindowSystem};

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
    windowsys: Option<Rc<WindowSystem>>,
    graphicsapi: Option<Rc<GraphicsAPI>>,
    app: A,
}

impl<A: AppHandler> Engine<A> {
    pub fn new(config: Rc<EngineConfig>, app: A) -> Self {
        let logger = Rc::new(Logger::new());

        Self {
            config,
            logger,
            windowsys: None,
            graphicsapi: None,
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

        let windowsys = Rc::new(WindowSystem::new(window_attributes, event_loop));

        tracing::info!("Window created!");

        let graphicsapi = Rc::new(GraphicsAPI::new(windowsys.clone()));

        self.windowsys = Some(windowsys);
        self.graphicsapi = Some(graphicsapi);

        self.app.on_update();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                tracing::info!("Closing...");
                event_loop.exit();
            }
            _ => {}
        }

        self.app.on_event(event_loop, &event);
    }
}
