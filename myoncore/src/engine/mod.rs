use egui::Context;
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowAttributes},
};

use crate::{
    graphics::Graphics, gui::Gui, logger::Logger, renderer::Renderer, utils::FrameTimer,
    window::WindowSystem,
};

#[derive(Default)]
pub struct EngineConfig {
    title: String,
    width: u32,
    height: u32,
    resizable: bool,
    without_titlebar: bool,
}

impl EngineConfig {
    pub fn new() -> Self {
        Self {
            title: String::from(""),
            width: 0,
            height: 0,
            resizable: false,
            without_titlebar: false,
        }
    }

    pub fn title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    pub fn width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }

    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }

    pub fn without_titlebar(mut self, without_titlebar: bool) -> Self {
        self.without_titlebar = without_titlebar;
        self
    }
}

pub trait AppHandler {
    fn on_event(&mut self, event_loop: &ActiveEventLoop, event: &WindowEvent);
    fn on_update(&mut self);
    fn on_render(&mut self, renderer: &mut Renderer);
    fn on_gui(
        &mut self,
        ctx: &mut Context,
        frametimer: &FrameTimer,
        window: &Window,
        event_loop: &ActiveEventLoop,
    );
}

#[derive(Default)]
pub struct Engine<A: AppHandler> {
    config: EngineConfig,
    frame_timer: FrameTimer,
    logger: Logger,
    windowsys: Option<WindowSystem>,
    graphics: Option<Graphics>,
    renderer: Option<Renderer>,
    gui: Option<Gui>,
    app: A,
}

impl<A: AppHandler> Engine<A> {
    pub fn new(config: EngineConfig, app: A) -> Self {
        let frame_timer = FrameTimer::new();
        let logger = Logger::new();

        Self {
            config,
            frame_timer,
            logger,
            windowsys: None,
            graphics: None,
            renderer: None,
            gui: None,
            app,
        }
    }
}

impl<A: AppHandler> ApplicationHandler for Engine<A> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = WindowAttributes::default()
            .with_title(&self.config.title)
            .with_inner_size(LogicalSize::new(self.config.width, self.config.height))
            .with_resizable(self.config.resizable)
            .with_decorations(!self.config.without_titlebar);

        let windowsys = WindowSystem::new(window_attributes, event_loop);
        self.windowsys = Some(windowsys);
        tracing::info!("Window created!");

        let window = self
            .windowsys
            .as_ref()
            .expect("Failed to acquire windowsys")
            .window
            .clone();

        let mut graphics = Graphics::new(window.clone());

        let size = window.inner_size();
        let width = size.width;
        let height = size.height;

        graphics.configure(width, height);

        self.graphics = Some(graphics);
        tracing::info!("Graphics API created!");

        let renderer = Renderer::new();
        self.renderer = Some(renderer);
        tracing::info!("Renderer created!");

        let gui = Gui::new(
            window,
            self.graphics
                .as_ref()
                .expect("Failed to acquire GraphicsAPI!"),
        );
        self.gui = Some(gui);
        tracing::info!("Created GUI!");

        self.app.on_update();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let windowsys = self.windowsys.as_mut().expect("Failed to get windowsys");
        let graphics = self.graphics.as_mut().expect("Failed to get graphicsapi");
        let renderer = self.renderer.as_mut().expect("Failed to get renderer");
        let gui = self.gui.as_mut().expect("Failed to get GUI");

        self.frame_timer.update();

        gui.handle_event(windowsys.window.as_ref(), &event);

        match event {
            WindowEvent::CloseRequested => {
                tracing::info!("Closing...");
                event_loop.exit();
            }

            WindowEvent::RedrawRequested => {
                match renderer.begin_frame(graphics) {
                    Ok(_) => {}

                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        let size = windowsys.window.inner_size();

                        graphics.resize(size.width, size.height);

                        windowsys.window.request_redraw();
                    }

                    Err(e) => {
                        panic!("Unable to render, reason: {e}")
                    }
                }

                self.app.on_render(renderer);

                gui.begin_frame(windowsys.window.as_ref());

                self.app.on_gui(
                    &mut gui.ctx,
                    &self.frame_timer,
                    windowsys.window.as_ref(),
                    event_loop,
                );

                gui.end_frame(windowsys.window.as_ref(), graphics, renderer);
                renderer.end_frame(graphics);

                windowsys.window.request_redraw();
            }

            WindowEvent::Resized(size) => {
                graphics.resize(size.width, size.height);

                windowsys.window.request_redraw();
            }

            _ => {}
        }

        self.app.on_event(event_loop, &event);
    }
}
