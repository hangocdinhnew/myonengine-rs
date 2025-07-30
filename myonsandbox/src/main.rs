use myoncore::{renderer::Renderer, AppHandler, Engine, EngineConfig};
use winit::{
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
};

struct Sandbox;

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
                depth_slice: None,
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
}

fn main() -> anyhow::Result<()> {
    let event_loop = EventLoop::new()?;
    let engineconfig = EngineConfig::new(String::from("MyonSandbox"), 800, 600, true);
    let mut engine = Engine::new(engineconfig, Sandbox);
    event_loop.run_app(&mut engine)?;

    Ok(())
}
