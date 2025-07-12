mod webgpu;

use std::sync::Arc;
use webgpu::WebGPUAPI;
use winit::window::Window;

use crate::window::WindowSystem;

pub struct GraphicsAPI {
    pub webgpu: WebGPUAPI,
    window: Arc<WindowSystem>,
}

impl GraphicsAPI {
    pub fn new(window: Arc<WindowSystem>) -> Self {
        let webgpu = WebGPUAPI::new(window.window.clone());

        Self { webgpu, window }
    }
}
