use winit::{
    application::ApplicationHandler, dpi::LogicalSize, event::WindowEvent,
    event_loop::ActiveEventLoop, window::WindowAttributes,
};

use crate::{
    graphics::GraphicsAPI, logger::Logger, renderer::Renderer, rhi::Backend, window::WindowSystem,
};

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
    fn on_render(&mut self, renderer: &mut Renderer, backend: Backend);
}

#[derive(Default)]
pub struct Engine<A: AppHandler> {
    config: EngineConfig,
    logger: Logger,
    windowsys: Option<WindowSystem>,
    graphicsapi: Option<GraphicsAPI>,
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
            graphicsapi: None,
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

        let graphicsapi = GraphicsAPI::new(
            Backend::WebGPU,
            self.windowsys
                .as_ref()
                .expect("Failed to acquire windowsys")
                .window
                .clone(),
        );
        self.graphicsapi = Some(graphicsapi);
        tracing::info!("Graphics API created!");

        let renderer = Renderer::new(
            self.graphicsapi
                .as_mut()
                .expect("Failed to acquire graphicsapi!")
                .backend,
        );
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
                let graphicsapi = self
                    .graphicsapi
                    .as_mut()
                    .expect("Failed to get graphicsapi");

                let renderer = self.renderer.as_mut().expect("Failed to get renderer");

                match graphicsapi.backend {
                    Backend::WebGPU => {
                        {
                            let webgpu = renderer
                                .webgpu
                                .as_mut()
                                .expect("Failed to get WebGPU renderer.");

                            let webgpu_api = graphicsapi
                                .webgpu
                                .as_mut()
                                .expect("Failed to get webgpu backend!");

                            match webgpu.begin_frame(webgpu_api) {
                                Ok(_) => {}

                                Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                                    let size = self
                                        .windowsys
                                        .as_ref()
                                        .expect("Failed to get windowsys")
                                        .window
                                        .inner_size();

                                    webgpu_api.resize(size.width, size.height);

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
                        }

                        self.app.on_render(renderer, graphicsapi.backend);

                        {
                            let webgpu = renderer
                                .webgpu
                                .as_mut()
                                .expect("Failed to get WebGPU renderer.");

                            let webgpu_api = graphicsapi
                                .webgpu
                                .as_mut()
                                .expect("Failed to get webgpu backend!");

                            webgpu.end_frame(webgpu_api);
                        }

                        self.windowsys
                            .as_ref()
                            .expect("Failed to get windowsys")
                            .window
                            .request_redraw();
                    }

                    _ => panic!("Unknown backend!"),
                }
            }

            WindowEvent::Resized(size) => {
                let graphicsapi = self
                    .graphicsapi
                    .as_mut()
                    .expect("Failed to get graphicsapi");

                let webgpu = graphicsapi
                    .webgpu
                    .as_mut()
                    .expect("Failed to get webgpu backend!");

                match graphicsapi.backend {
                    Backend::WebGPU => {
                        webgpu.resize(size.width, size.height);

                        self.windowsys
                            .as_ref()
                            .expect("Failed to get windowsys")
                            .window
                            .request_redraw();
                    }

                    _ => {}
                }
            }

            _ => {}
        }

        self.app.on_event(event_loop, &event);
    }
}
