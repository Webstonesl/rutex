use std::{borrow::Cow, str::Utf8Error};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO: {0}")]
    IOError(std::io::Error),
    #[error("Decoding Error for {encoding}: {error}")]
    DecodingError {
        encoding: Cow<'static, str>,
        error: Box<dyn std::error::Error>,
    },
}
impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::IOError(value)
    }
}
impl From<Utf8Error> for Error {
    fn from(value: Utf8Error) -> Self {
        Error::DecodingError {
            encoding: "UTF-8".into(),
            error: Box::new(value),
        }
    }
}
