#![feature(str_internals)]
#![feature(array_chunks)]
#![feature(slice_as_array)]
#![allow(internal_features)]
pub(crate) mod config;
pub mod error;
pub mod macros;
pub mod parser;
pub mod reader;
pub mod state;
pub mod writer;
pub use config::*;
#[cfg(test)]
pub(crate) mod test {
    use std::io::{self, Read};

    pub struct ByteReader(pub Vec<u8>, pub usize);
    impl ByteReader {
        pub const fn new(v: Vec<u8>) -> Self {
            ByteReader(v, 0)
        }
    }
    impl Read for ByteReader {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            let otherlen = buf.len();
            let selflen = self.0.len() - self.1;
            let len = selflen.min(otherlen);
            buf[..len].copy_from_slice(&self.0[self.1..(self.1 + len)]);
            self.1 += len;
            Ok(len)
        }
    }
}
