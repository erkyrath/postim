use std::fmt;

use crate::pixel::Pix;

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

    pub fn average(&self) -> Pix<f32> {
        let mut total: Pix<f32> = Pix { r:0.0, g:0.0, b:0.0 };
        for val in &self.pixels {
            total.r += val.r;
            total.g += val.g;
            total.b += val.b;
        }
        let pixcount: f32 = self.pixcount() as f32;
        total.r /= pixcount;
        total.g /= pixcount;
        total.b /= pixcount;
        return total;
    }

    pub fn at_lerp(&self, xpos: f32, ypos: f32) -> Pix<f32> {
        if xpos.is_nan() || ypos.is_nan() {
            return Pix::default();
        }
        
        let x0 = xpos.floor() as i32;
        let y0 = ypos.floor() as i32;
        let xfrac = xpos - xpos.floor();
        let yfrac = ypos - ypos.floor();

        let pix00 = self.at_clamped(x0, y0);
        let pix01 = self.at_clamped(x0+1, y0);
        let pix10 = self.at_clamped(x0, y0+1);
        let pix11 = self.at_clamped(x0+1, y0+1);

        let res = Pix {
            r: pix00.r * (1.0-xfrac) * (1.0-yfrac) + pix01.r * (xfrac) * (1.0-yfrac) + pix10.r * (1.0-xfrac) * yfrac + pix11.r * (xfrac) * (yfrac),
            g: pix00.g * (1.0-xfrac) * (1.0-yfrac) + pix01.g * (xfrac) * (1.0-yfrac) + pix10.g * (1.0-xfrac) * yfrac + pix11.g * (xfrac) * (yfrac),
            b: pix00.b * (1.0-xfrac) * (1.0-yfrac) + pix01.b * (xfrac) * (1.0-yfrac) + pix10.b * (1.0-xfrac) * yfrac + pix11.b * (xfrac) * (yfrac),
        };

        res
    }
    
    pub fn project<F>(&self, func: F) -> Img<f32>
    where F: Fn(f32, f32) -> (f32, f32) {
        let mut res = Img::new(self.width, self.height);
        for jx in 0..self.height {
            for ix in 0..self.width {
                let newpos = func(ix as f32, jx as f32);
                let pix = self.at_lerp(newpos.0, newpos.1);
                res.set(ix, jx, pix);
            }
        }
        res
    }
    
    pub fn project_shade<F>(&self, func: F) -> Img<f32>
    where F: Fn(f32, f32) -> (f32, f32, f32) {
        let mut res = Img::new(self.width, self.height);
        for jx in 0..self.height {
            for ix in 0..self.width {
                let (newx, newy, shade) = func(ix as f32, jx as f32);
                let mut pix = self.at_lerp(newx, newy);
                if shade > 0.0 {
                    pix.r = (1.0-shade) * pix.r + (shade) * 255.0;
                    pix.g = (1.0-shade) * pix.g + (shade) * 255.0;
                    pix.b = (1.0-shade) * pix.b + (shade) * 255.0;
                }
                else {
                    pix.r = (1.0+shade) * pix.r;
                    pix.g = (1.0+shade) * pix.g;
                    pix.b = (1.0+shade) * pix.b;
                }
                res.set(ix, jx, pix);
            }
        }
        res
    }

    pub fn interp_mask(&self, other: &Img<f32>, mask: &Img<f32>) -> Img<f32> {
        assert!(self.width == other.width);
        assert!(self.height == other.height);
        assert!(self.width == mask.width);
        assert!(self.height == mask.height);
        let mut res = Img::new(self.width, self.height);
        for jx in 0..self.height {
            for ix in 0..self.width {
                let selfpix = self.at(ix, jx);
                let otherpix = other.at(ix, jx);
                let maskpix = mask.at(ix, jx);
                let pix = Pix {
                    r: (1.0-maskpix.r) * selfpix.r + (maskpix.r) * otherpix.r,
                    g: (1.0-maskpix.g) * selfpix.g + (maskpix.g) * otherpix.g,
                    b: (1.0-maskpix.b) * selfpix.b + (maskpix.b) * otherpix.b,
                };
                res.set(ix, jx, pix);
            }
        }
        res
    }
    
    pub fn contrast(&self, val: f32) -> Img<f32> {
        let avpix = self.average();
        self.map(|pix| Pix {
            r:(pix.r-avpix.r) * val + avpix.r,
            g:(pix.g-avpix.g) * val + avpix.g,
            b:(pix.b-avpix.b) * val + avpix.b,
        })
    }
    
}
