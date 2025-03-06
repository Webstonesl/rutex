use std::{fmt::Display, num::ParseIntError};

#[derive(Debug)]
pub enum ErrorKind {
    UnknownError,
    UnknownTokenError,
    UnknownMacroError,
    EndOfFile,
    ParseError,
}
#[derive(Debug)]
pub struct Error {
    location: Option<(String, usize, usize)>,
    kind: ErrorKind,
    message: String,
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some((filename, row, column)) = &self.location {
            f.write_str(
                format!(
                    "{filename}:{row}:{column} [{:?}] {}",
                    self.kind, self.message
                )
                .as_str(),
            )?;
            Ok(())
        } else {
            f.write_str(format!("[{:?}] {}", self.kind, self.message).as_str())?;
            Ok(())
        }
    }
}
impl Error {
    pub fn new_with_location(
        location: Option<(String, usize, usize)>,
        kind: ErrorKind,
        message: String,
    ) -> Self {
        Error {
            location,
            kind,
            message,
        }
    }
    pub fn new(kind: ErrorKind, message: String) -> Self {
        Error {
            location: None,
            kind,
            message,
        }
    }

    pub fn eof() -> Error {
        Error {
            location: None,
            kind: ErrorKind::EndOfFile,
            message: "End of file reached".to_string(),
        }
    }
}
impl From<ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
        Error::new(ErrorKind::ParseError, value.to_string())
    }
}
