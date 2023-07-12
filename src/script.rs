pub mod parse;

#[derive(Debug, Clone)]
pub enum ScriptToken {
    Whitespace,
    Comment,
    OpArrow,
    Name(String),
    StoreTo(String),
    String(String),
    Integer(i32),
    Float(f32),
    Size(i32, i32),
    Color(u8, u8, u8),
}

pub struct Script {
    tokens: Vec<ScriptToken>,
}

impl Script {
    pub fn new(tokens: Vec<ScriptToken>) -> Script {
        Script {
	    tokens
	}
    }
}
