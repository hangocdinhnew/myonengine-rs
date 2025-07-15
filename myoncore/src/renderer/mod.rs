mod webgpu;

use std::cell::RefCell;
use std::rc::Rc;

use crate::graphics::GraphicsAPI;
use crate::rhi::Backend;
use webgpu::WebGPURenderer;

pub struct Renderer {
    pub webgpu: Option<WebGPURenderer>,
    graphicsapi: Rc<RefCell<GraphicsAPI>>,
}

impl Renderer {
    pub fn new(graphicsapi: Rc<RefCell<GraphicsAPI>>) -> Self {
        let backend;

        {
            let graphicsapi_borrow = graphicsapi.borrow();
            backend = graphicsapi_borrow.backend;
        }

        match backend {
            Backend::WebGPU => {
                let webgpu_inner = {
                    let graphicsapi_borrow = graphicsapi.borrow();
                    WebGPURenderer::new(
                        graphicsapi_borrow
                            .webgpu
                            .clone()
                            .expect("Graphics API failed to get."),
                    )
                };

                Self {
                    webgpu: Some(webgpu_inner),
                    graphicsapi,
                }
            }

            _ => panic!("Unknown backend!"),
        }
    }
}
