use crate::script::Script;

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum StackValue {
    String(String),
    Integer(i32),
    Float(f32),
    Size(i32, i32),
}

pub struct ExecContext {
    stack: Vec<StackValue>,
    heap: HashMap<String, StackValue>,
}

impl ExecContext {
    pub fn new() -> ExecContext {
        ExecContext {
            stack: Vec::new(),
            heap: HashMap::new(),
        }
    }

    pub fn execute(&mut self, script: &Script) {
    }
}
