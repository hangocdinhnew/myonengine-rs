use wgpu::{Instance, InstanceDescriptor};

#[derive(Default)]
pub struct WebGPUInstance {
    pub instancedescriptor: InstanceDescriptor,
    pub instance: Instance,
}

impl WebGPUInstance {
    pub fn new() -> Self {
        tracing::info!("Creating WebGPU instance...");

        let instancedescriptor = InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        };

        let instance = Instance::new(&instancedescriptor);

        Self {
            instancedescriptor,
            instance
        }
    }
}
