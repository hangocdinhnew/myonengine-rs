use super::webgpuadapter::WebGPUAdapter;
use std::rc::Rc;
use wgpu::{Device, Queue};

pub struct WebGPUDevice {
    pub device: Device,
    pub queue: Queue,
    adapter: Rc<WebGPUAdapter>,
}

impl WebGPUDevice {
    pub fn new(adapter: Rc<WebGPUAdapter>) -> Self {
        tracing::info!("Requesting device...");

        let descriptor = wgpu::DeviceDescriptor {
            label: None,
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::defaults(),
            memory_hints: Default::default(),
            trace: wgpu::Trace::Off,
        };

        let (device, queue) = pollster::block_on(adapter.adapter.request_device(&descriptor))
            .expect("Failed to create device/queue!");

        Self {
            device,
            queue,
            adapter,
        }
    }
}
