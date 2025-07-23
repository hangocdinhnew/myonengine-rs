mod webgpu;

use crate::rhi::Backend;
use webgpu::WebGPURenderer;

pub struct Renderer {
    pub webgpu: Option<WebGPURenderer>,
}

impl Renderer {
    pub fn new(backend: Backend) -> Self {
        match backend {
            Backend::WebGPU => {
                let webgpu_inner = WebGPURenderer::new();

                Self {
                    webgpu: Some(webgpu_inner),
                }
            }

            _ => panic!("Unknown backend!"),
        }
    }
}
