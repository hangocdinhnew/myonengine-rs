use crate::logger::Logger;

pub struct Engine {
    logger: Logger,
}

impl Engine {
    pub fn new() -> Self {
        let logger = Logger::new();

        Self { logger }
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}
