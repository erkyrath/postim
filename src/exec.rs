use std::collections::HashMap;
use std::rc::Rc;

use crate::pixel::Pix;
use crate::img::Img;
use crate::script::Script;
use crate::script::ScriptToken;
use crate::exec::except::ExecError;
use crate::script::parse;
use crate::img::ppmio;

pub mod except;
pub mod pushpop;
pub mod builtin;
pub mod util;

#[derive(Debug, Clone)]
pub enum StackValue {
    String(String),
    Integer(i32),
    Float(f32),
    Size(i32, i32),
    Color(Pix<f32>),
    Image(Rc<Img<f32>>),
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

    pub fn loadargs(&mut self, args: &Vec<String>) -> Result<(), ExecError> {
        for arg in args {
            if let Ok(ival) = arg.parse::<i32>() {
                self.push_int(ival);
            }
            else if let Ok(fval) = arg.parse::<f32>() {
                self.push_float(fval);
            }
            else if let Some((rval, gval, bval)) = parse::match_color(arg) {
                self.push_colorv(rval as f32, gval as f32, bval as f32);
            }
            else if let Some((width, height)) = parse::match_size(arg) {
                self.push_size(width, height);
            }
            else {
                let u8img = ppmio::img_read(arg)?;
                self.push_img(u8img.as_f32());
                println!("read {} {}x{}", arg, u8img.width, u8img.height);
            }
        }
        
        Ok(())
    }

    pub fn unloadargs(&mut self, outs: &Vec<String>) -> Result<(), ExecError> {
        for out in outs {
            let img = self.pop_img("output")?;
            ppmio::img_write(out, img.as_u8())?;
            println!("wrote {} {}x{}", out, img.width, img.height);
        }

        Ok(())
    }

    pub fn execute(&mut self, script: &Script) -> Result<(), ExecError> {
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
