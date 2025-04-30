use std::{
    collections::HashMap,
    env,
    error::Error,
    fmt::Display,
    path::{Path, PathBuf},
};

use super::{FontProvider, FontReference};

#[derive(Clone, Debug)]
pub struct FileProvider {
    pub cache: HashMap<String, PathBuf>,
    pub known_files: Vec<PathBuf>,
}
impl FileProvider {
    pub fn clean(s: String) -> String {
        s.replace('~', env::home_dir().unwrap().to_str().unwrap())
    }

    fn find(root: &Path, paths: &mut Vec<PathBuf>) -> Result<(), std::io::Error> {
        for path in root.read_dir()? {
            let path = path?.path();
            if path.is_dir() {
                let _ = Self::find(&path, paths);
            }
            if let Some(a) = path
                .extension()
                .and_then(|a| a.to_str())
                .map(|a| a.to_lowercase())
            {
                if crate::types::ACCEPTED_EXTENSIONS.contains(&a.as_str()) {
                    paths.push(path);
                }
            }
        }

        Ok(())
    }
    pub fn try_new<E: Display, T: IntoIterator<Item = E>>(
        roots: T,
    ) -> Result<Self, Box<dyn Error>> {
        let mut files = Vec::new();
        for root in roots {
            let pb = PathBuf::from(Self::clean(root.to_string()));
            let _ = Self::find(&pb, &mut files);
        }

        Ok(Self {
            cache: HashMap::default(),
            known_files: files,
        })
    }
}
pub struct FileFontReference {
    path: PathBuf,
}
impl FontReference for FileFontReference {
    type Variant = ();

    fn get_font(&self, _: &Self::Variant) -> Result<PathBuf, Box<dyn Error>> {
        Ok(self.path.clone())
    }
}
impl FontProvider for FileProvider {
    type Criteria = ();

    type FontReference = FileFontReference;

    fn new(cfg: &crate::config::Config) -> Result<Self, Box<dyn Error>> {
        Self::try_new(cfg.additional.iter().map(|a| a.to_str().unwrap()))
    }

    fn find_fonts(
        &mut self,
        _: &Self::Criteria,
    ) -> Result<Vec<Self::FontReference>, Box<dyn Error>> {
        Ok(self
            .known_files
            .iter()
            .map(|a| FileFontReference { path: a.clone() })
            .collect())
    }
}
