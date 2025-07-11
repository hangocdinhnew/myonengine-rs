use std::rc::Rc;
use webgpuinstance::WebGPUInstance;

mod webgpuinstance;

#[derive(Default)]
pub struct WebGPUAPI {
    pub instance: Rc<WebGPUInstance>
}

impl WebGPUAPI {
    pub fn new() -> Self {
        tracing::info!("Creating WebGPU backend...");

        let instance = Rc::new(WebGPUInstance::new());

        Self {
            instance
        }
    }
}
