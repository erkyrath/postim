use crate::script::Script;
use crate::script::ScriptToken;
use crate::exec::except::ExecError;

use std::collections::HashMap;

use crate::pixel::Pix;

pub mod except;
pub mod builtin;

#[derive(Debug, Clone)]
pub enum StackValue {
    String(String),
    Integer(i32),
    Float(f32),
    Size(i32, i32),
    Color(Pix<f32>),
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

    pub fn stack(&self) -> &[StackValue] {
        &self.stack
    }

    pub fn push(&mut self, val: StackValue) {
        self.stack.push(val);
    }

    pub fn push_int(&mut self, val: i32) {
        self.stack.push(StackValue::Integer(val));
    }

    pub fn push_float(&mut self, val: f32) {
        self.stack.push(StackValue::Float(val));
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
                ScriptToken::Color(valr, valg, valb) => {
                    let pix: Pix<f32> = Pix::new(*valr as f32, *valg as f32, *valb as f32);
                    self.push(StackValue::Color(pix));
                },
                ScriptToken::Name(val) => {
                    if let Some(heapval) = self.heap.get(val) {
                        self.push(heapval.clone());
                    }
                    else {
                        self.execute_builtin(val)?;
                    }
                },
                ScriptToken::StoreTo(val) => {
                    let stackval = self.stack.pop()
                        .ok_or_else(|| ExecError::new("stack underflow") )?;
                    self.heap.insert(val.to_string(), stackval);
                }
                other => {
                    let msg = format!("unknown token: {:?}", other);
                    return Err(ExecError::new(&msg))
                },
            }
        }

        Ok(())
    }
}
