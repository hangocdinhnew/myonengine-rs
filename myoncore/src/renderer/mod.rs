use std::iter;

use wgpu::{CommandEncoder, SurfaceTexture, TextureView};

use crate::graphics::Graphics;

pub struct Renderer {
    pub surface_texture: Option<SurfaceTexture>,
    pub texture_view: Option<TextureView>,
    pub command_encoder: Option<CommandEncoder>,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            surface_texture: None,
            texture_view: None,
            command_encoder: None,
        }
    }

    pub fn begin_frame(&mut self, graphics: &mut Graphics) -> Result<(), wgpu::SurfaceError> {
        self.surface_texture = Some(graphics.surface.get_current_texture()?);

        self.texture_view = Some(
            self.surface_texture
                .as_ref()
                .expect("Failed to acquire next texture")
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default()),
        );

        self.command_encoder = Some(
            graphics
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None }),
        );

        Ok(())
    }

    pub fn end_frame(&mut self, graphics: &mut Graphics) {
        graphics.queue.submit(iter::once(
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
