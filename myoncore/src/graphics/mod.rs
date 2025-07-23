mod webgpu;

use std::sync::Arc;

pub use webgpu::WebGPUAPI;
use winit::window::Window;

use crate::rhi::Backend;

pub struct GraphicsAPI {
    pub backend: Backend,
    pub webgpu: Option<WebGPUAPI>,
}

impl GraphicsAPI {
    pub fn new(backend: Backend, window: Arc<Window>) -> Self {
        match backend {
            Backend::WebGPU => {
                let mut webgpu = Some(WebGPUAPI::new(window.clone()));

                let size = window.inner_size();
                let width = size.width;
                let height = size.height;

                webgpu
                    .as_mut()
                    .expect("WebGPU backend isn't initialized!")
                    .configure(width, height);

                Self { backend, webgpu }
            }

            _ => panic!("Unknown backend!"),
        }
    }
}
