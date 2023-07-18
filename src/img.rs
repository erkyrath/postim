use std::fmt;

use crate::pixel::Pix;
use crate::exec::except::ExecError;

pub mod imgmath;
pub mod ppmio;

pub struct Img<T> {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Pix<T>>,
}

impl<T> fmt::Debug for Img<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<Img {}x{}>", self.width, self.height)
    }
}

impl<T: Copy> Img<T> {
    pub fn new_grey(width: usize, height: usize, val: T) -> Img<T> {
        let pix: Pix<T> = Pix { r:val, g:val, b:val };
        let res = Img {
            width,
            height,
            pixels: vec![pix; width*height],
        };

        res
    }
}

impl<T: Default + Clone> Img<T> {
    pub fn new(width: usize, height: usize) -> Img<T> {
        let pix: Pix<T> = Pix::default();
        let res = Img {
            width,
            height,
            pixels: vec![pix; width*height],
        };

        res
    }

    pub fn new_func<F>(width: usize, height: usize, mut func: F) -> Result<Img<T>, ExecError>
    where F: FnMut(f32, f32) -> Result<Pix<T>, ExecError> {
        let mut res = Img::new(width, height);

        for jx in 0..height {
            let jval = (jx as f32) / (height as f32);
            for ix in 0..width {
                let ival = (ix as f32) / (width as f32);
                res.set(ix, jx, func(ival, jval)?);
            }
        }
        
        Ok(res)
    }

    pub fn tile_by(&self, xcount: usize, ycount: usize) -> Img<T> {
        let mut res = Img::new(self.width*xcount, self.height*ycount);

        for jx in 0..res.height {
            for ix in 0..res.width {
                let pix = self.at(ix % self.width, jx % self.height);
                res.set(ix, jx, pix.clone());
            }
        }

        res
    }

}

impl<T: Clone> Img<T> {
    pub fn new_constant(width: usize, height: usize, pix: Pix<T>) -> Img<T> {
        let res = Img {
            width,
            height,
            pixels: vec![pix; width*height],
        };

        res
    }

    pub fn map_val<F>(&self, func: F) -> Img<T>
    where F: Fn(&T) -> T {
        let mut res = Img {
            width: self.width,
            height: self.height,
            pixels: Vec::with_capacity(self.pixcount()),
        };

        for val in &self.pixels {
            let pix = Pix { r:func(&val.r), g:func(&val.g), b:func(&val.b) };
            res.pixels.push(pix);
        }

        res
    }

    pub fn map_val_mut<F>(&self, mut func: F) -> Result<Img<T>, ExecError>
    where F: FnMut(&T) -> Result<T, ExecError> {
        let mut res = Img {
            width: self.width,
            height: self.height,
            pixels: Vec::with_capacity(self.pixcount()),
        };

        for val in &self.pixels {
            let pix = Pix { r:func(&val.r)?, g:func(&val.g)?, b:func(&val.b)? };
            res.pixels.push(pix);
        }

        Ok(res)
    }

    pub fn map<F>(&self, func: F) -> Img<T>
    where F: Fn(&Pix<T>) -> Pix<T> {
        let mut res = Img {
            width: self.width,
            height: self.height,
            pixels: Vec::with_capacity(self.pixcount()),
        };

        for val in &self.pixels {
            res.pixels.push(func(val));
        }

        res
    }

    pub fn map_mut<F>(&self, mut func: F) -> Result<Img<T>, ExecError>
    where F: FnMut(&Pix<T>) -> Result<Pix<T>, ExecError> {
        let mut res = Img {
            width: self.width,
            height: self.height,
            pixels: Vec::with_capacity(self.pixcount()),
        };

        for val in &self.pixels {
            res.pixels.push(func(val)?);
        }

        Ok(res)
    }

    pub fn combine<F>(&self, other: &Img<T>, func: F) -> Img<T>
    where F: Fn(&Pix<T>, &Pix<T>) -> Pix<T> {
        assert!(self.width == other.width);
        assert!(self.height == other.height);
        
        let mut res = Img {
            width: self.width,
            height: self.height,
            pixels: Vec::with_capacity(self.pixcount()),
        };

        for (val, valo) in std::iter::zip(&self.pixels, &other.pixels) {
            res.pixels.push(func(val, valo));
        }

        res
    }

    pub fn combine_val<F>(&self, other: &Img<T>, func: F) -> Img<T>
    where F: Fn(&T, &T) -> T {
        assert!(self.width == other.width);
        assert!(self.height == other.height);
        
        let mut res = Img {
            width: self.width,
            height: self.height,
            pixels: Vec::with_capacity(self.pixcount()),
        };

        for (val, valo) in std::iter::zip(&self.pixels, &other.pixels) {
            let pix = Pix { r:func(&val.r, &valo.r), g:func(&val.g, &valo.g), b:func(&val.b, &valo.b) };
            res.pixels.push(pix);
        }

        res
    }

    pub fn convert<U: Clone, F>(&self, func: F) -> Img<U>
    where F: Fn(&T) -> U {
        let mut res = Img {
            width: self.width,
            height: self.height,
            pixels: Vec::with_capacity(self.pixcount()),
        };

        for val in &self.pixels {
            let pix: Pix<U> = Pix { r:func(&val.r), g:func(&val.g), b:func(&val.b) };
            res.pixels.push(pix);
        }

        res
    }

    pub fn set(&mut self,  xpos: usize, ypos: usize, pix: Pix<T>) {
        self.pixels[ypos*self.width + xpos] = pix;
    }

    pub fn at(&self, xpos: usize, ypos: usize) -> &Pix<T> {
        &self.pixels[ypos*self.width + xpos]
    }

    pub fn at_clamped(&self, xpos: i32, ypos: i32) -> &Pix<T> {
        let xp = xpos.max(0).min(self.width as i32 - 1);
        let yp = ypos.max(0).min(self.height as i32 - 1);
        &self.pixels[(yp as usize)*self.width + (xp as usize)]
    }

    pub fn pixcount(&self) -> usize {
        self.width * self.height
    }

    pub fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }
}

impl Img<u8> {
    pub fn as_f32(&self) -> Img<f32> {
        self.convert(|val| *val as f32)
    }
}

impl Img<f32> {
    pub fn as_u8(&self) -> Img<u8> {
        self.convert(|val| val.clamp(0.0, 255.0) as u8)
    }
    
    pub fn as_u8_wrap(&self) -> Img<u8> {
        self.convert(|val| ((*val as i32) & 0xFF) as u8)
    }
    
}
