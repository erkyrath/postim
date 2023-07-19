use std::rc::Rc;

use crate::img::pixel::Pix;
use crate::img::Img;
use crate::img::ppmio;
use crate::script::ScriptToken;
use crate::exec::StackValue;
use crate::exec::ExecContext;
use crate::exec::estack::LendStackIter;
use crate::exec::except::ExecError;
use crate::exec::util::elementwise;
use crate::exec::util::elementwise_bool;
use crate::exec::util::elementwise_2;
use crate::exec::util::elementwise_bool_2;
use crate::exec::util::sigmoid;

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
    IsNan,
    Sin,
    Cos,
    Tan,
    ASin,
    ACos,
    ATan,
    OpLT,
    OpGT,
    OpLTE,
    OpGTE,
    OpAdd,
    OpSub,
    OpMul,
    OpDiv,
    Hypot,
    Shade,
    Sigmoid,
    Average,
    Map,
    MapVal,
    Project,
    ProjectMap,
    Contrast,
    HalfShift,
    TileBy,
    Diamond,
    Holify,
    TaxiBlur,
    Seamless,
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
            "isnan" => Some(BuiltInSymbol::IsNan),
            "sin" => Some(BuiltInSymbol::Sin),
            "cos" => Some(BuiltInSymbol::Cos),
            "tan" => Some(BuiltInSymbol::Tan),
            "asin" => Some(BuiltInSymbol::ASin),
            "acos" => Some(BuiltInSymbol::ACos),
            "atan" => Some(BuiltInSymbol::ATan),
            "<" => Some(BuiltInSymbol::OpLT),
            ">" => Some(BuiltInSymbol::OpGT),
            "<=" => Some(BuiltInSymbol::OpLTE),
            ">=" => Some(BuiltInSymbol::OpGTE),
            "+" => Some(BuiltInSymbol::OpAdd),
            "-" => Some(BuiltInSymbol::OpSub),
            "*" => Some(BuiltInSymbol::OpMul),
            "/" => Some(BuiltInSymbol::OpDiv),
            "hypot" => Some(BuiltInSymbol::Hypot),
            "shade" => Some(BuiltInSymbol::Shade),
            "sigmoid" => Some(BuiltInSymbol::Sigmoid),
            "average" => Some(BuiltInSymbol::Average),
            "map" => Some(BuiltInSymbol::Map),
            "mapval" => Some(BuiltInSymbol::MapVal),
            "project" => Some(BuiltInSymbol::Project),
            "projectmap" => Some(BuiltInSymbol::ProjectMap),
            "contrast" => Some(BuiltInSymbol::Contrast),
            "halfshift" => Some(BuiltInSymbol::HalfShift),
            "tileby" => Some(BuiltInSymbol::TileBy),
            "diamond" => Some(BuiltInSymbol::Diamond),
            "holify" => Some(BuiltInSymbol::Holify),
            "taxiblur" => Some(BuiltInSymbol::TaxiBlur),
            "seamless" => Some(BuiltInSymbol::Seamless),
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
                // SIZE PROC image, INT INT PROC image
                let color: StackValue;
                
                let colorval = self.pop("image")?;
                match colorval {
                    StackValue::Color(pix) => {
                        color = StackValue::Color(pix);
                    },
                    StackValue::Integer(ival) => {
                        color = StackValue::Color(Pix::grey(ival as f32));
                    },
                    StackValue::Float(fval) => {
                        color = StackValue::Color(Pix::grey(fval));
                    },
                    StackValue::Proc(pval) => {
                        color = StackValue::Proc(pval);
                    },
                    _ => {
                        let msg = format!("image needs color, num, or proc: {:?}", colorval);
                        return Err(ExecError::new(&msg));
                    },
                }

                let (width, height) = self.pop_as_size("image")?;
                
                if width <= 0 || height <= 0 {
                    let msg = format!("image size must be positive: {width}x{height}");
                    return Err(ExecError::new(&msg));
                }

                let img: Img<f32>;
                match color {
                    StackValue::Color(pix) => {
                        img = Img::new_constant(width as usize, height as usize, pix);
                    },
                    StackValue::Proc(proc) => {
                        let mut subctx = self.clone_env();
                        let mut subexecstack: LendStackIter<ScriptToken> = LendStackIter::new();
                        img = Img::new_func_mut(width as usize, height as usize, |px, py| {
                            subctx.execute_proc_2(&proc, &mut subexecstack, StackValue::Float(px), StackValue::Float(py))?;
                            let pval = subctx.pop_as_color("image proc")?;
                            Ok(pval)
                        })?;
                    },
                    _ => {
                        let msg = format!("should not have generated color: {:?}", color);
                        return Err(ExecError::new(&msg));
                    },
                }
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

            BuiltInSymbol::IsNan => {
                let varg = self.pop("isnan")?;
                let stackval = elementwise_bool(varg, |val| val.is_nan())?;
                self.push(stackval);                
            },

            BuiltInSymbol::Sin => {
                let varg = self.pop("sin")?;
                let stackval = elementwise(varg, |val| val.sin())?;
                self.push(stackval);                
            },

            BuiltInSymbol::Cos => {
                let varg = self.pop("cos")?;
                let stackval = elementwise(varg, |val| val.cos())?;
                self.push(stackval);                
            },

            BuiltInSymbol::Tan => {
                let varg = self.pop("tan")?;
                let stackval = elementwise(varg, |val| val.tan())?;
                self.push(stackval);                
            },

            BuiltInSymbol::ASin => {
                let varg = self.pop("asin")?;
                let stackval = elementwise(varg, |val| val.asin())?;
                self.push(stackval);                
            },

            BuiltInSymbol::ACos => {
                let varg = self.pop("acos")?;
                let stackval = elementwise(varg, |val| val.acos())?;
                self.push(stackval);                
            },

            BuiltInSymbol::ATan => {
                let varg = self.pop("atan")?;
                let stackval = elementwise(varg, |val| val.atan())?;
                self.push(stackval);
            },

            BuiltInSymbol::OpLT => {
                let varg2 = self.pop("<")?;
                let varg1 = self.pop("<")?;
                let stackval = elementwise_bool_2(varg1, varg2, |v1, v2| v1<v2)?;
                self.push(stackval);
            },

            BuiltInSymbol::OpGT => {
                let varg2 = self.pop(">")?;
                let varg1 = self.pop(">")?;
                let stackval = elementwise_bool_2(varg1, varg2, |v1, v2| v1>v2)?;
                self.push(stackval);
            },

            BuiltInSymbol::OpLTE => {
                let varg2 = self.pop("<=")?;
                let varg1 = self.pop("<=")?;
                let stackval = elementwise_bool_2(varg1, varg2, |v1, v2| v1<=v2)?;
                self.push(stackval);
            },

            BuiltInSymbol::OpGTE => {
                let varg2 = self.pop(">=")?;
                let varg1 = self.pop(">=")?;
                let stackval = elementwise_bool_2(varg1, varg2, |v1, v2| v1>=v2)?;
                self.push(stackval);
            },

            BuiltInSymbol::OpAdd => {
                let varg2 = self.pop("+")?;
                let varg1 = self.pop("+")?;
                let stackval = elementwise_2(varg1, varg2, |v1, v2| v1+v2)?;
                self.push(stackval);
            },

            BuiltInSymbol::OpSub => {
                let varg2 = self.pop("-")?;
                let varg1 = self.pop("-")?;
                let stackval = elementwise_2(varg1, varg2, |v1, v2| v1-v2)?;
                self.push(stackval);
            },

            BuiltInSymbol::OpMul => {
                let varg2 = self.pop("*")?;
                let varg1 = self.pop("*")?;
                let stackval = elementwise_2(varg1, varg2, |v1, v2| v1*v2)?;
                self.push(stackval);
            },

            BuiltInSymbol::OpDiv => {
                let varg2 = self.pop("/")?;
                let varg1 = self.pop("/")?;
                let stackval = elementwise_2(varg1, varg2, |v1, v2| v1/v2)?;
                self.push(stackval);
            },

            BuiltInSymbol::Hypot => {
                let varg2 = self.pop("hypot")?;
                let varg1 = self.pop("hypot")?;
                let stackval = elementwise_2(varg1, varg2, |v1, v2| v1.hypot(*v2))?;
                self.push(stackval);
            },

            BuiltInSymbol::Shade => {
                let varg2 = self.pop("shade")?;
                let varg1 = self.pop("shade")?;
                let stackval = elementwise_2(varg1, varg2, |v1, vshade| {
                    if vshade >= &0.0 {
                        (1.0-vshade) * v1 + (vshade) * 255.0
                    }
                    else {
                        (1.0+vshade) * v1
                    }
                })?;
                self.push(stackval);
            },

            BuiltInSymbol::Sigmoid => {
                let varg2 = self.pop("sigmoid")?;
                let varg1 = self.pop("sigmoid")?;
                let stackval = elementwise_2(varg1, varg2, |val, vsharp| {
                    1.0 / (1.0 + (-vsharp*(2.0*val-1.0)).exp())
                })?;
                self.push(stackval);
            },

            BuiltInSymbol::Average => {
                // IMG contrast
                let img: Rc<Img<f32>> = self.pop_img("average")?;
                let pix = img.average();
                self.push_color(pix);
            },

            BuiltInSymbol::Map => {
                // IMG PROC map
                let proc = self.pop_proc("map")?;
                let img: Rc<Img<f32>> = self.pop_img("map")?;
                
                let mut subctx = self.clone_env();
                let mut subexecstack: LendStackIter<ScriptToken> = LendStackIter::new();
                
                let res = img.map_mut(|val: &Pix<f32>| {
                    subctx.execute_proc(&proc, &mut subexecstack, StackValue::Color(val.clone()))?;
                    let pval = subctx.pop_as_color("map proc")?;
                    Ok(pval)
                })?;
                self.push_img(res);
            },

            BuiltInSymbol::MapVal => {
                // IMG PROC mapval
                let proc = self.pop_proc("mapval")?;
                let img: Rc<Img<f32>> = self.pop_img("mapval")?;
                
                let mut subctx = self.clone_env();
                let mut subexecstack: LendStackIter<ScriptToken> = LendStackIter::new();
                
                let res = img.map_val_mut(|val: &f32| {
                    subctx.execute_proc(&proc, &mut subexecstack, StackValue::Float(*val))?;
                    let fval = subctx.pop_as_float("mapval")?;
                    Ok(fval)
                })?;
                self.push_img(res);
            },

            BuiltInSymbol::Project => {
                // IMG PROC project
                //### or IMG IMG project?
                let proc = self.pop_proc("project")?;
                let img: Rc<Img<f32>> = self.pop_img("project")?;
                
                let mut subctx = self.clone_env();
                let mut subexecstack: LendStackIter<ScriptToken> = LendStackIter::new();
                
                let res = img.project_mut(|px, py| {
                    subctx.execute_proc_2(&proc, &mut subexecstack, StackValue::Float(px), StackValue::Float(py))?;
                    let yval = subctx.pop_as_float("project proc")?;
                    let xval = subctx.pop_as_float("project proc")?;
                    Ok((xval, yval))
                })?;
                self.push_img(res);
            },

            BuiltInSymbol::ProjectMap => {
                // IMG PROC PROC projectmap
                //### or IMG IMG PROC projectmap?
                let pixproc = self.pop_proc("projectmap")?;
                let locproc = self.pop_proc("projectmap")?;
                let img: Rc<Img<f32>> = self.pop_img("projectmap")?;
                
                let mut subctx = self.clone_env();
                let mut subexecstack: LendStackIter<ScriptToken> = LendStackIter::new();
                let res = Img::new_func_mut(img.width, img.height, |px, py| {
                    subctx.execute_proc_2(&locproc, &mut subexecstack, StackValue::Float(px * img.width as f32), StackValue::Float(py * img.height as f32))?;
                    let yval = subctx.pop_as_float("projectmap locproc")?;
                    let xval = subctx.pop_as_float("projectmap locproc")?;
                    let pix = img.at_lerp(xval, yval);
                    subctx.execute_proc(&pixproc, &mut subexecstack, StackValue::Color(pix.clone()))?;
                    let pval = subctx.pop_as_color("projectmap pixproc")?;
                    Ok(pval)
                })?;
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
            
            BuiltInSymbol::Seamless => {
                // IMG NUM seamless
                let val = self.pop_as_float("seamless")?;
                let img: Rc<Img<f32>> = self.pop_img("seamless")?;
                let imgmask = Img::diamond(img.width, img.height).map_val(|x| sigmoid(*x, val));
                let imgflip = img.halfshift();
                let res = img.interp_mask(&imgflip, &imgmask);
                self.push_img(res);
            }
        }
        
        Ok(())
    }
}
