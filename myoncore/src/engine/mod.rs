use std::rc::Rc;
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::{ElementState, KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowAttributes},
};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use crate::logger::Logger;

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
            resizable
        }
    }
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self::new(String::from("My window"), 800, 600, true)
    }
}

pub struct Engine {
    config: Rc<EngineConfig>,
    logger: Rc<Logger>,
    window: Option<Rc<Window>>,
}

impl Engine {
    pub fn new(config: Rc<EngineConfig>) -> Self {
        let logger = Rc::new(Logger::new());

        Self {
            config,
            logger,
            window: None,
        }
    }
}

impl ApplicationHandler for Engine {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = WindowAttributes::default()
            .with_title(&self.config.title)
            .with_inner_size(LogicalSize::new(self.config.width, self.config.height))
            .with_resizable(self.config.resizable);

        let window = event_loop
            .create_window(window_attributes)
            .expect("Failed to create window");

        self.window = Some(Rc::new(window));
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                tracing::info!("Close requested.");
                event_loop.exit();
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(KeyCode::Escape),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => {
                tracing::info!("Escape pressed. Exiting.");
                event_loop.exit();
            }
            _ => {}
        }
    }
}
