use std::time::{Duration, Instant};

pub struct FrameTimer {
    last_instant: Instant,
    frame_count: u32,
    accumulated: Duration,
    pub fps: f32,
    pub delta_time: f32,
}

impl Default for FrameTimer {
    fn default() -> Self {
        Self::new()
    }
}

impl FrameTimer {
    pub fn new() -> Self {
        Self {
            last_instant: Instant::now(),
            frame_count: 0,
            accumulated: Duration::ZERO,
            fps: 0.0,
            delta_time: 0.0,
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        let frame_time = now - self.last_instant;
        self.last_instant = now;

        self.delta_time = frame_time.as_secs_f32();

        self.accumulated += frame_time;
        self.frame_count += 1;

        if self.accumulated >= Duration::from_secs(1) {
            self.fps = self.frame_count as f32 / self.accumulated.as_secs_f32();
            self.frame_count = 0;
            self.accumulated = Duration::ZERO;
        }
    }

    pub fn fps(&self) -> f32 {
        self.fps
    }
}
