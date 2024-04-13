use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct IllegalArgumentException(String);

impl IllegalArgumentException {
    pub fn new(msg: &str) -> IllegalArgumentException {
        IllegalArgumentException(msg.to_string())
    }
}

impl fmt::Display for IllegalArgumentException {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for IllegalArgumentException {
    fn description(&self) -> &str {
        &self.0
    }
}

#[derive(Debug)]
pub struct IllegalStateException {
    details: String,
}

impl IllegalStateException {
    pub fn new(msg: &str) -> IllegalStateException {
        IllegalStateException {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for IllegalStateException {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for IllegalStateException {
    fn description(&self) -> &str {
        &self.details
    }
}
