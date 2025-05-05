use std::{
    error::Error,
    fmt::{Debug, Display},
};
#[derive(Debug)]
pub enum SFNTError {
    IOError(Box<std::io::Error>),
    DynamicError(Box<dyn std::error::Error>),
}
impl From<std::io::Error> for SFNTError {
    fn from(value: std::io::Error) -> Self {
        Self::IOError(Box::new(value))
    }
}
impl Display for SFNTError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SFNTError::IOError(error) => {
                write!(f, "IO Error: ")?;
                Display::fmt(error, f)?
            }
            SFNTError::DynamicError(error) => Display::fmt(error, f)?,
        }
        Ok(())
    }
}
impl Error for SFNTError {}
