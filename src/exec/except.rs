use std::fmt;
use std::error::Error;

use crate::img::ppmio;

#[derive(Debug)]
pub struct ExecError {
    details: String,
}

impl ExecError {
    pub fn new(msg: &str) -> ExecError {
        ExecError{details: msg.to_string()}
    }
}

impl fmt::Display for ExecError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for ExecError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl From<String> for ExecError {
    fn from(err: String) -> ExecError {
        return ExecError::new(&err);
    }
}

impl From<ppmio::PPMError> for ExecError {
    fn from(err: ppmio::PPMError) -> ExecError {
        return ExecError::new(&err.to_string());
    }
}

