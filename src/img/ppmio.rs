use std::fs;
use std::fmt;
use std::error::Error;
use std::io::Read;
use std::io::Write;

use crate::img::pixel::Pix;
use crate::img::Img;

fn read_line(fl: &mut fs::File) -> Option<String> {
    let mut vec: Vec<u8> = Vec::with_capacity(80);
    
    let mut ch: [u8; 1] = [0; 1];
    loop {
        fl.read_exact(&mut ch).ok()?;
        if ch[0] == b'\n' {
            break;
        }
        vec.push(ch[0]);
    }

    let st = String::from_utf8(vec);
    match st {
        Ok(st) => Some(st),
        Err(_) => None,
    }
}

#[derive(Debug)]
pub struct PPMError {
    details: String,
}

impl PPMError {
    fn new(msg: &str) -> PPMError {
        PPMError{details: msg.to_string()}
    }
}

impl fmt::Display for PPMError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for PPMError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl From<std::io::Error> for PPMError {
    fn from(err: std::io::Error) -> PPMError {
        return PPMError::new(&err.to_string());
    }
}

impl From<std::num::ParseIntError> for PPMError {
    fn from(err: std::num::ParseIntError) -> PPMError {
        return PPMError::new(&err.to_string());
    }
}

pub fn img_read(filename: &str) -> Result<Img<u8>, PPMError> {
    let mut fl = fs::File::open(&filename)?;

    let ppmtype = read_line(&mut fl)
        .ok_or(PPMError::new("can't read type line"))?;
    if ppmtype != "P6" {
        return Err(PPMError::new("type line is not P6"));
    }

    let comment = read_line(&mut fl)
        .ok_or(PPMError::new("can't read comment line"))?;
    if !comment.starts_with("#") {
        return Err(PPMError::new("comment line is not comment"));
    }

    let size = read_line(&mut fl)
        .ok_or(PPMError::new("can't read size line"))?;
        
    let vec: Vec<&str> = size.split(' ').collect();
    if vec.len() != 2 {
        return Err(PPMError::new("bad type line"));
    }

    let width = vec[0].parse::<usize>()?;
    let height = vec[1].parse::<usize>()?;

    let bits = read_line(&mut fl)
        .ok_or(PPMError::new("can't read bits line"))?;
    if bits != "255" {
        return Err(PPMError::new("bits line is not 255"));
    }

    let mut img: Img<u8> = Img::new(width, height);
    
    let mut buf: Vec<u8> = vec![0; 3*img.width];
    for jx in 0..img.height {
        fl.read_exact(&mut buf)?;
        for ix in 0..img.width {
            let pix: Pix<u8> = Pix { r:buf[ix*3+0], g:buf[ix*3+1], b:buf[ix*3+2] };
            img.set(ix, jx, pix);
        }
    }

    Ok(img)
}

pub fn img_write(filename: &str, img: Img<u8>) -> Result<(), PPMError> {
    let mut fl = fs::File::create(&filename)?;

    fl.write_all(b"P6\n")?;
    fl.write_all(b"#\n")?;

    let size = format!("{} {}\n", img.width, img.height);
    fl.write_all(size.as_bytes())?;
    
    fl.write_all(b"255\n")?;

    let mut buf: Vec<u8> = vec![0; 3*img.width];
    for jx in 0..img.height {
        for ix in 0..img.width {
            let pix = img.at(ix, jx);
            buf[ix*3+0] = pix.r;
            buf[ix*3+1] = pix.g;
            buf[ix*3+2] = pix.b;
        }
        fl.write_all(&buf)?;
    }
    
    Ok(())
}
