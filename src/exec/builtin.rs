use std::rc::Rc;

use crate::pixel::Pix;
use crate::img::Img;
use crate::img::ppmio;
use crate::script::ScriptToken;
use crate::exec::StackValue;
use crate::exec::ExecContext;
use crate::exec::estack::LendStackIter;
use crate::exec::except::ExecError;
use crate::exec::util::elementwise;

#[derive(Debug, Clone)]
pub enum BuiltInSymbol {
    Dup,
    Pop,
    Swap,
    Eval,
    If,
    IfElse,
    Split,
    Size,
    Color,
    Image,
    Write,
    Read,
    OpAdd,
    OpSub,
    OpMul,
    OpDiv,
    Average,
    MapVal,
    Contrast,
    HalfShift,
    TileBy,
    Diamond,
    Holify,
    TaxiBlur,
}

impl ExecContext {
    pub fn search_builtin(&self, tok: &str) -> Option<BuiltInSymbol> {
        match tok {
            "dup" => Some(BuiltInSymbol::Dup),
            "pop" => Some(BuiltInSymbol::Pop),
            "swap" => Some(BuiltInSymbol::Swap),
            "eval" => Some(BuiltInSymbol::Eval),
            "if" => Some(BuiltInSymbol::If),
            "ifelse" => Some(BuiltInSymbol::IfElse),
            "split" => Some(BuiltInSymbol::Split),
            "size" => Some(BuiltInSymbol::Size),
            "color" => Some(BuiltInSymbol::Color),
            "image" => Some(BuiltInSymbol::Image),
            "write" => Some(BuiltInSymbol::Write),
            "read" => Some(BuiltInSymbol::Read),
            "+" => Some(BuiltInSymbol::OpAdd),
            "-" => Some(BuiltInSymbol::OpSub),
            "*" => Some(BuiltInSymbol::OpMul),
            "/" => Some(BuiltInSymbol::OpDiv),
            "average" => Some(BuiltInSymbol::Average),
            "mapval" => Some(BuiltInSymbol::MapVal),
            "contrast" => Some(BuiltInSymbol::Contrast),
            "halfshift" => Some(BuiltInSymbol::HalfShift),
            "tileby" => Some(BuiltInSymbol::TileBy),
            "diamond" => Some(BuiltInSymbol::Diamond),
            "holify" => Some(BuiltInSymbol::Holify),
            "taxiblur" => Some(BuiltInSymbol::TaxiBlur),
            _ => None,
        }
    }
    
    pub fn execute_builtin(&mut self, sym: BuiltInSymbol, execstack: &mut LendStackIter<ScriptToken>) -> Result<(), ExecError> {
        match sym {
        
            BuiltInSymbol::Dup => {
                let stackval = self.stack.last()
                    .ok_or_else(|| ExecError::new("stack underflow") )?;
                self.push(stackval.clone());
            },
            
            BuiltInSymbol::Pop => {
                let _ = self.pop("pop")?;
            },

            BuiltInSymbol::Swap => {
                let val1 = self.pop("swap")?;
                let val2 = self.pop("swap")?;
                self.push(val1);
                self.push(val2);
            },

            BuiltInSymbol::Eval => {
                let stackval = self.pop("eval")?;
                match stackval {
                    StackValue::Proc(proc) => {
                        execstack.push(&proc);
                    },
                    StackValue::String(val) => {
                        if let Some(heapval) = self.heap.get(&val) {
                            if let StackValue::Proc(proc) = heapval {
                                execstack.push(proc);
                            }
                            else {
                                self.push(heapval.clone());
                            }
                        }
                        else if let Some(symbol) = self.search_builtin(&val) {
                            self.execute_builtin(symbol, execstack)?;
                        }
                        else {
                            let msg = format!("symbol not known: {:?}", val);
                            return Err(ExecError::new(&msg));
                        }
                    },
                    _ => {
                        let msg = format!("cannot eval: {:?}", stackval);
                        return Err(ExecError::new(&msg));
                    }                    
                }
            },

            BuiltInSymbol::If => {
                let flag = self.pop_int("if")?;
                let val = self.pop("if")?;
                if flag != 0 {
                    if let StackValue::Proc(proc) = val {
                        execstack.push(&proc);
                    }
                    else {
                        self.push(val);
                    }
                }
            }
            
            BuiltInSymbol::IfElse => {
                let flag = self.pop_int("ifelse")?;
                let val2 = self.pop("ifelse")?;
                let val1 = self.pop("ifelse")?;
                if flag != 0 {
                    if let StackValue::Proc(proc) = val1 {
                        execstack.push(&proc);
                    }
                    else {
                        self.push(val1);
                    }
                }
                else {
                    if let StackValue::Proc(proc) = val2 {
                        execstack.push(&proc);
                    }
                    else {
                        self.push(val2);
                    }
                }
            }
            
            BuiltInSymbol::Split => {
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

            BuiltInSymbol::Size => {
                // INT INT size, IMAGE size, SIZE size
                let (width, height) = self.pop_as_size("size")?;
                self.push_size(width, height);
            },

            BuiltInSymbol::Color => {
                // NUM NUM NUM color, COLOR color
                let pix = self.pop_as_color("color")?;
                self.push_color(pix);
            },

            BuiltInSymbol::Image => {
                // SIZE COLOR image, INT INT COLOR image
                // SIZE NUM image, INT INT NUM image
                let color: Pix<f32>;
                
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

                let (width, height) = self.pop_as_size("image")?;
                
                if width <= 0 || height <= 0 {
                    let msg = format!("image size must be positive: {width}x{height}");
                    return Err(ExecError::new(&msg));
                }
                
                let img = Img::new_constant(width as usize, height as usize, color);
                self.push_img(img);
            },

            BuiltInSymbol::Write => {
                // IMG STR write
                let name: String = self.pop_str("write")?;
                let img: Rc<Img<f32>> = self.pop_img("write")?;
                ppmio::img_write(&name, img.as_u8())?;
            },
            
            BuiltInSymbol::Read => {
                // STR read
                let name: String = self.pop_str("read")?;
                let inimg = ppmio::img_read(&name)?;
                self.push_img(inimg.as_f32());
            },

            BuiltInSymbol::OpAdd => {
                let varg2 = self.pop("+")?;
                let varg1 = self.pop("+")?;
                let stackval = elementwise(varg1, varg2, |v1, v2| v1+v2)?;
                self.push(stackval);
            },

            BuiltInSymbol::OpSub => {
                let varg2 = self.pop("-")?;
                let varg1 = self.pop("-")?;
                let stackval = elementwise(varg1, varg2, |v1, v2| v1-v2)?;
                self.push(stackval);
            },

            BuiltInSymbol::OpMul => {
                let varg2 = self.pop("*")?;
                let varg1 = self.pop("*")?;
                let stackval = elementwise(varg1, varg2, |v1, v2| v1*v2)?;
                self.push(stackval);
            },

            BuiltInSymbol::OpDiv => {
                let varg2 = self.pop("/")?;
                let varg1 = self.pop("/")?;
                let stackval = elementwise(varg1, varg2, |v1, v2| v1/v2)?;
                self.push(stackval);
            },

            BuiltInSymbol::Average => {
                // IMG contrast
                let img: Rc<Img<f32>> = self.pop_img("average")?;
                let pix = img.average();
                self.push_color(pix);
            },

            BuiltInSymbol::MapVal => {
                // IMG PROC mapval
                let proc = self.pop_proc("mapval")?;
                let img: Rc<Img<f32>> = self.pop_img("mapval")?;
                
                let mut subctx = self.clone_env();
                let mut subexecstack: LendStackIter<ScriptToken> = LendStackIter::new();
                
                let res = img.map_val_mut(|val: &f32| {
                    if let Err(_) = subctx.execute_proc(&proc, &mut subexecstack, StackValue::Float(*val)) {
                        0.0
                    }
                    else {
                        if let Ok(fval) = subctx.pop_float("mapval") {
                            fval
                        }
                        else {
                            0.0
                        }
                    }
                });
                self.push_img(res);
            },

            BuiltInSymbol::Contrast => {
                // IMG NUM contrast
                let val = self.pop_as_float("contrast")?;
                let img: Rc<Img<f32>> = self.pop_img("contrast")?;
                let res = img.contrast(val);
                self.push_img(res);
            },

            BuiltInSymbol::HalfShift => {
                let img: Rc<Img<f32>> = self.pop_img("halfshift")?;
                let res = img.halfshift();
                self.push_img(res);
            },

            BuiltInSymbol::TileBy => {
                // IMG SIZE tileby, IMG NUM NUM tileby
                let (width, height) = self.pop_as_size("tileby")?;
                let img: Rc<Img<f32>> = self.pop_img("tileby")?;
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

            BuiltInSymbol::Diamond => {
                // SIZE diamond, etc
                let (width, height) = self.pop_as_size("diamond")?;
                let (uwidth, uheight) = (width as usize, height as usize);
                let res : Img<f32> = Img::diamond(uwidth, uheight);
                self.push_img(res);
            },

            BuiltInSymbol::Holify => {
                // IMG NUM holify
                let val = self.pop_as_float("holify")?;
                let img: Rc<Img<f32>> = self.pop_img("holify")?;
                let res = img.holify(val);
                self.push_img(res);
            },
            
            BuiltInSymbol::TaxiBlur => {
                // IMG INT taxiblur
                let val = self.pop_int("taxiblur")?;
                let img: Rc<Img<f32>> = self.pop_img("taxiblur")?;
                let res = img.taxiblur(val);
                self.push_img(res);
            },
        }
        
        Ok(())
    }
}
