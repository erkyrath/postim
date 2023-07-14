use std::rc::Rc;

use crate::pixel::Pix;
use crate::img::Img;
use crate::exec::except::ExecError;
use crate::exec::ExecContext;
use crate::exec::StackValue;

impl ExecContext {

    pub fn pop(&mut self, label: &str) -> Result<StackValue, ExecError> {
        let val = self.stack.pop()
            .ok_or_else(|| {
                let msg = format!("stack underflow in {}", label);
                ExecError::new(&msg)
            })?;
        Ok(val)
    }

    pub fn pop_int(&mut self, label: &str) -> Result<i32, ExecError> {
        let val = self.pop(label)?;
        
        if let StackValue::Integer(ival) = val {
            Ok(ival)
        }
        else {
            let msg = format!("{} needs str: {:?}", label, val);
            Err(ExecError::new(&msg))
        }
    }

    pub fn pop_float(&mut self, label: &str) -> Result<f32, ExecError> {
        let val = self.pop(label)?;
        
        if let StackValue::Float(fval) = val {
            Ok(fval)
        }
        else {
            let msg = format!("{} needs str: {:?}", label, val);
            Err(ExecError::new(&msg))
        }
    }

    pub fn pop_str(&mut self, label: &str) -> Result<String, ExecError> {
        let val = self.pop(label)?;
        
        if let StackValue::String(strval) = val {
            Ok(strval)
        }
        else {
            let msg = format!("{} needs str: {:?}", label, val);
            Err(ExecError::new(&msg))
        }
    }

    pub fn pop_img(&mut self, label: &str) -> Result<Rc<Img<f32>>, ExecError> {
        let val = self.pop(label)?;
        
        if let StackValue::Image(imgval) = val {
            Ok(imgval)
        }
        else {
            let msg = format!("{} needs image: {:?}", label, val);
            Err(ExecError::new(&msg))
        }
    }

    pub fn pop_as_float(&mut self, label: &str) -> Result<f32, ExecError> {
        let val = self.pop(label)?;

        match val {
            StackValue::Float(fval) => Ok(fval),
            StackValue::Integer(ival) => Ok(ival as f32),
            _ => {
                let msg = format!("{} needs num: {:?}", label, val);
                Err(ExecError::new(&msg))
            }
        }
    }

    pub fn pop_as_size(&mut self, label: &str) -> Result<(i32, i32), ExecError> {
        match &self.stack[..] {
            [.., StackValue::Image(img) ] => {
                let res = (img.width as i32, img.height as i32);
                self.pop(label)?;
                Ok(res)
            },
            [.., StackValue::Size(width, height) ] => {
                let res = (*width, *height);
                self.pop(label)?;
                Ok(res)
            },
            [.., StackValue::Integer(width), StackValue::Integer(height) ] => {
                let res = (*width, *height);
                self.pop(label)?;
                self.pop(label)?;
                Ok(res)
            },
            _ => {
                let msg = format!("{} needs size, img, or int int", label);
                Err(ExecError::new(&msg))
            }
        }
    }
    
    pub fn pop_as_color(&mut self, label: &str) -> Result<Pix<f32>, ExecError> {
        match &self.stack[..] {
            [.., StackValue::Color(pix) ] => {
                let res = pix.clone();
                self.pop(label)?;
                Ok(res)
            },
            [.., StackValue::Integer(rval), StackValue::Integer(gval), StackValue::Integer(bval) ] => {
                let res: Pix<f32> = Pix::new(*rval as f32, *gval as f32, *bval as f32);
                self.pop(label)?;
                self.pop(label)?;
                self.pop(label)?;
                Ok(res)
            },
            [.., StackValue::Float(rval), StackValue::Float(gval), StackValue::Float(bval) ] => {
                let res: Pix<f32> = Pix::new(*rval, *gval, *bval);
                self.pop(label)?;
                self.pop(label)?;
                self.pop(label)?;
                Ok(res)
            },
            _ => {
                let msg = format!("{} needs color or num num num", label);
                Err(ExecError::new(&msg))
            }
        }
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

    pub fn push_str(&mut self, val: String) {
        self.stack.push(StackValue::String(val));
    }

    pub fn push_size(&mut self, width: i32, height: i32) {
        self.stack.push(StackValue::Size(width, height));
    }

    pub fn push_color(&mut self, val: Pix<f32>) {
        self.stack.push(StackValue::Color(val));
    }

    pub fn push_colorv(&mut self, rval: f32, gval: f32, bval: f32) {
        let pix: Pix<f32> = Pix::new(rval, gval, bval);
        self.stack.push(StackValue::Color(pix));
    }

    pub fn push_img(&mut self, val: Img<f32>) {
        self.stack.push(StackValue::Image(Rc::new(val)));
    }

}
