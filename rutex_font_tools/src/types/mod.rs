use std::io::{Read, Seek};

pub const ACCEPTED_EXTENSIONS: &[&str] = &["ttf", "woff2", "otf"];
pub mod postscript;
pub mod sfnt_types;
pub mod woff2;

pub trait FontStream: Seek + Read {}
impl<T: Seek + Read> FontStream for T {}
