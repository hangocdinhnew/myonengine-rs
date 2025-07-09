use crate::logger::Logger;
use std::sync::Arc;

pub struct Engine {
    logger: Arc<Logger>,
}

impl Engine {
    pub fn new() -> Self {
        let logger = Arc::new(Logger::new());

        Self { logger }
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}
