mod webgpu;

use webgpu::WebGPUAPI;

#[derive(Default)]
pub struct GraphicsAPI {
    pub webgpu: WebGPUAPI
}

impl GraphicsAPI {
    pub fn new() -> Self {
        let webgpu = WebGPUAPI::new();

        Self {
            webgpu
        }
    }
}
