use std::{collections::HashMap, path::PathBuf};

use crate::error::Error;

#[derive(Debug, Clone)]
pub struct FontFamily {
    name: String,
    files: HashMap<String, String>,
}
pub trait FontProvider {
    type SearchType;
    fn list_fonts(&mut self, family: &Self::SearchType) -> Result<Vec<FontFamily>, Error>;
    fn get_font<P: Into<PathBuf>>(
        &self,
        font_option: &mut FontFamily,
        parent: P,
    ) -> Result<(), Error>;
}

#[cfg(feature = "google_fonts")]
pub mod googlefonts;
pub mod osfonts;
// pub impl
