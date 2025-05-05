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
pub mod pdf;
