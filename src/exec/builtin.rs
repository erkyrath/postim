use crate::exec::StackValue;
use crate::exec::ExecContext;
use crate::exec::except::ExecError;

use crate::pixel::Pix;
use crate::img::Img;

impl ExecContext {
    pub fn execute_builtin(&mut self, tok: &str) -> Result<(), ExecError> {
        match tok {
        
            "dup" => {
                let stackval = self.stack.last()
                    .ok_or_else(|| ExecError::new("stack underflow") )?;
                self.push(stackval.clone());
            },
            
            "pop" => {
                let _ = self.pop("pop")?;
            },

            "split" => {
                // COLOR split, SIZE split
                let stackval = self.pop("split")?;
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

            "image" => {
                // SIZE COLOR image, INT INT COLOR image
                // SIZE NUM image, INT INT NUM image
                let color: Pix<f32>;
                let width: i32;
                let height: i32;
                
                let colorval = self.pop("image")?;
                match colorval {
                    StackValue::Color(pix) => {
                        color = pix;
                    },
                    StackValue::Integer(ival) => {
                        color = Pix::grey(ival as f32);
                    },
                    StackValue::Float(fval) => {
                        color = Pix::grey(fval);
                    },
                    _ => {
                        let msg = format!("image needs color or num: {:?}", colorval);
                        return Err(ExecError::new(&msg));
                    }
                }

                let sizeval = self.pop("image")?;
                match sizeval {
                    StackValue::Size(wval, hval) => {
                        (width, height) = (wval, hval);
                    },
                    StackValue::Integer(ival) => {
                        width = ival;
                        let heightval = self.pop("image")?;
                        if let StackValue::Integer(jval) = heightval {
                            height = jval;
                        }
                        else {
                            let msg = format!("image needs size or int int: {:?}", heightval);
                            return Err(ExecError::new(&msg));
                        }
                    }
                    _ => {
                        let msg = format!("image needs size or int int: {:?}", sizeval);
                        return Err(ExecError::new(&msg));
                    }
                }
                
                if width <= 0 || height <= 0 {
                    let msg = format!("image size must be positive: {width}x{height}");
                    return Err(ExecError::new(&msg));
                }
                
                let img = Img::new_constant(width as usize, height as usize, color);
                self.push_img(img);
            },
            
            _ => {
                let msg = format!("name not known: {:?}", tok);
                return Err(ExecError::new(&msg));
            },
        }
        
        Ok(())
    }
}
