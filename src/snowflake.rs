use std::sync::{Arc, Mutex};

use snowflake::SnowflakeIdGenerator;

#[derive(Debug, Clone)]
pub struct Snowflake {
    generator: Arc<Mutex<SnowflakeIdGenerator>>,
}

impl Snowflake {
    pub fn new() -> Self {
        Self {
            generator: Arc::new(Mutex::new(SnowflakeIdGenerator::new(1, 1))),
        }
    }

    pub fn generate(self) -> i64 {
        self.generator.lock().unwrap().generate()
    }
}
