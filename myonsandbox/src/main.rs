use egui::Context;
use myoncore::{renderer::Renderer, utils::FrameTimer, AppHandler, Engine, EngineConfig};
use winit::{
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::Window,
};

struct Sandbox {
    show_fps: bool,
}

impl AppHandler for Sandbox {
    fn on_event(&mut self, event_loop: &ActiveEventLoop, event: &WindowEvent) {}

    fn on_update(&mut self) {}

    fn on_render(&mut self, renderer: &mut Renderer) {
        let texture_view = renderer.texture_view.as_ref().expect("TextureView missing");
        let encoder = renderer
            .command_encoder
            .as_mut()
            .expect("CommandEncoder missing");

        let render_pass_descriptor = wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: texture_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 1.0,
                        g: 1.0,
                        b: 1.0,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        };

        {
            let render_pass = encoder.begin_render_pass(&render_pass_descriptor);
        }
    }

    fn on_gui(
        &mut self,
        ctx: &mut Context,
        frametimer: &FrameTimer,
        window: &Window,
        event_loop: &ActiveEventLoop,
    ) {
        egui::TopBottomPanel::top("debug_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Exit").clicked() {
                        event_loop.exit();
                    }
                });

                #[cfg(debug_assertions)]
                ui.menu_button("View", |ui| {
                    if ui.button("Show FPS").clicked() {
                        self.show_fps = !self.show_fps;
                        ui.close();
                    }
                })
            });
        });

        if self.show_fps {
            egui::Window::new("FPS + Deltatime")
                .open(&mut self.show_fps)
                .resizable(true)
                .show(ctx, |ui| {
                    ui.label(format!("FPS: {:.2}", frametimer.fps));
                    ui.label(format!("Deltatime: {:.8}", frametimer.delta_time));
                });
        }
    }
}

fn main() -> anyhow::Result<()> {
    let event_loop = EventLoop::new()?;
    let engineconfig = EngineConfig::new()
        .title(String::from("MyonSandbox"))
        .width(800)
        .height(600)
        .resizable(true);

    let mut engine = Engine::new(engineconfig, Sandbox { show_fps: false });
    event_loop.run_app(&mut engine)?;

    Ok(())
}
