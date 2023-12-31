use std::rc::Rc;
use rand::Rng;
use rand::rngs::SmallRng;
use rand::seq::SliceRandom;

use crate::img::pixel::Pix;
use crate::img::Img;
use crate::img::ppmio;
use crate::script::ScriptToken;
use crate::script::parse::load_script_file;
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
    Mark,
    Array,
    Dup,
    Pop,
    Swap,
    Eval,
    If,
    IfElse,
    Cond,
    Break,
    Random,
    Split,
    Size,
    Color,
    Image,
    Write,
    Read,
    Run,
    IsNan,
    Pi,
    Sin,
    Cos,
    Tan,
    ASin,
    ACos,
    ATan,
    OpOr,
    OpAnd,
    OpLT,
    OpGT,
    OpLTE,
    OpGTE,
    OpNeg,
    OpAdd,
    OpSub,
    OpMul,
    OpDiv,
    OpMod,
    Hypot,
    Shade,
    Sigmoid,
    Average,
    Map,
    MapVal,
    Project,
    ProjectMap,
    Interpolate,
    At,
    NAt,
    Contrast,
    Shift,
    HalfShift,
    TileBy,
    Concat,
    Diamond,
    Holify,
    TaxiBlur,
    Seamless,
}

impl ExecContext {
    pub fn search_builtin(&self, tok: &str) -> Option<BuiltInSymbol> {
        match tok {
            "[" => Some(BuiltInSymbol::Mark),
            "]" => Some(BuiltInSymbol::Array),
            "dup" => Some(BuiltInSymbol::Dup),
            "pop" => Some(BuiltInSymbol::Pop),
            "swap" => Some(BuiltInSymbol::Swap),
            "eval" => Some(BuiltInSymbol::Eval),
            "if" => Some(BuiltInSymbol::If),
            "ifelse" => Some(BuiltInSymbol::IfElse),
            "cond" => Some(BuiltInSymbol::Cond),
            "break" => Some(BuiltInSymbol::Break),
            "random" => Some(BuiltInSymbol::Random),
            "split" => Some(BuiltInSymbol::Split),
            "size" => Some(BuiltInSymbol::Size),
            "color" => Some(BuiltInSymbol::Color),
            "image" => Some(BuiltInSymbol::Image),
            "write" => Some(BuiltInSymbol::Write),
            "read" => Some(BuiltInSymbol::Read),
            "run" => Some(BuiltInSymbol::Run),
            "isnan" => Some(BuiltInSymbol::IsNan),
            "pi" => Some(BuiltInSymbol::Pi),
            "sin" => Some(BuiltInSymbol::Sin),
            "cos" => Some(BuiltInSymbol::Cos),
            "tan" => Some(BuiltInSymbol::Tan),
            "asin" => Some(BuiltInSymbol::ASin),
            "acos" => Some(BuiltInSymbol::ACos),
            "atan" => Some(BuiltInSymbol::ATan),
            "or" => Some(BuiltInSymbol::OpOr),
            "and" => Some(BuiltInSymbol::OpAnd),
            "<" => Some(BuiltInSymbol::OpLT),
            ">" => Some(BuiltInSymbol::OpGT),
            "<=" => Some(BuiltInSymbol::OpLTE),
            ">=" => Some(BuiltInSymbol::OpGTE),
            "neg" => Some(BuiltInSymbol::OpNeg),
            "+" => Some(BuiltInSymbol::OpAdd),
            "-" => Some(BuiltInSymbol::OpSub),
            "*" => Some(BuiltInSymbol::OpMul),
            "/" => Some(BuiltInSymbol::OpDiv),
            "%" => Some(BuiltInSymbol::OpMod),
            "hypot" => Some(BuiltInSymbol::Hypot),
            "shade" => Some(BuiltInSymbol::Shade),
            "sigmoid" => Some(BuiltInSymbol::Sigmoid),
            "average" => Some(BuiltInSymbol::Average),
            "map" => Some(BuiltInSymbol::Map),
            "mapval" => Some(BuiltInSymbol::MapVal),
            "project" => Some(BuiltInSymbol::Project),
            "projectmap" => Some(BuiltInSymbol::ProjectMap),
            "at" => Some(BuiltInSymbol::At),
            "nat" => Some(BuiltInSymbol::NAt),
            "interpolate" => Some(BuiltInSymbol::Interpolate),
            "contrast" => Some(BuiltInSymbol::Contrast),
            "shift" => Some(BuiltInSymbol::Shift),
            "halfshift" => Some(BuiltInSymbol::HalfShift),
            "tileby" => Some(BuiltInSymbol::TileBy),
            "concat" => Some(BuiltInSymbol::Concat),
            "diamond" => Some(BuiltInSymbol::Diamond),
            "holify" => Some(BuiltInSymbol::Holify),
            "taxiblur" => Some(BuiltInSymbol::TaxiBlur),
            "seamless" => Some(BuiltInSymbol::Seamless),
            _ => None,
        }
    }
    
    pub fn execute_builtin(&mut self, sym: BuiltInSymbol, execstack: &mut LendStackIter<ScriptToken>) -> Result<(), ExecError> {
        match sym {
        
            BuiltInSymbol::Mark => {
                self.push(StackValue::Mark);
            },
            
            BuiltInSymbol::Array => {
                let pos = self.stack.iter().rposition(|val| match val {
                    StackValue::Mark => true,
                    _ => false,
                })
                    .ok_or_else(|| ExecError::new("no array mark on stack") )?;
                let tail = self.stack.split_off(pos+1);
                let _ = self.pop("array")?;
                self.push_array(tail);
            },
            
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
            },

            BuiltInSymbol::Cond => {
                // [ PROC1 FLAG1 PROC2 FLAG2 ... ] cond
                // [ PROC1 FLAG1 PROC2 FLAG2 ... PROCELSE ] cond
                let arr = self.pop_array("cond")?;
                let mut res: Option<&StackValue> = None;
                let mut index = 0;
                while index+1 < arr.len() {
                    if let StackValue::Integer(ival) = arr[index+1] {
                        if ival != 0 {
                            res = Some(&arr[index]);
                            break;
                        }
                    }
                    else {
                        let msg = format!("cond entry needs int: {:?}", arr[index]);
                        return Err(ExecError::new(&msg));
                    }

                    index += 2;
                }
                if res.is_none() && index < arr.len() {
                    res = Some(&arr[index]);
                }
                if let Some(val) = res {
                    if let StackValue::Proc(proc) = val {
                        execstack.push(&proc);
                    }
                    else {
                        self.push(val.clone());
                    }
                }
            },
            
            BuiltInSymbol::Break => {
                execstack.pop();
            },
            
            BuiltInSymbol::Random => {
                let stackval = self.pop("random")?;
                match stackval {
                    StackValue::Integer(ival) => {
                        if ival <= 0 {
                            let msg = format!("random integer range must be positive: {ival}");
                            return Err(ExecError::new(&msg));
                        }
                        let res: i32 = {
                            let mut rng = self.rng.borrow_mut();
                            rng.gen_range(0..ival)
                        };
                        self.push_int(res);
                    },
                    StackValue::Float(fval) => {
                        if fval <= 0.0 {
                            let msg = format!("random float range must be positive: {fval}");
                            return Err(ExecError::new(&msg));
                        }
                        let res: f32 = {
                            let mut rng = self.rng.borrow_mut();
                            rng.gen_range(0.0..fval)
                        };
                        self.push_float(res);
                    },
                    StackValue::Array(arr) => {
                        let res = {
                            let mut rng = self.rng.borrow_mut();
                            arr.choose::<SmallRng>(&mut rng)
                        }.ok_or_else(|| ExecError::new("random array must be nonempty") )?;
                        self.push(res.clone());
                    }
                    _ => {
                        let msg = format!("cannot random: {:?}", stackval);
                        return Err(ExecError::new(&msg));
                    }
                }
            },
            
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

            BuiltInSymbol::Run => {
                // STR read
                let name: String = self.pop_str("read")?;
                let script = load_script_file(&name)?;
                execstack.push(&script.tokens());
            },

            BuiltInSymbol::IsNan => {
                let varg = self.pop("isnan")?;
                let stackval = elementwise_bool(varg, |val| val.is_nan())?;
                self.push(stackval);                
            },

            BuiltInSymbol::Pi => {
                self.push_float(std::f32::consts::PI);
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

            BuiltInSymbol::OpOr => {
                let flag2 = self.pop_int("or")?;
                let flag1 = self.pop_int("or")?;
                let res = flag1 != 0 || flag2 != 0;
                self.push_int(res as i32);
            },

            BuiltInSymbol::OpAnd => {
                let flag2 = self.pop_int("and")?;
                let flag1 = self.pop_int("and")?;
                let res = flag1 != 0 && flag2 != 0;
                self.push_int(res as i32);
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

            BuiltInSymbol::OpNeg => {
                let varg = self.pop("acos")?;
                let stackval = elementwise(varg, |val| -val)?;
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

            BuiltInSymbol::OpMod => {
                let varg2 = self.pop("%")?;
                let varg1 = self.pop("%")?;
                let stackval = elementwise_2(varg1, varg2, |v1, v2| v1%v2)?;
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
                //### get a SIZE in there?
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

            BuiltInSymbol::At => {
                // IMG NUM NUM at
                let ypos = self.pop_as_float("at")?;
                let xpos = self.pop_as_float("at")?;
                let img: Rc<Img<f32>> = self.pop_img("at")?;
                let res = img.at_lerp(xpos, ypos);
                self.push_color(res);
            },
            
            BuiltInSymbol::NAt => {
                // IMG NUM NUM nat
                let ypos = self.pop_as_float("nat")?;
                let xpos = self.pop_as_float("nat")?;
                let img: Rc<Img<f32>> = self.pop_img("nat")?;
                let res = img.at_lerp(xpos * img.width as f32, ypos * img.height as f32);
                self.push_color(res);
            },
            
            BuiltInSymbol::Interpolate => {
                // IMG1 IMG2 IMGMASK interpolate
                //### or IMG1 IMG2 PROC interpolate?
                let imgmask: Rc<Img<f32>> = self.pop_img("interpolate")?;
                let img2: Rc<Img<f32>> = self.pop_img("interpolate")?;
                let img1: Rc<Img<f32>> = self.pop_img("interpolate")?;
                let res = img1.interp_mask(&img2, &imgmask);
                self.push_img(res);
            }

            BuiltInSymbol::Contrast => {
                // IMG NUM contrast
                let val = self.pop_as_float("contrast")?;
                let img: Rc<Img<f32>> = self.pop_img("contrast")?;
                let res = img.contrast(val);
                self.push_img(res);
            },

            BuiltInSymbol::Shift => {
                // IMG SIZE shift, IMG NUM NUM shift
                let (width, height) = self.pop_as_size("shift")?;
                let img: Rc<Img<f32>> = self.pop_img("shift")?;
                let res = img.shift(width, height);
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

            BuiltInSymbol::Concat => {
                // IMG... SIZE tilecat, IMG... NUM NUM tilecat
                let (width, height) = self.pop_as_size("concat")?;
                if width <= 0 || height <= 0 {
                    let msg = format!("tilecat size must be positive: {width}x{height}");
                    return Err(ExecError::new(&msg));
                }
                let imgls: Vec<Rc<Img<f32>>> = (0..width*height)
                    .map(|_| { self.pop_img("concat") })
                    .collect::<Result<Vec<_>, _>>()?;
                let (cellwidth, cellheight) = imgls[0].size();
                for img in &imgls {
                    if img.width != cellwidth || img.height != cellheight {
                        let msg = format!("concat size does not match: {}x{} vs {}x{}", img.width, img.height, cellwidth, cellheight);
                        return Err(ExecError::new(&msg));
                    }
                }
                let (totalwidth, totalheight) = (cellwidth * width as usize, cellheight * height as usize);
                if totalwidth >= 0x10000 || totalheight > 0x10000 {
                    let msg = format!("concat size is too large: {totalwidth}x{totalheight}");
                    return Err(ExecError::new(&msg));
                }
                let mut res : Img<f32> = Img::new(totalwidth, totalheight);
                for (index, img) in imgls.iter().enumerate() {
                    let row = index / width as usize;
                    let col = index - (row * width as usize);
                    for jx in 0..img.height {
                        for ix in 0..img.width {
                            let pix = img.at(ix, jx);
                            res.set(col*cellwidth+ix, row*cellheight+jx, pix.clone());
                        }
                    }
                }
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
