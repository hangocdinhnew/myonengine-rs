use super::{webgpuinstance::WebGPUInstance, webgpusurface::WebGPUSurface};
use std::rc::Rc;
use wgpu::{Adapter, RequestAdapterOptions};

pub struct WebGPUAdapter {
    pub adapter: Adapter,
    instance: Rc<WebGPUInstance>,
    surface: Rc<WebGPUSurface>,
}

impl WebGPUAdapter {
    pub fn new(instance: Rc<WebGPUInstance>, surface: Rc<WebGPUSurface>) -> Self {
        let request_adapter_options = RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface.surface),
            force_fallback_adapter: false,
        };

        let adapter =
            pollster::block_on(instance.instance.request_adapter(&request_adapter_options))
                .expect("Failed to request adapter!");

        Self {
            adapter,
            instance,
            surface,
        }
    }
}
