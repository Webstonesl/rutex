use clap::{Parser, Subcommand};
use rutex_font_tools::config::{self, Config};
use std::error::Error;
use std::fs::OpenOptions;
use std::{
    env::{self, temp_dir},
    path::PathBuf,
};
#[derive(Subcommand)]
pub enum Source {
    /// Gets the font from a provided file
    File { source: PathBuf },
    /// Searches for the font in the os
    #[cfg(feature = "os_fonts")]
    Os { name: String },
    /// Gets the font from online
    #[cfg(feature = "online_fonts")]
    Online { url: reqwest::Url },
}
impl Source {
    fn require_td(&self) -> bool {
        match self {
            Source::Online { .. } => true,
            _ => false,
        }
    }
}
#[derive(Parser)]
pub struct Arguments {
    word: String,
    #[command(subcommand)]
    font: Source,
}
use thiserror::Error;
#[derive(Error, Debug)]
pub enum ErrorType {
    #[error("Could not find font {0}")]
    CouldNotFindFont(String),
}
pub use rutex_font_tools::types::sfnt_types::{
    SFNTFile,
    tables::name::{NameID, NameTable},
};
#[cfg(feature = "os_fonts")]
pub fn find_font(cfg: &config::Config, name: String) -> Result<SFNTFile, Box<dyn Error>> {
    use std::fs::OpenOptions;

    use rutex_font_tools::providers::{
        FontProvider, FontReference, filebasedprovider::FileFontReference, os::OSProvider,
    };

    let mut provider = OSProvider::new(cfg)?;
    let fr = provider.find_fonts(&())?;
    for fr in fr {
        let path = fr.get_font(&())?;
        let mut file: SFNTFile = OpenOptions::new().read(true).open(&path)?.try_into()?;
        let name_table = file.get_table::<NameTable>()?;
        if let Some(font_name) = name_table.get_name(NameID::FullFontName, None)? {
            if font_name == name {
                return Ok(file);
            }
        }
        // dbg!(path);
    }
    Err(ErrorType::CouldNotFindFont(name).into())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Arguments = Arguments::parse();
    let td = temp_dir();
    let td_existed = td.exists();
    let td_created = !td_existed && args.font.require_td();
    if td_created {
        std::fs::create_dir(&td)?;
    }
    let config = Config::new(&td);

    let file = match args.font {
        Source::File { source } => OpenOptions::new().read(true).open(source)?.try_into()?,
        Source::Os { name } => find_font(&config, name)?,
        Source::Online { url } => download_font(&config, url)?,
    };


    if td_created {
        std::fs::remove_dir_all(&td)?;
    }
    // let config = config::Config::default();
    Ok(())
}

#[cfg(feature = "online_fonts")]
fn download_font(config: &Config, url: reqwest::Url) -> Result<SFNTFile, Box<dyn Error>> {
    use std::{fs, io::Write};
    let response = reqwest::blocking::get(url.clone())?;
    let mut path = config.temp_dir.as_ref().clone();
    let filetype = response
        .headers()
        .get("content-type")
        .unwrap()
        .to_str()?
        .split('/')
        .last()
        .unwrap();
    let path = path.with_file_name(format!("file.{filetype}"));
    let bytes = response.bytes()?;

    let mut file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&path)?;
    file.write_all(&bytes)?;
    drop(file);
    OpenOptions::new().read(true).open(&path)?.try_into()
}

// https://ttfonts.net/sfonts/0/07558_CenturyGothic.ttf
