use crate::pixel::Pix;
use crate::img::Img;

impl Img<f32> {

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
    
    pub fn halfshift(&self) -> Img<f32> {
        let mut res: Img<f32> = Img::new(self.width, self.height);
        
        let halfwidth = self.width/2;
        let halfheight = self.height/2;
        
        for jx in 0..self.height {
            for ix in 0..self.width {
                let pix = self.at((ix+halfwidth) % self.width, (jx+halfheight) % self.height);
                res.set(ix, jx, pix.clone());
            }
        }
        
        res
    }

}
