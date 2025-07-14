use std::{rc::Rc, sync::Arc};
use wgpu::{
    Adapter, CreateSurfaceError, Device, Surface, SurfaceCapabilities, SurfaceConfiguration, TextureFormat
};
use winit::window::Window;

use super::webgpuinstance::WebGPUInstance;

pub struct WebGPUSurface {
    pub surface: Surface<'static>,
    instance: Rc<WebGPUInstance>,
    window: Arc<Window>,
}

impl WebGPUSurface {
    pub fn new(instance: Rc<WebGPUInstance>, window: Arc<Window>) -> Self {
        tracing::info!("Creating WebGPU surface...");

        let surface = instance
            .instance
            .create_surface(window.clone())
            .expect("Failed to create surface!");

        Self {
            surface,
            instance,
            window,
        }
    }
}
