use std::rc::Rc;

use crate::pixel::Pix;
use crate::img::Img;
use crate::exec::StackValue;
use crate::exec::except::ExecError;

pub fn elementwise_2<F>(varg1: StackValue, varg2: StackValue, func: F) -> Result<StackValue, ExecError>
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
            Ok(StackValue::Float(func(&f1, &f2)))
        },
        (StackValue::Color(p1), StackValue::Color(p2)) => {
            let res: Pix<f32> = Pix::new(func(&p1.r, &p2.r), func(&p1.g, &p2.g), func(&p1.b, &p2.b));
            Ok(StackValue::Color(res))
        },
        (StackValue::Image(img1), StackValue::Image(img2)) => {
            if img1.size() != img2.size() {
                let msg = format!("image sizes do not match: {:?} {:?}", img1, img2);
                return Err(ExecError::new(&msg));
            }
            let res = img1.combine_val(&img2, func);
            Ok(StackValue::Image(Rc::new(res)))
        },
        (StackValue::Color(pix), StackValue::Float(fl)) => {
            let res: Pix<f32> = Pix::new(func(&pix.r, &fl), func(&pix.g, &fl), func(&pix.b, &fl));
            Ok(StackValue::Color(res))
        },
        (StackValue::Float(fl), StackValue::Color(pix)) => {
            let res: Pix<f32> = Pix::new(func(&fl, &pix.r), func(&fl, &pix.g), func(&fl, &pix.b));
            Ok(StackValue::Color(res))
        },
        (StackValue::Image(img), StackValue::Float(fl)) => {
            let res = img.map_val(|val| func(val, &fl));
            Ok(StackValue::Image(Rc::new(res)))
        },
        (StackValue::Float(fl), StackValue::Image(img)) => {
            let res = img.map_val(|val| func(&fl, val));
            Ok(StackValue::Image(Rc::new(res)))
        },
        (StackValue::Image(img), StackValue::Color(pix)) => {
            let res = img.map(|val| Pix::new(func(&val.r, &pix.r), func(&val.g, &pix.g), func(&val.b, &pix.b)));
            Ok(StackValue::Image(Rc::new(res)))
        },
        (StackValue::Color(pix), StackValue::Image(img)) => {
            let res = img.map(|val| Pix::new(func(&pix.r, &val.r), func(&pix.g, &val.g), func(&pix.b, &val.b)));
            Ok(StackValue::Image(Rc::new(res)))
        },
        (xarg1, xarg2) => {
            let msg = format!("no arithmetic operation: {:?} {:?}", xarg1, xarg2);
            Err(ExecError::new(&msg))
        }
    }

}

