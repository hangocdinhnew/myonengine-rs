use std::sync::Arc;
use wgpu::{Instance, Surface};
use winit::window::Window;

pub struct WebGPUSurface {
    pub surface: Surface<'static>,
}

impl WebGPUSurface {
    pub fn new(instance: &Instance, window: Arc<Window>) -> Self {
        tracing::info!("Creating WebGPU surface...");

        let surface = instance
            .create_surface(window)
            .expect("Failed to create surface!");

        Self { surface }
    }
}
