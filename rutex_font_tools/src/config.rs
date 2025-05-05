#[cfg(feature = "google_fonts")]
use crate::providers::google::GoogleFontsConfig;
use std::sync::Arc;
use std::{env, path::PathBuf};
pub struct Config {
    #[cfg(feature = "google_fonts")]
    pub google_fonts_config: Option<GoogleFontsConfig>,
    #[cfg(feature = "temp_dir")]
    pub temp_dir: Arc<PathBuf>,
    pub current_dir: PathBuf,
    pub additional: Vec<PathBuf>,
}
impl Config {
    #[cfg(feature = "google_fonts")]
    pub fn with_google_fonts(mut self, api_key: String) -> Self {
        self.google_fonts_config = Some(GoogleFontsConfig {
            api_key,
            temp_dir: self.temp_dir.clone(),
        });
        self
    }
    #[cfg(feature = "temp_dir")]
    pub fn new<T: Into<PathBuf>>(temp_dir: T) -> Self {
        Self {
            #[cfg(feature = "google_fonts")]
            google_fonts_config: None,
            temp_dir: Arc::new(temp_dir.into()),
            current_dir: env::current_dir().unwrap(),
            additional: vec![],
        }
    }
}
impl Config {}
#[cfg(not(feature = "temp_dir"))]
impl Default for Config {
    fn default() -> Self {
        Self {
            #[cfg(feature = "google_fonts")]
            google_fonts_config: 'a: {
                #[cfg(GOOGLE_API_KEY)]
                break 'a Some(GoogleFontsConfig::default());
                #[cfg(not(GOOGLE_API_KEY))]
                break 'a None;
            },

            current_dir: env::current_dir().unwrap(),
            additional: Default::default(),
        }
    }
}
