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
                let (width, height) = self.pop_as_size(tok)?;
                self.push_size(width, height);
            },

            "image" => {
                // SIZE COLOR image, INT INT COLOR image
                // SIZE NUM image, INT INT NUM image
                let color: Pix<f32>;
                
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

                let (width, height) = self.pop_as_size(tok)?;
                
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
            },
            
            "contrast" => {
                // IMG NUM contrast
                let val = self.pop_as_float(tok)?;
                let img: Rc<Img<f32>> = self.pop_img(tok)?;
                let res = img.contrast(val);
                self.push_img(res);
            },

            "halfshift" => {
                let img: Rc<Img<f32>> = self.pop_img(tok)?;
                let res = img.halfshift();
                self.push_img(res);
            },

            "tileby" => {
                // IMG SIZE tileby, IMG NUM NUM tileby
                let (width, height) = self.pop_as_size(tok)?;
                let img: Rc<Img<f32>> = self.pop_img(tok)?;
                if width <= 0 || height <= 0 {
                    let msg = format!("tileby size must be positive: {width}x{height}");
                    return Err(ExecError::new(&msg));
                }
                let (uwidth, uheight) = (width as usize, height as usize);
                if img.width * uwidth >= 0x10000 || img.height * uheight > 0x10000 {
                    let msg = format!("tileby size is too large: {width}x{height}");
                    return Err(ExecError::new(&msg));
                }
                let res = img.tile_by(uwidth, uheight);
                self.push_img(res);
            },

            "diamond" => {
                // SIZE diamond, etc
                let (width, height) = self.pop_as_size(tok)?;
                let (uwidth, uheight) = (width as usize, height as usize);
                let res : Img<f32> = Img::diamond(uwidth, uheight);
                self.push_img(res);
            },

            "holify" => {
                // IMG NUM holify
                let val = self.pop_as_float(tok)?;
                let img: Rc<Img<f32>> = self.pop_img(tok)?;
                let res = img.holify(val);
                self.push_img(res);
            },
            
            "taxiblur" => {
                // IMG INT taxiblur
                let val = self.pop_int(tok)?;
                let img: Rc<Img<f32>> = self.pop_img(tok)?;
                let res = img.taxiblur(val);
                self.push_img(res);
            },
            
            _ => {
                let msg = format!("name not known: {:?}", tok);
                return Err(ExecError::new(&msg));
            },
        }
        
        Ok(())
    }
}
