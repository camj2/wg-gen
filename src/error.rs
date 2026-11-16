use core::result;
use std::error;
use std::fmt;
use std::io;

// Example: https://github.com/serde-rs/json/blob/master/src/error.rs

pub type Result<T> = result::Result<T, Error>;

pub enum Error {
    IOError(io::Error),
    SerdeError(serde_json::Error),
    NoneFound,
    TooManyPeers,
    V4NotSupported,
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::IOError(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::SerdeError(e)
    }
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::IOError(e) => write!(f, "{}", e)?,
            Self::SerdeError(e) => write!(f, "{}", e)?,
            Self::NoneFound => write!(f, "None Found. No peers have been generated.")?,
            Self::TooManyPeers => write!(f, "Too many peers. Not enough slots.")?,
            Self::V4NotSupported => write!(f, "IPv4 not supported.")?,
        };
        Ok(())
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
