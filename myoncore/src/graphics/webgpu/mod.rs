mod webgpuadapter;
mod webgpudevice;
mod webgpuinstance;
mod webgpusurface;

use std::{rc::Rc, sync::Arc};
use wgpu::{SurfaceCapabilities, SurfaceConfiguration, TextureFormat};
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
    surface_caps: Option<SurfaceCapabilities>,
    surface_format: Option<TextureFormat>,
    surface_config: Option<SurfaceConfiguration>,
    window: Arc<Window>,
}

impl WebGPUAPI {
    pub fn configure(&mut self, width: u32, height: u32) {
        tracing::info!("Configuring surface...");

        let surface_caps = self.surface.surface.get_capabilities(&self.adapter.adapter);

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

        self.surface.surface.configure(
            &self.device.device,
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

        self.surface.surface.configure(
            &self.device.device,
            self.surface_config
                .as_ref()
                .expect("Failed to unwrap surface_config"),
        );
    }

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
            surface_caps: None,
            surface_format: None,
            surface_config: None,
            window,
        }
    }
}
