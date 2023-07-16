use std::fmt;
use std::rc::Rc;

pub mod parse;

#[derive(Debug, Clone)]
pub enum ScriptToken {
    Whitespace,        // parse only
    Comment,           // parse only
    Operator(String),  // parse only
    Delimiter(String), // parse only
    Name(String),
    StoreTo(String),
    String(String),
    Integer(i32),
    Float(f32),
    Size(i32, i32),
    Color(u8, u8, u8),
    Proc(Rc<Vec<ScriptToken>>),
}

pub struct Script {
    filename: String,
    tokens: Rc<Vec<ScriptToken>>,
}

impl Script {
    pub fn new(filename: &str, tokens: Rc<Vec<ScriptToken>>) -> Script {
        Script {
            filename: filename.to_string(),
            tokens
        }
    }

    pub fn tokens(&self) -> &[ScriptToken] {
        return &self.tokens;
    }

    pub fn tokeniter(&self) -> TokenRefIter<ScriptToken> {
        return TokenRefIter::new(Rc::clone(&self.tokens));
    }
}

impl fmt::Debug for Script {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<Script \"{}\">", self.filename)
    }
}

pub struct TokenRefIter<'a, T: 'a> {
    tokens: Rc<Vec<T>>,
    count: usize,
    _marker: std::marker::PhantomData<&'a T>,
}

impl<'a, T> TokenRefIter<'a, T> {
    fn new(tokens: Rc<Vec<T>>) -> TokenRefIter<'a, T> {
        TokenRefIter {
            tokens,
            count: 0,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<'a, T> Iterator for TokenRefIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if self.count < self.tokens.len() {
            let oldcount = self.count;
            self.count += 1;
            Some(&self.tokens[oldcount])
        }
        else {
            None
        }
    }
}
