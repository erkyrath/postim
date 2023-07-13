
#[derive(Default)]
#[derive(Clone)]
#[derive(Debug)]
pub struct Pix<T> {
    pub r: T,
    pub g: T,
    pub b: T,
}

impl<T: Copy> Pix<T> {
    pub fn new(valr: T, valg: T, valb: T) -> Pix<T> {
        Pix { r:valr, g:valg, b:valb }
    }
    
    pub fn grey(val: T) -> Pix<T> {
        Pix { r:val, g:val, b:val }
    }
}

