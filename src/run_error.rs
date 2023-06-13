use std::error;
use std::fmt;
use luxlib::ColorizeError;

#[derive(Debug, Clone)]
pub enum RunError {
    Colorize(ColorizeError),
    File(String),
    Scroller(String)
}

impl fmt::Display for RunError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Colorize(e) => write!(f, "Colorize: {}", e),
            Self::File(e) => write!(f, "File: {}", e),
            Self::Scroller(e) => write!(f, "Scroller: {}", e),
        }
    }
}

impl error::Error for RunError {}
