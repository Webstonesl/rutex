use std::{collections::HashMap, fs, io::Write, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::{Error, ErrorKind};

use super::{FontFamily, FontProvider};

//env!("WEBFONTSKEY");

#[derive(Serialize, Deserialize, Debug, Clone)]
struct WebFontSpec {
    family: String,
    variants: Vec<String>,
    subsets: Vec<String>,
    category: String,
    version: String,
    files: HashMap<String, String>,
    menu: String,
    stored_location: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct GoogleFontResult {
    items: Vec<WebFontSpec>,
}

pub struct GoogleFontProvider {
    api_key: String,
    map: HashMap<String, WebFontSpec>,
}
impl GoogleFontProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            map: HashMap::new(),
        }
    }
}
impl FontProvider for GoogleFontProvider {
    type SearchType = HashMap<String, String>;
    fn list_fonts(
        &mut self,
        family: &Self::SearchType,
    ) -> Result<Vec<FontFamily>, crate::error::Error> {
        let key = "key".to_string();
        let sorting = (&String::from("sort"), &String::from("popularity"));
        let kv = family
            .iter()
            .chain([(&key, &self.api_key), sorting].into_iter())
            .map(|(x, y)| format!("{}={}", x, y))
            .reduce(|a, b| format!("{}&{}", a, b))
            .unwrap();
        let v = match reqwest::blocking::get(format!(
            "https://www.googleapis.com/webfonts/v1/webfonts?{kv}",
        )) {
            Ok(e) => e,
            Err(e) => {
                return Err(Error::new_with_message(
                    crate::error::ErrorKind::IoError,
                    e.to_string(),
                ));
            }
        };
        let GoogleFontResult { items } = v.json().map_err(|e| {
            Error::new_with_message(crate::error::ErrorKind::ParseError, e.to_string())
        })?;

        Ok(items
            .into_iter()
            .map(|a| {
                self.map.insert(a.family.clone(), a.clone());
                FontFamily {
                    name: a.family,
                    files: HashMap::new(),
                }
            })
            .collect())
    }

    fn get_font<P: Into<std::path::PathBuf>>(
        &self,
        font_option: &mut FontFamily,
        parent: P,
    ) -> Result<(), crate::error::Error> {
        let mut parent: PathBuf = parent.into();

        let value = self.map.get(&font_option.name).ok_or_else(|| {
            Error::new_with_message(
                ErrorKind::KeyError,
                format!("Font {:?} not found.", font_option.name),
            )
        })?;
        dbg!(value);
        let mut viter = value.files.iter();
        let mut text = &"menu".to_string();
        let mut url = &value.menu;
        loop {
            let extension = url.split(".").last().unwrap();
            parent.push(format!("{text}.{extension}"));
            let bytes = match reqwest::blocking::get(url).and_then(|o| o.bytes()) {
                Ok(o) => o,
                Err(e) => return Err(Error::new_with_message(ErrorKind::IoError, e)),
            };
            {
                let mut file = fs::File::create_new(&parent)?;
                let mut b = 0;
                loop {
                    let a = file.write(&bytes[b..])?;
                    if b + a < bytes.len() {
                        if a == 0 {}
                        b += a;
                    } else {
                        break;
                    }
                }
                font_option
                    .files
                    .insert(text.to_string(), parent.to_str().unwrap().to_string());
            }
            parent.pop();
            if let Some((a, b)) = viter.next() {
                text = a;
                url = b;
            } else {
                break;
            }
        }
        Ok(())
    }
}
