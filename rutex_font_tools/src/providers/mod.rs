use std::{
    error::Error,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use crate::config::Config;

pub mod filebasedprovider;
#[cfg(feature = "google_fonts")]
pub mod google;
pub mod local;
#[cfg(feature = "online_fonts")]
pub mod online;
#[cfg(feature = "os_fonts")]
pub mod os;
pub trait FontReference {
    type Variant;
    fn get_font(&self, variant: &Self::Variant) -> Result<PathBuf, Box<dyn Error>>;
}
pub trait FontProvider: Sized {
    type Criteria;
    type FontReference: FontReference;
    const NAME: &str;
    fn new(cfg: &Config) -> Result<Self, Box<dyn Error>>;
    fn find_fonts(
        &mut self,
        criteria: &Self::Criteria,
    ) -> Result<Vec<Self::FontReference>, Box<dyn Error>>;
}

#[derive(Clone)]
pub struct FontWrapper<T: FontProvider>(Arc<Mutex<T>>);
impl<T: FontProvider> FontWrapper<T> {
    pub fn find_fonts(
        &self,
        criteria: &T::Criteria,
    ) -> Result<Vec<T::FontReference>, Box<dyn Error + '_>> {
        let mut v = self.0.lock()?;
        v.find_fonts(criteria)
    }
}
