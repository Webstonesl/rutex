use std::{cell::LazyCell, collections::HashMap, fs, path::PathBuf, sync::LazyLock};

use crate::{
    error::{self, Error},
    fonts::filetypes::sfnt::SFNTReader,
};

use super::FontProvider;

#[cfg(target_os = "macos")]
mod mac_os {
    use regex::Captures;

    use crate::error::{Error, ErrorKind};
    use std::{collections::HashMap, env::var, iter::Map, path::PathBuf, sync::Mutex};
    const PATHS: &'static [&'static str] = &[
        "~/Library/Fonts/",
        "/Library/Fonts/",
        "/Network/Library/Fonts/",
        "/System/Library/Fonts/",
    ];

    struct Replacer {
        variables: HashMap<String, String>,
    }
    impl Replacer {
        fn new() -> Self {
            Replacer {
                variables: HashMap::new(),
            }
        }
    }
    impl regex::Replacer for Replacer {
        fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut String) {
            let c = match caps.get(0).unwrap().as_str() {
                "~" => "HOME",
                _ => caps.get(1).unwrap().as_str(),
            };
            if let Ok(a) = std::env::var(c) {
                dst.push_str(&a);
            }
        }
    }
    pub fn search(root: PathBuf, v: &mut Vec<PathBuf>) -> Result<(), Error> {
        if let Ok(rdi) = root.read_dir() {
            for item in rdi {
                if let Ok(item) = item {
                    let ft = item.file_type().unwrap();
                    if ft.is_dir() {
                        search(item.path(), v)?;
                    } else if ft.is_file() {
                        let path = item.path();
                        if let Some(a) = path.extension() {
                            if let Some(a) = a.to_str() {
                                match a.to_lowercase().as_str() {
                                    "ttf" | "otf" | "woff" | "woff2" => v.push(path),
                                    _ => {}
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub fn get_font_files() -> Result<Vec<PathBuf>, Error> {
        let rg = regex::Regex::new(r"\$([A-Z_][A-Z_0-9]+)|~").unwrap();
        let mut v = Vec::new();
        for path in PATHS {
            let path = rg.replace_all(*path, Replacer::new());
            let a = PathBuf::from(&*path);
            if a.exists() {
                search(a, &mut v)?;
            }
        }
        Ok(v)
    }
}
#[cfg(target_os = "macos")]
const GET_FONT_FILES: fn() -> Result<Vec<PathBuf>, error::Error> = mac_os::get_font_files;

static FONT_FILES: LazyLock<Result<Vec<PathBuf>, Error>> = LazyLock::new(GET_FONT_FILES);
#[derive(Debug)]
pub struct OSFontProvider {
    pub files: Vec<PathBuf>,
}
impl OSFontProvider {
    pub fn new(files: Vec<PathBuf>) -> Result<Self, Error> {
        for file in &files {
            let extension = file
                .extension()
                .and_then(|a| a.to_str())
                .and_then(|a| Some(a.to_string()))
                .ok_or_else(|| {
                    Error::new_with_message(
                        error::ErrorKind::IoError,
                        format!("{file:?} has no file extension"),
                    )
                })?;
            let file = fs::File::open(file)?;
            let v = match extension.to_lowercase().as_str() {
                "ttf" | "otf" => SFNTReader::read_from_file(file)?,
                a => {
                    return Err(Error::new_with_message(
                        error::ErrorKind::IllegalParameter,
                        format!("{a:?} is an illegal extension"),
                    ));
                }
            };
        }
        Ok(Self { files })
    }
    #[cfg(target_os = "macos")]
    pub fn default() -> Result<Self, Error> {
        FONT_FILES.clone().and_then(|e| Self::new(e))
    }
}

impl FontProvider for OSFontProvider {
    type SearchType = HashMap<String, String>;

    fn list_fonts(&mut self, family: &Self::SearchType) -> Result<Vec<super::FontFamily>, Error> {
        let paths = match FONT_FILES.as_ref() {
            Ok(v) => v,
            Err(e) => return Err(e.clone()),
        };
        for p in paths {
            println!("{:?}", p);
        }
        todo!()
    }

    fn get_font<P: Into<PathBuf>>(
        &self,
        font_option: &mut super::FontFamily,
        parent: P,
    ) -> Result<(), Error> {
        todo!()
    }
}
#[test]
fn test() {
    dbg!(OSFontProvider::default().unwrap());
}
