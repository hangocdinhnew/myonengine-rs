use wgpu::{Adapter, Device, Queue};

pub struct WebGPUDevice {
    pub device: Device,
    pub queue: Queue,
}

impl WebGPUDevice {
    pub fn new(adapter: &Adapter) -> Self {
        tracing::info!("Requesting device...");

        let descriptor = wgpu::DeviceDescriptor {
            label: None,
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::defaults(),
            memory_hints: Default::default(),
            trace: wgpu::Trace::Off,
        };

        let (device, queue) = pollster::block_on(adapter.request_device(&descriptor))
            .expect("Failed to create device/queue!");

        Self { device, queue }
    }
}
