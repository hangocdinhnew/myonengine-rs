use std::iter;

use wgpu::{CommandEncoder, RenderPassDescriptor, SurfaceTexture, TextureView};

use crate::graphics::WebGPUAPI;

pub struct WebGPURenderer {
    pub surface_texture: Option<SurfaceTexture>,
    pub texture_view: Option<TextureView>,
    pub command_encoder: Option<CommandEncoder>,
}

impl WebGPURenderer {
    pub fn new() -> Self {
        Self {
            surface_texture: None,
            texture_view: None,
            command_encoder: None,
        }
    }

    pub fn begin_frame(&mut self, webgpu: &mut WebGPUAPI) -> Result<(), wgpu::SurfaceError> {
        self.surface_texture = Some(webgpu.surface.surface.get_current_texture()?);

        self.texture_view = Some(
            self.surface_texture
                .as_ref()
                .expect("Failed to acquire next texture")
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default()),
        );

        self.command_encoder = Some(
            webgpu
                .device
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None }),
        );

        Ok(())
    }

    pub fn create_render_pass(&mut self, descriptor: &RenderPassDescriptor) -> wgpu::RenderPass {
        self.command_encoder
            .as_mut()
            .expect("Failed to create render pass.")
            .begin_render_pass(descriptor)
    }

    pub fn end_frame(&mut self, webgpu: &mut WebGPUAPI) {
        webgpu.device.queue.submit(iter::once(
            self.command_encoder
                .take()
                .expect("Failed to submit command encoder.")
                .finish(),
        ));

        self.surface_texture
            .take()
            .expect("Failed to present surface texture")
            .present();
    }
}
