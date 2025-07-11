mod webgpu;

use std::rc::Rc;
use webgpu::WebGPUAPI;

#[derive(Default)]
pub struct GraphicsAPI {
    pub webgpu: Rc<WebGPUAPI>
}

impl GraphicsAPI {
    pub fn new() -> Self {
        let webgpu = Rc::new(WebGPUAPI::new());

        Self {
            webgpu
        }
    }
}
