use std::rc::Rc;

use crate::img::pixel::Pix;
use crate::img::Img;
use crate::exec::StackValue;
use crate::exec::except::ExecError;

pub fn elementwise<F>(arg: StackValue, func: F) -> Result<StackValue, ExecError>
    where F: Fn(&f32) -> f32 {
    
    match arg {
        StackValue::Integer(ival) => {
            Ok(StackValue::Float(func(&(ival as f32))))
        },
        StackValue::Float(fval) => {
            Ok(StackValue::Float(func(&fval)))
        },
        StackValue::Color(pval) => {
            let res: Pix<f32> = Pix::new(func(&pval.r), func(&pval.g), func(&pval.b));
            Ok(StackValue::Color(res))
        },
        StackValue::Image(img) => {
            let res = img.map_val(func);
            Ok(StackValue::Image(Rc::new(res)))
        },
        _ => {
            let msg = format!("no arithmetic operation: {:?}", arg);
            Err(ExecError::new(&msg))
        }
    }
}

pub fn elementwise_bool<F>(arg: StackValue, func: F) -> Result<StackValue, ExecError>
    where F: Fn(&f32) -> bool {
    
    match arg {
        StackValue::Integer(ival) => {
            Ok(StackValue::Integer(if func(&(ival as f32)) {1} else {0}))
        },
        StackValue::Float(fval) => {
            Ok(StackValue::Integer(if func(&fval) {1} else {0}))
        },
        StackValue::Color(pval) => {
            let res: Pix<f32> = Pix::new(
                if func(&pval.r) {1.0} else {0.0},
                if func(&pval.g) {1.0} else {0.0},
                if func(&pval.b) {1.0} else {0.0});
            Ok(StackValue::Color(res))
        },
        StackValue::Image(img) => {
            let res = img.map_val(|val| if func(val) {1.0} else {0.0} );
            Ok(StackValue::Image(Rc::new(res)))
        },
        _ => {
            let msg = format!("no arithmetic operation: {:?}", arg);
            Err(ExecError::new(&msg))
        }
    }
}

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

pub fn elementwise_bool_2<F>(varg1: StackValue, varg2: StackValue, func: F) -> Result<StackValue, ExecError>
    where F: Fn(&f32, &f32) -> bool {
    
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
            Ok(StackValue::Integer(if func(&f1, &f2) {1} else {0} ))
        },
        (StackValue::Color(p1), StackValue::Color(p2)) => {
            let res: Pix<f32> = Pix::new(
                if func(&p1.r, &p2.r) {1.0} else {0.0},
                if func(&p1.g, &p2.g) {1.0} else {0.0},
                if func(&p1.b, &p2.b) {1.0} else {0.0});
            Ok(StackValue::Color(res))
        },
        (StackValue::Image(img1), StackValue::Image(img2)) => {
            if img1.size() != img2.size() {
                let msg = format!("image sizes do not match: {:?} {:?}", img1, img2);
                return Err(ExecError::new(&msg));
            }
            let res = img1.combine_val(&img2, |xp, yp| if func(xp, yp) {1.0} else {0.0});
            Ok(StackValue::Image(Rc::new(res)))
        },
        (StackValue::Color(pix), StackValue::Float(fl)) => {
            let res: Pix<f32> = Pix::new(
                if func(&pix.r, &fl) {1.0} else {0.0},
                if func(&pix.g, &fl) {1.0} else {0.0},
                if func(&pix.b, &fl) {1.0} else {0.0});
            Ok(StackValue::Color(res))
        },
        (StackValue::Float(fl), StackValue::Color(pix)) => {
            let res: Pix<f32> = Pix::new(
                if func(&fl, &pix.r) {1.0} else {0.0},
                if func(&fl, &pix.g) {1.0} else {0.0},
                if func(&fl, &pix.b) {1.0} else {0.0});
            Ok(StackValue::Color(res))
        },
        (StackValue::Image(img), StackValue::Float(fl)) => {
            let res = img.map_val(|val| if func(val, &fl) {1.0} else {0.0});
            Ok(StackValue::Image(Rc::new(res)))
        },
        (StackValue::Float(fl), StackValue::Image(img)) => {
            let res = img.map_val(|val| if func(&fl, val) {1.0} else {0.0});
            Ok(StackValue::Image(Rc::new(res)))
        },
        (StackValue::Image(img), StackValue::Color(pix)) => {
            let res = img.map(|val| Pix::new(
                if func(&val.r, &pix.r) {1.0} else {0.0},
                if func(&val.g, &pix.g) {1.0} else {0.0},
                if func(&val.b, &pix.b) {1.0} else {0.0}));
            Ok(StackValue::Image(Rc::new(res)))
        },
        (StackValue::Color(pix), StackValue::Image(img)) => {
            let res = img.map(|val| Pix::new(
                if func(&pix.r, &val.r) {1.0} else {0.0},
                if func(&pix.g, &val.g) {1.0} else {0.0},
                if func(&pix.b, &val.b) {1.0} else {0.0}));
            Ok(StackValue::Image(Rc::new(res)))
        },
        (xarg1, xarg2) => {
            let msg = format!("no arithmetic operation: {:?} {:?}", xarg1, xarg2);
            Err(ExecError::new(&msg))
        }
    }

}

pub fn sigmoid(val: f32, sharp: f32) -> f32 {
    1.0 / (1.0 + (-sharp*(2.0*val-1.0)).exp())
}
