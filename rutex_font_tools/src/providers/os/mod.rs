#[cfg(target_family = "unix")]
pub mod unix {
    #[cfg(target_os = "macos")]
    pub const SEARCH_PATHS: &[&str] = &[
        "~/Library/Fonts/",
        "/Library/Fonts/",
        "/Network/Library/Fonts/",
        "/System/Library/Fonts/",
    ];
    #[cfg(target_os = "linux")]
    pub const SEARCH_PATHS: &[&str] = &["/usr/share/fonts", "~/.local/share/fonts"];
    pub fn expand_home(s: &str) -> String {
        s.replace("~", std::env::home_dir().unwrap().to_str().unwrap())
    }
}
use std::{error::Error, path::PathBuf};

#[cfg(target_family = "unix")]
pub use unix::*;

use super::{FontProvider, filebasedprovider::FileProvider};

#[cfg(target_family = "windows")]
pub mod windows {
    pub const SEARCH_PATHS: &[&str] = &[
        r"C:\Windows\Fonts",
        r"~\AppData\Local\Microsoft\Windows\Fonts",
    ];
    pub fn expand_home(s: &str) -> String {
        s.to_string()
    }
}
pub fn get_paths() -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut result = Vec::new();
    for p in SEARCH_PATHS {
        let pb = PathBuf::from(expand_home(p));
        if !pb.exists() {
            continue;
        }
        result.push(pb);
    }
    Ok(result)
}
pub struct OSProvider(FileProvider);
impl FontProvider for OSProvider {
    type Criteria = <FileProvider as FontProvider>::Criteria;

    type FontReference = <FileProvider as FontProvider>::FontReference;
    const NAME: &str = "os";

    fn new(_: &crate::config::Config) -> Result<Self, Box<dyn Error>> {
        Ok(Self(FileProvider::try_new(SEARCH_PATHS)?))
    }

    fn find_fonts(
        &mut self,
        criteria: &Self::Criteria,
    ) -> Result<Vec<Self::FontReference>, Box<dyn Error>> {
        self.0.find_fonts(criteria)
    }
}
impl AsRef<FileProvider> for OSProvider {
    fn as_ref(&self) -> &FileProvider {
        &self.0
    }
}
impl AsMut<FileProvider> for OSProvider {
    fn as_mut(&mut self) -> &mut FileProvider {
        &mut self.0
    }
}
