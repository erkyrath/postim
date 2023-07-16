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

    pub fn tokens(&self) -> Rc<Vec<ScriptToken>> {
        Rc::clone(&self.tokens)
    }
}

impl fmt::Debug for Script {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<Script \"{}\">", self.filename)
    }
}

