use crate::providers::google::GoogleFontsConfig;
use std::{env, path::PathBuf};
pub struct Config {
    #[cfg(feature = "google_fonts")]
    pub google_fonts_config: Option<GoogleFontsConfig>,
    pub current_dir: PathBuf,
    pub additional: Vec<PathBuf>,
}
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
