use crate::exec::StackValue;
use crate::exec::ExecContext;
use crate::exec::except::ExecError;

impl ExecContext {
    pub fn execute_builtin(&mut self, tok: &str) -> Result<(), ExecError> {
        match tok {
        
            "dup" => {
                let stackval = self.stack.last()
                    .ok_or_else(|| ExecError::new("stack underflow") )?;
                self.push(stackval.clone());
            },
            
            "pop" => {
                let _ = self.stack.pop()
                    .ok_or_else(|| ExecError::new("stack underflow") )?;
            },

            "split" => {
                let stackval = self.stack.pop()
                    .ok_or_else(|| ExecError::new("stack underflow") )?;
                match stackval {
                    StackValue::Size(xval, yval) => {
                        self.push_int(xval);
                        self.push_int(yval);
                    }
                    StackValue::Color(pix) => {
                        self.push_float(pix.r);
                        self.push_float(pix.g);
                        self.push_float(pix.b);
                    }
                    _ => {
                        let msg = format!("cannot split: {:?}", stackval);
                        return Err(ExecError::new(&msg));
                    }
                }
            },
            
            _ => {
                let msg = format!("name not known: {:?}", tok);
                return Err(ExecError::new(&msg));
            },
        }
        
        Ok(())
    }
}
