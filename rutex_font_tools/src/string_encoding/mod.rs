use std::{
    fmt::Debug,
    str::Utf8Error,
    string::{FromUtf8Error, FromUtf16Error},
};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum DecodingError {
    #[error("{0}")]
    UnknownError(String),
    #[error("No mapping found for {1} in {0:?}")]
    NoSuchMappingError(String, String),
    #[error("{0}")]
    InvalidLength(String),
}
impl From<Utf8Error> for DecodingError {
    fn from(value: Utf8Error) -> Self {
        DecodingError::UnknownError(value.to_string())
    }
}
impl From<FromUtf8Error> for DecodingError {
    fn from(value: FromUtf8Error) -> Self {
        DecodingError::UnknownError(value.utf8_error().to_string())
    }
}
impl From<FromUtf16Error> for DecodingError {
    fn from(value: FromUtf16Error) -> Self {
        DecodingError::UnknownError(value.to_string())
    }
}
pub trait Decoder {
    fn decode(&self, source: &[u8]) -> Result<String, DecodingError>;
}
pub struct FixedSizeDecoder<'a, 'b: 'a, Input: Copy + Sized + Ord>(
    pub &'a str,
    pub &'a [(Input, &'b [char])],
);
// impl<'a, 'b: 'a, Input: Copy + Sized + Ord> Sized for FixedSizeDecoder<'a, 'b, Input> {}
impl<'a, 'b: 'a> Decoder for FixedSizeDecoder<'a, 'b, u8> {
    fn decode(&self, input: &[u8]) -> Result<String, DecodingError> {
        let mut value = String::new();
        for input_value in input {
            match self.1.binary_search_by(|(probe, _)| probe.cmp(input_value)) {
                Ok(a) => value.extend(self.1[a].1),
                Err(_) => {
                    return Err(DecodingError::NoSuchMappingError(
                        self.0.to_string(),
                        format!("{:?}", input_value),
                    ));
                }
            }
        }
        Ok(value)
    }
}
impl<'a, 'b: 'a> Decoder for FixedSizeDecoder<'a, 'b, u16> {
    fn decode(&self, input: &[u8]) -> Result<String, DecodingError> {
        let mut value = String::new();
        if input.len() % 2 == 1 {
            return Err(DecodingError::InvalidLength(format!(
                "Input must have even length, {} found.",
                input.len()
            )));
        }
        for input_value in input.chunks_exact(2) {
            let input_value = u16::from_be_bytes(input_value.try_into().unwrap());
            match self
                .1
                .binary_search_by(|(probe, _)| probe.cmp(&input_value))
            {
                Ok(a) => value.extend(self.1[a].1),
                Err(_) => {
                    return Err(DecodingError::NoSuchMappingError(
                        self.0.to_string(),
                        format!("{:?}", input_value),
                    ));
                }
            }
        }
        Ok(value)
    }
}

pub struct UTF8Decoder;
impl Decoder for UTF8Decoder {
    fn decode(&self, source: &[u8]) -> Result<String, DecodingError> {
        Ok(str::from_utf8(source)?.to_string())
    }
}
pub struct UTF16Decoder<const BIG_ENDIAN: bool>;

impl<const BIG_ENDIAN: bool> Decoder for UTF16Decoder<BIG_ENDIAN> {
    fn decode(&self, source: &[u8]) -> Result<String, DecodingError> {
        if source.len() % 2 != 0 {
            return Err(DecodingError::InvalidLength(
                "UTF-16 requires an even number of bytes".to_string(),
            ));
        }

        let converter = if BIG_ENDIAN {
            u16::from_be_bytes
        } else {
            u16::from_le_bytes
        };

        let c = source.chunks_exact(2);
        Ok(String::from_utf16(
            &c.map(|a| converter(a.try_into().unwrap()))
                .collect::<Vec<u16>>(),
        )?)
    }
}
pub static UTF8_DECODER: UTF8Decoder = UTF8Decoder;
pub static UTF16BE_DECODER: UTF16Decoder<true> = UTF16Decoder;
pub static UTF16LE_DECODER: UTF16Decoder<false> = UTF16Decoder;

#[cfg(test)]
mod test {}
