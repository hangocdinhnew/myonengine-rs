use winit::{
    event_loop::ActiveEventLoop,
    window::{Window, WindowAttributes},
};

pub struct WindowSystem {
    pub window: Window,
}

impl WindowSystem {
    pub fn new(window_attributes: WindowAttributes, event_loop: &ActiveEventLoop) -> Self {
        let window = event_loop
            .create_window(window_attributes)
            .expect("Failed to create window!");

        Self { window }
    }
}
