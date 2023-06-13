use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub enum ColorizeError {
    RegexPattern(String),
    ParseTag(String)
}

impl fmt::Display for ColorizeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RegexPattern(e) => write!(f, "Regular expression: {}", e),
            Self::ParseTag(e) => write!(f, "Parse tag: {}", e),
        }
    }
}

impl error::Error for ColorizeError {}
