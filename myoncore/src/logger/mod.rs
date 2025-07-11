use tracing::info;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{EnvFilter, fmt};

#[derive(Default)]
pub struct Logger;

impl Logger {
    pub fn new() -> Self {
        let default_level = if cfg!(debug_assertions) {
            "trace"
        } else {
            "info"
        };

        let env_filter =
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(default_level));

        let fmt_layer = fmt::layer()
            .with_target(true)
            .with_timer(fmt::time::time())
            .with_level(true)
            .with_thread_names(true)
            .with_ansi(true);

        tracing_subscriber::registry()
            .with(env_filter)
            .with(fmt_layer)
            .init();

        info!("Logger system initialized!");

        Self
    }
}
