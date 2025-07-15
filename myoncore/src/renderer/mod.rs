mod webgpu;

use std::rc::Rc;

use crate::graphics::GraphicsAPI;
use crate::rhi::Backend;
use webgpu::WebGPURenderer;

pub struct Renderer {
    pub webgpu: Option<WebGPURenderer>,
    graphicsapi: Rc<GraphicsAPI>,
}

impl Renderer {
    pub fn new(graphicsapi: Rc<GraphicsAPI>) -> Self {
        match graphicsapi.backend {
            Backend::WebGPU => {
                let mut webgpu = Some(WebGPURenderer::new(
                    graphicsapi.webgpu.clone().expect("Graphics API failed to get."),
                ));

                Self {
                    webgpu,
                    graphicsapi,
                }
            }

            _ => {
                panic!("Unknown backend!");
            }
        }
    }
}
