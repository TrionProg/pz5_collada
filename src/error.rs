use std;

use collada;

#[derive(Debug)]
pub enum Error{
    ColladaError(collada::Error),
    StringParseError(String),
    SemanticsParse(String),
    ByteOrderError(std::io::Error),
    Other(String),
}

impl std::fmt::Display for Error{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self{
            Error::ColladaError(ref e) => write!(f, "Collada error:{}", e),
            Error::StringParseError(ref message) => write!(f, "String parse error: {}", message),
            Error::SemanticsParse(ref e) => write!(f, "Semantics parse error:{}", e),
            Error::ByteOrderError(ref e) => write!(f, "Byte order error:{}", e),
            Error::Other(ref message) => write!(f, "{}", message),
        }
    }
}
/*
impl From<i32> for Error {
    fn from(n:i32) -> Error {
        match n {
            -1 => Error::Gen,
            -2 => Error::NotOpen,
            -3 => Error::Wave,
            _ => Error::Unknown,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Sphinx audio device error:{}",
            match *self {
                Error::Gen => "Gen",
                Error::NotOpen => "NotOpen",
                Error::Wave => "Wave",
                Error::Unknown => "Unknown",
            }
        )
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Gen => "Gen",
            Error::NotOpen => "NotOpen",
            Error::Wave => "Wave",
            Error::Unknown => "Unknown",
        }
    }
    fn cause(&self) -> Option<&std::error::Error> { None }
}
*/
