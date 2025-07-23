use std::rc::Rc;
use wgpu::{Adapter, Instance, RequestAdapterOptions, Surface};

pub struct WebGPUAdapter {
    pub adapter: Adapter,
}

impl WebGPUAdapter {
    pub fn new(instance: &Instance, surface: &Surface) -> Self {
        tracing::info!("Requesting adapter...");

        let request_adapter_options = RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(surface),
            force_fallback_adapter: false,
        };

        let adapter = pollster::block_on(instance.request_adapter(&request_adapter_options))
            .expect("Failed to request adapter!");

        Self { adapter }
    }
}
