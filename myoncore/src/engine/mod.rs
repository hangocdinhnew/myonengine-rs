use std::{cell::RefCell, rc::Rc};
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
    fn on_render(&mut self, renderer: Rc<RefCell<Renderer>>, backend: Backend);
}

#[derive(Default)]
pub struct Engine<A: AppHandler> {
    config: EngineConfig,
    logger: Logger,
    windowsys: Option<Rc<WindowSystem>>,
    graphicsapi: Option<Rc<RefCell<GraphicsAPI>>>,
    renderer: Option<Rc<RefCell<Renderer>>>,
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

        let windowsys = Rc::new(WindowSystem::new(window_attributes, event_loop));

        tracing::info!("Window created!");

        let graphicsapi = Rc::new(RefCell::new(GraphicsAPI::new(
            Backend::WebGPU,
            windowsys.clone(),
        )));

        tracing::info!("Graphics API created!");

        let renderer = Rc::new(RefCell::new(Renderer::new(graphicsapi.clone())));

        tracing::info!("Renderer created!");

        self.windowsys = Some(windowsys);
        self.graphicsapi = Some(graphicsapi);
        self.renderer = Some(renderer);

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
                let mut graphicsapi = self
                    .graphicsapi
                    .as_ref()
                    .expect("Failed to get graphicsapi")
                    .borrow_mut();

                let renderer_rc = self
                    .renderer
                    .as_ref()
                    .expect("Failed to get renderer")
                    .clone();

                match graphicsapi.backend {
                    Backend::WebGPU => {
                        {
                            let mut renderer = renderer_rc.borrow_mut();
                            let webgpu = renderer
                                .webgpu
                                .as_mut()
                                .expect("Failed to get WebGPU renderer.");

                            match webgpu.acquire_next_image() {
                                Ok(_) => {}

                                Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                                    let mut webgpu = graphicsapi
                                        .webgpu
                                        .as_mut()
                                        .expect("Failed to get webgpu backend!")
                                        .borrow_mut();

                                    let size = self
                                        .windowsys
                                        .as_ref()
                                        .expect("Failed to get windowsys")
                                        .window
                                        .inner_size();

                                    webgpu.resize(size.width, size.height);

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

                            webgpu.create_texture_view();
                            webgpu.begin_command_buffer();
                        }

                        self.app.on_render(renderer_rc.clone(), graphicsapi.backend);

                        {
                            let mut renderer = renderer_rc.borrow_mut();
                            let webgpu = renderer
                                .webgpu
                                .as_mut()
                                .expect("Failed to get WebGPU renderer.");
                            webgpu.submit_command_buffer();
                            webgpu.present_surface_texture();
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
                let mut graphicsapi = self
                    .graphicsapi
                    .as_ref()
                    .expect("Failed to get graphicsapi")
                    .borrow_mut();

                match graphicsapi.backend {
                    Backend::WebGPU => {
                        let mut webgpu = graphicsapi
                            .webgpu
                            .as_mut()
                            .expect("Failed to get webgpu backend!")
                            .borrow_mut();

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
