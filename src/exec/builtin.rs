use std::rc::Rc;

use crate::pixel::Pix;
use crate::img::Img;
use crate::img::ppmio;
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
                let _ = self.pop(tok)?;
            },

            "swap" => {
                let val1 = self.pop(tok)?;
                let val2 = self.pop(tok)?;
                self.push(val1);
                self.push(val2);
            },

            "split" => {
                // COLOR split, SIZE split
                let stackval = self.pop(tok)?;
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

            "size" => {
                // INT INT size, IMAGE size, SIZE size
                let width: i32;
                let height: i32;
                let sizeval = self.pop(tok)?;
                match sizeval {
                    StackValue::Size(wval, hval) => {
                        (width, height) = (wval, hval);
                    },
                    StackValue::Image(img) => {
                        (width, height) = (img.width as i32, img.height as i32);
                    },
                    StackValue::Integer(ival) => {
                        height = ival;
                        let widthval = self.pop(tok)?;
                        if let StackValue::Integer(jval) = widthval {
                            width = jval;
                        }
                        else {
                            let msg = format!("size needs image or int int: {:?}", widthval);
                            return Err(ExecError::new(&msg));
                        }
                    }
                    _ => {
                        let msg = format!("image needs image or int int: {:?}", sizeval);
                        return Err(ExecError::new(&msg));
                    }
                }
                self.push_size(width, height);
            },

            "image" => {
                // SIZE COLOR image, INT INT COLOR image
                // SIZE NUM image, INT INT NUM image
                let color: Pix<f32>;
                let width: i32;
                let height: i32;
                
                let colorval = self.pop(tok)?;
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

                let sizeval = self.pop(tok)?;
                match sizeval {
                    StackValue::Size(wval, hval) => {
                        (width, height) = (wval, hval);
                    },
                    StackValue::Integer(ival) => {
                        height = ival;
                        let widthval = self.pop(tok)?;
                        if let StackValue::Integer(jval) = widthval {
                            width = jval;
                        }
                        else {
                            let msg = format!("image needs size or int int: {:?}", widthval);
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

            "write" => {
                // IMG STR write
                let name: String = self.pop_str(tok)?;
                let img: Rc<Img<f32>> = self.pop_img(tok)?;
                ppmio::img_write(&name, img.as_u8())?;
            },
            
            "read" => {
                // STR read
                let name: String = self.pop_str(tok)?;
                let inimg = ppmio::img_read(&name)?;
                self.push_img(inimg.as_f32());
            },

            "average" => {
                // IMG contrast
                let img: Rc<Img<f32>> = self.pop_img(tok)?;
                let pix = img.average();
                self.push_color(pix);
            }
            
            "contrast" => {
                // IMG NUM contrast
                let val = self.pop_as_float(tok)?;
                let img: Rc<Img<f32>> = self.pop_img(tok)?;
                let res = img.contrast(val);
                self.push_img(res);
            }
            
            _ => {
                let msg = format!("name not known: {:?}", tok);
                return Err(ExecError::new(&msg));
            },
        }
        
        Ok(())
    }
}
