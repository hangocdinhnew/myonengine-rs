use winit::{
    application::ApplicationHandler, dpi::LogicalSize, event::WindowEvent,
    event_loop::ActiveEventLoop, window::WindowAttributes,
};

use crate::{graphics::Graphics, logger::Logger, renderer::Renderer, window::WindowSystem};

#[derive(Default)]
pub struct EngineConfig {
    title: String,
    width: u16,
    height: u16,
    resizable: bool,
}

impl EngineConfig {
    pub fn new(title: String, width: u16, height: u16, resizable: bool) -> Self {
        Self {
            title,
            width,
            height,
            resizable,
        }
    }
}

pub trait AppHandler {
    fn on_event(&mut self, event_loop: &ActiveEventLoop, event: &WindowEvent);
    fn on_update(&mut self);
    fn on_render(&mut self, renderer: &mut Renderer);
}

#[derive(Default)]
pub struct Engine<A: AppHandler> {
    config: EngineConfig,
    logger: Logger,
    windowsys: Option<WindowSystem>,
    graphics: Option<Graphics>,
    renderer: Option<Renderer>,
    app: A,
}

impl<A: AppHandler> Engine<A> {
    pub fn new(config: EngineConfig, app: A) -> Self {
        let logger = Logger::new();

        Self {
            config,
            logger,
            windowsys: None,
            graphics: None,
            renderer: None,
            app,
        }
    }
}

impl<A: AppHandler> ApplicationHandler for Engine<A> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = WindowAttributes::default()
            .with_title(&self.config.title)
            .with_inner_size(LogicalSize::new(self.config.width, self.config.height))
            .with_resizable(self.config.resizable);

        let windowsys = WindowSystem::new(window_attributes, event_loop);
        self.windowsys = Some(windowsys);
        tracing::info!("Window created!");

        let window = self
            .windowsys
            .as_ref()
            .expect("Failed to acquire windowsys")
            .window
            .clone();

        let mut graphics = Graphics::new(
            window.clone()
        );

        let size = window.inner_size();
        let width = size.width;
        let height = size.height;

        graphics.configure(width, height);

        self.graphics = Some(graphics);
        tracing::info!("Graphics API created!");

        let renderer = Renderer::new();
        self.renderer = Some(renderer);
        tracing::info!("Renderer created!");

        self.app.on_update();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                tracing::info!("Closing...");
                event_loop.exit();
            }

            WindowEvent::RedrawRequested => {
                let graphics = self.graphics.as_mut().expect("Failed to get graphicsapi");
                let renderer = self.renderer.as_mut().expect("Failed to get renderer");

                match renderer.begin_frame(graphics) {
                    Ok(_) => {}

                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        let size = self
                            .windowsys
                            .as_ref()
                            .expect("Failed to get windowsys")
                            .window
                            .inner_size();

                        graphics.resize(size.width, size.height);

                        self.windowsys
                            .as_ref()
                            .expect("Failed to get windowsys")
                            .window
                            .request_redraw();
                    }

                    Err(e) => {
                        panic!("Unable to render, reason: {e}")
                    }
                }

                self.app.on_render(renderer);

                renderer.end_frame(graphics);

                self.windowsys
                    .as_ref()
                    .expect("Failed to get windowsys")
                    .window
                    .request_redraw();
            }

            WindowEvent::Resized(size) => {
                let graphics = self.graphics.as_mut().expect("Failed to get graphicsapi");

                graphics.resize(size.width, size.height);

                self.windowsys
                    .as_ref()
                    .expect("Failed to get windowsys")
                    .window
                    .request_redraw();
            }

            _ => {}
        }

        self.app.on_event(event_loop, &event);
    }
}
