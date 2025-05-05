#![warn(missing_docs)]
#![feature(slice_as_array)]
#![feature(maybe_uninit_uninit_array_transpose)]
#![feature(debug_closure_helpers)]
#![feature(maybe_uninit_array_assume_init)]
pub mod error;
// pub mod fonts;
pub mod config;
pub mod glyph;
pub mod localization;
pub mod providers;
pub mod types;
pub mod utils;
pub trait Font {}
impl Font for () {}
pub mod string_encoding;
