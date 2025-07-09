use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::{ElementState, KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowAttributes},
};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use crate::logger::Logger;

pub struct Engine {
    logger: Arc<Logger>,
    window: Option<Arc<Window>>,
}

impl Engine {
    pub fn new() -> Self {
        let logger = Arc::new(Logger::new());

        Self {
            logger,
            window: None,
        }
    }

    fn render(&self) {
        if let Some(window) = &self.window {
        }
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}

impl ApplicationHandler for Engine {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = WindowAttributes::default()
            .with_title("Engine Window")
            .with_resizable(true);

        let window = event_loop
            .create_window(window_attributes)
            .expect("Failed to create window");

        self.window = Some(Arc::new(window));
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
