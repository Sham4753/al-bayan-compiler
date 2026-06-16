use crate::musarrif::Musarrif;
use crate::generator::Generator;
use crate::runtime::{BayanRuntime, Value};
use crate::bayan_engine::BayanEngine;

pub struct Interpreter {
    engine: BayanEngine,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter { engine: BayanEngine::new() }
    }

    pub fn execute(&mut self, line: &str) -> Result<Value, String> {
        self.engine.execute_line(line)
    }

    pub fn execute_file(&mut self, path: &str) -> Result<(), String> {
        println!("🕌 تنفيذ برنامج البيان: {}\n", path);
        self.engine.execute_file(path)
    }

    pub fn show_state(&self) {
        self.engine.show_memory();
    }
}
