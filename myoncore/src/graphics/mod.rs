mod webgpu;

use std::sync::Arc;
use webgpu::WebGPUAPI;
use winit::window::Window;

pub struct GraphicsAPI {
    pub webgpu: WebGPUAPI,
    window: Arc<Window>,
}

impl GraphicsAPI {
    pub fn new(window: Arc<Window>) -> Self {
        let webgpu = WebGPUAPI::new(window.clone());

        Self { webgpu, window }
    }
}
