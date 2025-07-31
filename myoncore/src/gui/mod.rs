use std::sync::Arc;

use egui::ViewportId;

use egui::Context as EguiContext;
use egui_wgpu::Renderer as EguiRenderer;
use egui_wgpu::ScreenDescriptor;
use egui_winit::State as EguiWinitState;
use winit::window::Theme;
use winit::window::Window;

use crate::graphics::Graphics;
use crate::renderer::Renderer;

pub struct Gui {
    pub ctx: EguiContext,
    state: EguiWinitState,
    renderer: EguiRenderer,
}

impl Gui {
    pub fn new(window: Arc<Window>, graphics: &Graphics) -> Self {
        let ctx = EguiContext::default();
        let state = EguiWinitState::new(
            ctx.clone(),
            ViewportId::ROOT,
            window.as_ref(),
            Some(window.clone().scale_factor() as f32),
            Some(Theme::Dark),
            None,
        );

        let surface_format = graphics
            .surface_format
            .expect("Failed to get surface_format!");

        let renderer = EguiRenderer::new(&graphics.device.device, surface_format, None, 1, false);

        Self {
            ctx,
            state,
            renderer,
        }
    }

    pub fn handle_event(&mut self, window: &Window, event: &winit::event::WindowEvent) {
        let _ = self.state.on_window_event(window, event);
    }

    pub fn begin_frame(&mut self, window: &Window) {
        let raw_input = self.state.take_egui_input(window);
        self.ctx.begin_pass(raw_input);
    }

    pub fn end_frame(&mut self, window: &Window, graphics: &Graphics, renderer: &mut Renderer) {
        let texture_view = renderer.texture_view.as_ref().expect("TextureView missing");
        let encoder = renderer
            .command_encoder
            .as_mut()
            .expect("CommandEncoder missing");

        let full_output = self.ctx.end_pass();
        let paint_jobs = self
            .ctx
            .tessellate(full_output.shapes, self.ctx.pixels_per_point());

        let window_size = window.inner_size();
        let screen_descriptor = ScreenDescriptor {
            size_in_pixels: [window_size.width, window_size.height],
            pixels_per_point: self.ctx.pixels_per_point(),
        };

        self.state
            .handle_platform_output(window, full_output.platform_output);

        for (id, image_delta) in &full_output.textures_delta.set {
            self.renderer.update_texture(
                &graphics.device.device,
                &graphics.device.queue,
                *id,
                image_delta,
            );
        }

        self.renderer.update_buffers(
            &graphics.device.device,
            &graphics.device.queue,
            encoder,
            &paint_jobs,
            &screen_descriptor,
        );

        let render_pass_descriptor = wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: texture_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        };

        {
            let mut render_pass = encoder.begin_render_pass(&render_pass_descriptor);

            let static_render_pass: &mut wgpu::RenderPass<'static> =
                unsafe { std::mem::transmute(&mut render_pass) };

            self.renderer
                .render(static_render_pass, &paint_jobs, &screen_descriptor);

            for x in &full_output.textures_delta.free {
                self.renderer.free_texture(x)
            }
        }
    }
}
