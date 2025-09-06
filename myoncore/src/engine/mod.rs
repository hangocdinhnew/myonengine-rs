use egui::Context;
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowAttributes},
};

use std::ptr;

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

pub struct Engine<A: AppHandler> {
    config: EngineConfig,
    frame_timer: FrameTimer,
    logger: Logger,
    windowsys: *mut WindowSystem,
    graphics: *mut Graphics,
    renderer: *mut Renderer,
    gui: *mut Gui,
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
	    windowsys: ptr::null_mut(),
	    graphics: ptr::null_mut(),
	    renderer: ptr::null_mut(),
	    gui: ptr::null_mut(),
	    app,
        }
    }

    pub unsafe fn unsafe_resumed(&mut self, event_loop: &ActiveEventLoop) {
	unsafe {
            let window_attributes = WindowAttributes::default()
		.with_title(&self.config.title)
		.with_inner_size(LogicalSize::new(self.config.width, self.config.height))
		.with_resizable(self.config.resizable)
		.with_decorations(!self.config.without_titlebar);

            let windowsys_box = Box::new(WindowSystem::new(window_attributes, event_loop));
	    self.windowsys = Box::into_raw(windowsys_box) as *mut WindowSystem;
            tracing::info!("Window created!");

            let window = &(*self.windowsys)
		.window as *const Window;

            let mut graphics = Graphics::new(window);

            let size = (*window).inner_size();
            let width = size.width;
            let height = size.height;

            graphics.configure(width, height);

	    let graphics_box = Box::new(graphics);
	    self.graphics = Box::into_raw(graphics_box) as *mut Graphics;
            tracing::info!("Graphics API created!");

            let renderer_box = Box::new(Renderer::new(self.graphics));
	    self.renderer = Box::into_raw(renderer_box) as *mut Renderer;
            tracing::info!("Renderer created!");

            let gui_box = Box::new(Gui::new(
		window,
		self.graphics,
		self.renderer,
            ));
	    self.gui = Box::into_raw(gui_box) as *mut Gui;
            tracing::info!("Created GUI!");

            self.app.on_update();
	}
    }

    pub unsafe fn unsafe_window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _id: winit::window::WindowId,
        event: WindowEvent,
    ) {
	unsafe {
            let windowsys = &mut (*self.windowsys);
            let graphics = &mut (*self.graphics);
            let renderer = &mut (*self.renderer);
            let gui = &mut (*self.gui);

	    let window = &(*windowsys).window
		as *const Window;

            self.frame_timer.update();

            gui.handle_event(&event);

            match event {
		WindowEvent::CloseRequested => {
                    tracing::info!("Closing...");
                    event_loop.exit();
		}

		WindowEvent::RedrawRequested => {
                    match renderer.begin_frame() {
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

                    gui.begin_frame();

                    self.app.on_gui(
			&mut gui.ctx,
			&self.frame_timer,
			&*window,
			event_loop,
                    );

                    gui.end_frame();
                    renderer.end_frame();

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
}

impl<A: AppHandler> ApplicationHandler for Engine<A> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
	unsafe {
	    self.unsafe_resumed(event_loop);
	}
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        id: winit::window::WindowId,
        event: WindowEvent,
    ) {
	unsafe {
	    self.unsafe_window_event(event_loop, id, event);
	}
    }
}

impl<A: AppHandler> Drop for Engine<A> {
    fn drop(&mut self) {
	unsafe {
	    let _ = Box::from_raw(self.windowsys);
	    let _ = Box::from_raw(self.graphics);
	    let _ = Box::from_raw(self.renderer);
	    let _ = Box::from_raw(self.gui);
	}
    }
}
