use std::rc::Rc;

use crate::pixel::Pix;
use crate::img::Img;
use crate::exec::StackValue;
use crate::exec::except::ExecError;

pub fn elementwise<F>(varg1: StackValue, varg2: StackValue, func: F) -> Result<StackValue, ExecError>
    where F: Fn(&f32, &f32) -> f32 {
    
    let arg2 = if let StackValue::Integer(ival) = varg2 {
        StackValue::Float(ival as f32)
    }
    else {
        varg2
    };
    let arg1 = if let StackValue::Integer(ival) = varg1 {
        StackValue::Float(ival as f32)
    }
    else {
        varg1
    };
    
    match (arg1, arg2) {
        (StackValue::Float(f1), StackValue::Float(f2)) => {
            Ok(StackValue::Float(f1 * f2))
        },
        (StackValue::Color(p1), StackValue::Color(p2)) => {
            let res: Pix<f32> = Pix::new(p1.r * p2.r, p1.g * p2.g, p1.b * p2.b);
            Ok(StackValue::Color(res))
        },
        (StackValue::Image(img1), StackValue::Image(img2)) => {
            let res = img1.combine_val(&img2, |v1, v2| v1*v2);
            Ok(StackValue::Image(Rc::new(res)))
        },
        (StackValue::Color(pix), StackValue::Float(fl)) => {
            let res: Pix<f32> = Pix::new(pix.r * fl, pix.g * fl, pix.b * fl);
            Ok(StackValue::Color(res))
        },
        (StackValue::Float(fl), StackValue::Color(pix)) => {
            let res: Pix<f32> = Pix::new(pix.r * fl, pix.g * fl, pix.b * fl);
            Ok(StackValue::Color(res))
        },
        (StackValue::Image(img), StackValue::Float(fl)) => {
            let res = img.map_val(|val| val*fl);
            Ok(StackValue::Image(Rc::new(res)))
        },
        (StackValue::Float(fl), StackValue::Image(img)) => {
            let res = img.map_val(|val| val*fl);
            Ok(StackValue::Image(Rc::new(res)))
        },
        (StackValue::Image(img), StackValue::Color(pix)) => {
            let res = img.map(|val| Pix::new(val.r*pix.r, val.g*pix.g, val.b*pix.b));
            Ok(StackValue::Image(Rc::new(res)))
        },
        (StackValue::Color(pix), StackValue::Image(img)) => {
            let res = img.map(|val| Pix::new(val.r*pix.r, val.g*pix.g, val.b*pix.b));
            Ok(StackValue::Image(Rc::new(res)))
        },
        (xarg1, xarg2) => {
            let msg = format!("cannot multiply: {:?} {:?}", xarg1, xarg2);
            Err(ExecError::new(&msg))
        }
    }

}

