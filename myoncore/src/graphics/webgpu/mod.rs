mod webgpuadapter;
mod webgpudevice;
mod webgpuinstance;
mod webgpusurface;

use std::{rc::Rc, sync::Arc};
use winit::window::Window;

use webgpuadapter::WebGPUAdapter;
use webgpudevice::WebGPUDevice;
use webgpuinstance::WebGPUInstance;
use webgpusurface::WebGPUSurface;

pub struct WebGPUAPI {
    pub instance: Rc<WebGPUInstance>,
    pub surface: Rc<WebGPUSurface>,
    pub adapter: Rc<WebGPUAdapter>,
    pub device: Rc<WebGPUDevice>,
    window: Arc<Window>,
}

impl WebGPUAPI {
    pub fn new(window: Arc<Window>) -> Self {
        tracing::info!("Creating WebGPU backend...");

        let instance = Rc::new(WebGPUInstance::new());
        let surface = Rc::new(WebGPUSurface::new(instance.clone(), window.clone()));
        let adapter = Rc::new(WebGPUAdapter::new(instance.clone(), surface.clone()));
        let device = Rc::new(WebGPUDevice::new(adapter.clone()));

        Self {
            instance,
            surface,
            adapter,
            device,
            window,
        }
    }
}
