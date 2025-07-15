use std::{cell::RefCell, iter, rc::Rc};
use wgpu::{CommandEncoder, RenderPassDescriptor, SurfaceTexture, TextureView};

use crate::graphics::WebGPUAPI;

pub struct WebGPURenderer {
    pub surface_texture: Option<SurfaceTexture>,
    pub texture_view: Option<TextureView>,
    pub command_encoder: Option<CommandEncoder>,

    webgpu: Rc<RefCell<WebGPUAPI>>,
}

impl WebGPURenderer {
    pub fn new(webgpu: Rc<RefCell<WebGPUAPI>>) -> Self {
        Self {
            surface_texture: None,
            texture_view: None,
            command_encoder: None,
            webgpu,
        }
    }

    pub fn acquire_next_image(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.surface_texture = Some(self.webgpu.borrow().surface.surface.get_current_texture()?);
        Ok(())
    }

    pub fn create_texture_view(&mut self) {
        self.texture_view = Some(
            self.surface_texture
                .as_ref()
                .expect("Failed to acquire next texture")
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default()),
        );
    }

    pub fn begin_command_buffer(&mut self) {
        self.command_encoder = Some(
            self.webgpu
                .borrow()
                .device
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None }),
        );
    }

    pub fn submit_command_buffer(&mut self) {
        self.webgpu.borrow().device.queue.submit(iter::once(
            self.command_encoder
                .take()
                .expect("Failed to submit command encoder.")
                .finish(),
        ));
    }

    pub fn present_surface_texture(&mut self) {
        self.surface_texture
            .take()
            .expect("Failed to present surface texture")
            .present();
    }

    pub fn create_render_pass(&mut self, descriptor: &RenderPassDescriptor) -> wgpu::RenderPass {
        self.command_encoder
            .as_mut()
            .expect("Failed to create render pass.")
            .begin_render_pass(descriptor)
    }
}
