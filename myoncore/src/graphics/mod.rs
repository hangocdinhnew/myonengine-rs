use std::sync::Arc;

use wgpu::{
    Adapter, Device, Instance, Queue, Surface, SurfaceCapabilities, SurfaceConfiguration,
    TextureFormat,
};
use winit::window::Window;

pub struct Graphics {
    pub instance: Instance,
    pub surface: Surface<'static>,
    pub adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
    pub surface_format: Option<TextureFormat>,
    surface_caps: Option<SurfaceCapabilities>,
    surface_config: Option<SurfaceConfiguration>,
}

impl Graphics {
    pub fn configure(&mut self, width: u32, height: u32) {
        tracing::info!("Configuring surface...");

        let surface_caps = self.surface.get_capabilities(&self.adapter);

        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width,
            height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            desired_maximum_frame_latency: 2,
            view_formats: vec![],
        };

        self.surface_caps = Some(surface_caps);
        self.surface_format = Some(surface_format);
        self.surface_config = Some(config);

        self.surface.configure(
            &self.device,
            self.surface_config
                .as_ref()
                .expect("Failed to unwrap surface_config"),
        );
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        let surface_config = self
            .surface_config
            .as_mut()
            .expect("Failed to get surface_format!");

        surface_config.width = width;
        surface_config.height = height;

        self.surface.configure(
            &self.device,
            self.surface_config
                .as_ref()
                .expect("Failed to unwrap surface_config"),
        );
    }

    pub fn new(window: Arc<Window>) -> Self {
        tracing::info!("Creating WebGPU backend...");

        tracing::debug!("Creating Instance...");

        let instancedescriptor = wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        };

        let instance = Instance::new(&instancedescriptor);

        tracing::debug!("Creating surface...");

        let surface = instance
            .create_surface(window)
            .expect("Failed to create surface!");

        tracing::debug!("Requesting adapter...");

        let request_adapter_options = wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        };

        let adapter = pollster::block_on(instance.request_adapter(&request_adapter_options))
            .expect("Failed to request adapter!");

        tracing::debug!("Creating device...");

        let descriptor = wgpu::DeviceDescriptor {
            label: None,
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
            memory_hints: Default::default(),
            trace: wgpu::Trace::Off,
        };

        let (device, queue) = pollster::block_on(adapter.request_device(&descriptor))
            .expect("Failed to create device/queue!");

        Self {
            instance,
            surface,
            adapter,
            device,
            queue,
            surface_caps: None,
            surface_format: None,
            surface_config: None,
        }
    }
}
