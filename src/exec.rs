use crate::script::Script;
use crate::script::ScriptToken;
use crate::exec::except::ExecError;

use std::collections::HashMap;

pub mod except;

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

    pub fn push(&mut self, val: StackValue) {
        self.stack.push(val);
    }

    pub fn execute(&mut self, script: &Script) -> Result<(), ExecError> {
        println!("### running {:?}", script);

        for tok in script.tokens() {
            match tok {
                ScriptToken::Integer(val) => {
                    self.push(StackValue::Integer(*val));
                },
                ScriptToken::Float(val) => {
                    self.push(StackValue::Float(*val));
                },
                ScriptToken::String(val) => {
                    self.push(StackValue::String(val.clone()));
                },
                ScriptToken::Size(valx, valy) => {
                    self.push(StackValue::Size(*valx, *valy));
                },
                other => {
                    let msg = format!("Unknown token: {:?}", other);
                    return Err(ExecError::new(&msg))
                },
            }
        }

        Ok(())
    }
}
