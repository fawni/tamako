use std::sync::{Arc, Mutex};

use snowflake::SnowflakeIdGenerator;

/// A wrapper around the snowflake generator that holds its state
#[derive(Debug, Clone)]
pub struct Snowflake {
    /// The snowflake generator
    generator: Arc<Mutex<SnowflakeIdGenerator>>,
}

impl Snowflake {
    /// Creates a new snowflake generator
    pub fn new() -> Self {
        Self {
            generator: Arc::new(Mutex::new(SnowflakeIdGenerator::new(1, 1))),
        }
    }

    /// Generates a new snowflake
    pub fn generate(self) -> i64 {
        self.generator.lock().unwrap().generate()
    }
}
