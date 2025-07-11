mod webgpuinstance;
mod webgpusurface;

use std::{rc::Rc, sync::Arc};
use webgpuinstance::WebGPUInstance;
use webgpusurface::WebGPUSurface;
use winit::window::Window;

pub struct WebGPUAPI {
    pub instance: Rc<WebGPUInstance>,
    pub surface: Rc<WebGPUSurface>,
    window: Arc<Window>
}

impl WebGPUAPI {
    pub fn new(window: Arc<Window>) -> Self {
        tracing::info!("Creating WebGPU backend...");

        let instance = Rc::new(WebGPUInstance::new());
        let surface = Rc::new(WebGPUSurface::new(instance.clone(), window.clone()));

        Self {
            instance,
            surface,
            window
        }
    }
}
