mod webgpu;

use std::cell::RefCell;
use std::{rc::Rc, sync::Arc};
pub use webgpu::WebGPUAPI;
use winit::window::Window;

use crate::rhi::Backend;
use crate::window::WindowSystem;

pub struct GraphicsAPI {
    pub backend: Backend,
    pub webgpu: Option<Rc<RefCell<WebGPUAPI>>>,
    window: Rc<WindowSystem>,
}

impl GraphicsAPI {
    pub fn new(backend: Backend, window: Rc<WindowSystem>) -> Self {
        match backend {
            Backend::WebGPU => {
                let mut webgpu = Some(Rc::new(RefCell::new(WebGPUAPI::new(window.window.clone()))));

                let size = window.window.inner_size();
                let width = size.width;
                let height = size.height;

                webgpu
                    .as_mut()
                    .expect("WebGPU backend isn't initialized!")
                    .borrow_mut()
                    .configure(width, height);

                Self {
                    backend,
                    webgpu,
                    window,
                }
            }

            _ => panic!("Unknown backend!")
        }
    }
}
