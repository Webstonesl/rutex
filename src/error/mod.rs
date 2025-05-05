#[cfg(test)]
use std::backtrace::Backtrace;
use std::{
    char::DecodeUtf16Error,
    fmt::{Debug, Display},
    num::ParseIntError,
    str::Utf8Error,
};

use crate::pdf::maths;

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    IoError,
    EoFError,
    DoesNotExistError,
    ArgumentError,
    ReadError(String),
    InvalidCharacter,
    Terminate,
    InGroup,
    PopGroup(crate::parser::Token),
    NoToken,
    ParseError,
    IllegalParameter,
    DuplicateParameters,
    PatternMatchError,
    MathsError(maths::ErrorKind),
    PdfParseUnknownKeyword,
    PdfParseUnendingStream,
    PdfParseIllegalSymbol,
    KeyError,
    NotImplemented,
}

impl Into<Error> for ErrorKind {
    fn into(self) -> Error {
        Error::new(self)
    }
}

pub struct Error {
    pub(crate) kind: ErrorKind,
    pub(crate) message: Option<String>,
    #[cfg(test)]
    pub(crate) backtrace: Backtrace,
}
impl Clone for Error {
    fn clone(&self) -> Self {
        Self {
            kind: self.kind.clone(),
            message: self.message.clone(),
            #[cfg(test)]
            backtrace: Backtrace::capture(),
        }
    }
}
// pub use ErrorKind::*;
impl Error {
    #[inline]
    pub fn new_with_message<T: ToString>(kind: ErrorKind, message: T) -> Self {
        Error {
            kind,
            message: Some(message.to_string()),
            #[cfg(test)]
            backtrace: Backtrace::capture(),
        }
    }

    #[inline]
    pub fn new(kind: ErrorKind) -> Self {
        Error {
            kind,
            message: None,
            #[cfg(test)]
            backtrace: Backtrace::capture(),
        }
    }

    pub fn with_message(mut self, s: &str) -> Self {
        self.message = Some(s.to_string());
        return self;
    }

    pub fn read_error(s: String) -> Self {
        Error {
            kind: ErrorKind::ReadError(s.clone()),
            message: Some(s),
            #[cfg(test)]
            backtrace: Backtrace::capture(),
        }
    }
}
// impl From<&utf8streamreader::errors::Error> for Error {
//     fn from(value: &utf8streamreader::errors::Error) -> Self {
//         match value {
//             utf8streamreader::errors::Error::IoError(error) => Self {
//                 kind: ErrorKind::IoError,
//                 message: Some(error.to_string()),
//                 #[cfg(test)]
//                 backtrace: Backtrace::force_capture(),
//             },
//             utf8streamreader::errors::Error::EofError => Error {
//                 kind: ErrorKind::EoFError,
//                 message: Some("EOF reached".to_string()),
//                 #[cfg(test)]
//                 backtrace: Backtrace::force_capture(),
//             },
//             utf8streamreader::errors::Error::Other(s) => Error::read_error(s.clone()),
//             utf8streamreader::errors::Error::Utf8Error(s) => Error::read_error(s.clone()),
//         }
//     }
// }

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error {
            kind: ErrorKind::IoError,
            message: Some(value.to_string()),
            #[cfg(test)]
            backtrace: Backtrace::capture(),
        }
    }
}

// impl From<utf8streamreader::errors::Error> for Error {
//     fn from(value: utf8streamreader::errors::Error) -> Self {
//         match value {
//             utf8streamreader::errors::Error::IoError(error) => Self {
//                 kind: ErrorKind::IoError,
//                 message: Some(error.to_string()),
//                 #[cfg(test)]
//                 backtrace: Backtrace::capture(),
//             },
//             utf8streamreader::errors::Error::EofError => Error {
//                 kind: ErrorKind::EoFError,
//                 message: Some("EOF reached".to_string()),
//                 #[cfg(test)]
//                 backtrace: Backtrace::capture(),
//             },
//             utf8streamreader::errors::Error::Other(s) => Error::read_error(s.clone()),
//             utf8streamreader::errors::Error::Utf8Error(s) => Error::read_error(s.clone()),
//         }
//     }
// }

impl From<clap::Error> for Error {
    fn from(value: clap::Error) -> Self {
        return Self {
            kind: ErrorKind::ArgumentError,
            message: Some(value.to_string()),
            #[cfg(test)]
            backtrace: Backtrace::capture(),
        };
    }
}

impl From<ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
        Self::new_with_message(ErrorKind::ParseError, value)
    }
}
impl From<Utf8Error> for Error {
    fn from(value: Utf8Error) -> Self {
        Self::new_with_message(ErrorKind::ParseError, value.to_string())
    }
}
impl From<DecodeUtf16Error> for Error {
    fn from(value: DecodeUtf16Error) -> Self {
        Self::new_with_message(ErrorKind::ParseError, value.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {:?}", self.kind)?;

        if let Some(ref message) = self.message {
            write!(f, " {}", message.trim_start_matches("error:"))?;
        }
        #[cfg(test)]
        {
            write!(f, " {:?}", self.backtrace)?;
        }
        writeln!(f)
    }
}
impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[cfg(not(test))]
        {
            return f
                .debug_struct("Error")
                .field("kind", &self.kind)
                .field("message", &self.message)
                .finish();
        }
        #[cfg(test)]
        {
            return f
                .debug_struct("Error")
                .field("kind", &self.kind)
                .field("message", &self.message)
                .field("backtrace", &self.backtrace)
                .finish();
        }
    }
}
