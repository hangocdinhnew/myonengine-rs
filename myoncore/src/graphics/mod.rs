mod webgpu;

use std::{rc::Rc, sync::Arc};
use webgpu::WebGPUAPI;
use winit::window::Window;

use crate::window::WindowSystem;

pub struct GraphicsAPI {
    pub webgpu: WebGPUAPI,
    window: Rc<WindowSystem>,
}

impl GraphicsAPI {
    pub fn new(window: Rc<WindowSystem>) -> Self {
        let webgpu = WebGPUAPI::new(window.window.clone());

        Self { webgpu, window }
    }
}
