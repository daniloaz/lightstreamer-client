use std::fmt;
use std::error::Error;

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

impl From<Box<dyn Error>> for IllegalArgumentException {
    fn from(error: Box<dyn Error>) -> Self {
        IllegalArgumentException::new(&error.to_string())
    }
}

#[derive(Debug)]
pub struct IllegalStateException {
    details: String
}

impl IllegalStateException {
    pub fn new(msg: &str) -> IllegalStateException {
        IllegalStateException{details: msg.to_string()}
    }
}

impl fmt::Display for IllegalStateException {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for IllegalStateException {
    fn description(&self) -> &str {
        &self.details
    }
}

impl From<Box<dyn Error>> for IllegalStateException {
    fn from(error: Box<dyn Error>) -> Self {
        IllegalStateException::new(&error.to_string())
    }
}

impl From<serde_urlencoded::ser::Error> for IllegalStateException {
    fn from(err: serde_urlencoded::ser::Error) -> Self {
        IllegalStateException::new(&format!("Serialization error: {}", err))
    }
}

impl From<tokio_tungstenite::tungstenite::Error> for IllegalStateException {
    fn from(err: tokio_tungstenite::tungstenite::Error) -> Self {
        IllegalStateException::new(&format!("WebSocket error: {}", err))
    }
}