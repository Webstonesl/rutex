use std::error::Error;
use std::fmt::Display;
use std::fs::OpenOptions;
use std::io::Write as _;
use std::path::PathBuf;
use std::{collections::HashMap, sync::Arc};

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Debug)]
pub struct GoogleFontsConfig {
    pub api_key: String,
    pub temp_dir: PathBuf,
}
#[cfg(GOOGLE_API_KEY)]
impl Default for GoogleFontsConfig {
    fn default() -> Self {
        Self {
            api_key: env!("GOOGLE_API_KEY").to_string(),
            temp_dir: Default::default(),
        }
    }
}

use super::{FontProvider, FontReference};
#[derive(Error, Debug)]
pub enum GoogleFontsError {
    #[error("No Google Fonts API key found.")]
    NoGoogleApiConfig,
    #[error("Variant not found")]
    VariantNotFound,
}
const GOOGLE_FONTS_TARGET: &str = "https://www.googleapis.com/webfonts/v1/webfonts";

#[derive(Default, Clone)]
pub struct GoogleFontsCriteria {
    pub family: Option<String>,
    pub subset: Option<String>,
    pub category: Option<String>,
    pub capability: Option<String>,
    pub sort: Option<String>,
}

impl GoogleFontsCriteria {
    pub fn with_family<T: Display>(mut self, family: T) -> Self {
        self.family = Some(family.to_string());
        self
    }
    pub fn with_subset<T: Display>(mut self, subset: T) -> Self {
        self.subset = Some(subset.to_string());
        self
    }
    pub fn with_category<T: Display>(mut self, category: T) -> Self {
        self.category = Some(category.to_string());
        self
    }
    pub fn with_capability<T: Display>(mut self, capability: T) -> Self {
        self.capability = Some(capability.to_string());
        self
    }
    pub fn with_sort<T: Display>(mut self, sort: T) -> Self {
        self.sort = Some(sort.to_string());
        self
    }
    fn to_query_params(&self) -> String {
        let mut result = String::new();
        if let Some(ref family) = self.family {
            result.push_str(&format!("&family={}", family));
        }
        if let Some(ref subset) = self.subset {
            result.push_str(&format!("&subset={}", subset));
        }
        if let Some(ref category) = self.category {
            result.push_str(&format!("&category={}", category));
        }
        if let Some(ref capability) = self.capability {
            result.push_str(&format!("&capability={}", capability));
        }
        if let Some(ref sort) = self.sort {
            result.push_str(&format!("&sort={}", sort));
        }
        result
    }
}

pub struct GoogleFontProvider {
    api_key: String,
    temp_dir: Arc<PathBuf>,
}
impl FontProvider for GoogleFontProvider {
    type Criteria = GoogleFontsCriteria;

    type FontReference = GoogleWebFontSpec;

    fn new(cfg: &crate::config::Config) -> Result<Self, Box<dyn std::error::Error>> {
        if let Some(GoogleFontsConfig {
            ref api_key,
            ref temp_dir,
        }) = cfg.google_fonts_config
        {
            Ok(GoogleFontProvider {
                api_key: api_key.clone(),
                temp_dir: Arc::new(temp_dir.clone()),
            })
        } else {
            Err(Box::new(GoogleFontsError::NoGoogleApiConfig))
        }
    }

    fn find_fonts(
        &mut self,
        criteria: &Self::Criteria,
    ) -> Result<Vec<Self::FontReference>, Box<dyn std::error::Error>> {
        let data = reqwest::blocking::get(format!(
            "{}?key={}{}",
            GOOGLE_FONTS_TARGET,
            self.api_key,
            criteria.to_query_params()
        ))?;
        eprintln!("DATA: {:?}", data);
        let result: GoogleFontResult = data.json()?;

        Ok(result
            .items
            .into_iter()
            .map(|a| a.with_temp_dir(self.temp_dir.clone()))
            .collect())
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct _WebFontSpec {
    family: String,
    variants: Vec<String>,
    subsets: Vec<String>,
    category: String,
    version: String,
    files: HashMap<String, String>,
    menu: String,
}
impl _WebFontSpec {
    fn with_temp_dir(self, temp_dir: Arc<PathBuf>) -> GoogleWebFontSpec {
        let _WebFontSpec {
            family,
            variants,
            subsets,
            category,
            version,
            files,
            menu,
        } = self;
        GoogleWebFontSpec {
            family,
            variants,
            subsets,
            category,
            version,
            files,
            menu,
            temp_dir,
        }
    }
}
#[derive(Clone, Debug)]
pub struct GoogleWebFontSpec {
    pub family: String,
    pub variants: Vec<String>,
    pub subsets: Vec<String>,
    pub category: String,
    pub version: String,
    pub files: HashMap<String, String>,
    pub menu: String,
    pub temp_dir: Arc<PathBuf>,
}

impl FontReference for GoogleWebFontSpec {
    type Variant = Option<String>;

    fn get_font(&self, variant: &Self::Variant) -> Result<PathBuf, Box<dyn Error>> {
        let value = if let Some(a) = variant {
            self.files.get(a)
        } else {
            Some(&self.menu)
        };
        if value.is_none() {
            return Err(GoogleFontsError::VariantNotFound.into());
        }
        let value = value.unwrap();

        let data = reqwest::blocking::get(value)?.error_for_status()?;
        let mut d = self.temp_dir.to_path_buf();
        d.push(PathBuf::from(data.url().path()).file_name().unwrap());
        let mut file = OpenOptions::new().create(true).truncate(true).open(&d)?;
        file.write_all(&data.bytes()?)?;

        Ok(d)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct GoogleFontResult {
    items: Vec<_WebFontSpec>,
}
