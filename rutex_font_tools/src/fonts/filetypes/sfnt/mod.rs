pub mod encodings;
pub mod sfnt_primitives;
pub mod tables;
use sfnt_primitives::*;
use std::{
    any::Any,
    collections::BTreeMap,
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader, Seek},
    path::PathBuf,
};
use tables::{
    common_tables::{NameTable, NameType},
    SFNTType, TableTag,
};
use truetype::TrueType;
pub mod opentype;
pub mod truetype;

use crate::{
    error::{Error, ErrorKind},
    fonts::filetypes::FontTrait,
};

#[derive(Debug, Clone, Copy)]
enum Headers {
    TrueType,
    PostScript,
    OpenType,
}
impl TryFrom<&[u8]> for Headers {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        match value {
            b"true" | &[0, 1, 0, 0] => Ok(Self::TrueType),
            b"typ1" => Ok(Self::PostScript),
            b"OTTO" => Ok(Self::OpenType),
            a => Err(Error::new_with_message(
                ErrorKind::IoError,
                format!(
                    "{:?} is an invalid header",
                    match a.is_ascii() {
                        true => String::from_iter(a.iter().map(|a| *a as char)),
                        false => format!("{:02x?}", a),
                    }
                ),
            )),
        }
    }
}

pub trait SeekAndBufRead: BufRead + Seek {}

impl<T: BufRead + Seek> SeekAndBufRead for T {}

#[derive(Clone, Copy)]
pub struct TableReference {
    tag: TableTag,
    offset: u32,
    length: u32,
    checksum: u32,
}
impl Debug for TableReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TableReference")
            .field("tag", &self.tag)
            .field("offset", &self.offset)
            .field("length", &self.length)
            .field("checksum", &self.checksum)
            .finish()
    }
}
impl PartialEq for TableReference {
    fn eq(&self, other: &Self) -> bool {
        self.tag == other.tag
    }
}
impl Eq for TableReference {}
impl PartialOrd for TableReference {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(Ord::cmp(&self, &other))
    }
}
impl Ord for TableReference {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.tag.cmp(&other.tag)
    }
}

pub struct SFNTFile {
    path: Option<PathBuf>,
    file: Box<dyn SeekAndBufRead>,
    tables: BTreeMap<TableTag, TableReference>, //  BufReader<File>,
}
impl SFNTFile {
    pub fn new(path: Option<PathBuf>, file: File) -> Self {
        Self {
            path,
            file: Box::new(BufReader::new(file)),
            tables: BTreeMap::new(),
        }
    }
    pub fn parse(mut self) -> Result<Box<dyn SFNTSuperFont>, Error> {
        self.file.seek(std::io::SeekFrom::Start(0))?;
        let header: [u8; 4] = u8::read_from_into_array(&mut self.file)?;
        let header = Headers::try_from(header.as_slice())?;
        let num_tables = u16::read_from(&mut self.file)?;
        let _ = u16::read_from(&mut self.file)?;
        let _ = u16::read_from(&mut self.file)?;
        let _ = u16::read_from(&mut self.file)?;
        for _ in 0..num_tables {
            let tag = TableTag::read_from(&mut self.file)?;
            let checksum = u32::read_from(&mut self.file)?;
            let offset = u32::read_from(&mut self.file)?;
            let length = u32::read_from(&mut self.file)?;
            debug_assert!(self
                .tables
                .insert(
                    tag,
                    TableReference {
                        tag,
                        offset,
                        checksum,
                        length
                    }
                )
                .is_none());
        }

        Ok(match header {
            Headers::TrueType => Box::new(TrueType::new(self)?),
            Headers::PostScript => todo!(),
            Headers::OpenType => Box::new(opentype::OpenType::new(self)?),
        })
    }
    pub fn iter_tables(&self) -> Vec<TableReference> {
        self.tables.values().copied().collect()
    }
    pub fn get_table(&self, table: &TableTag) -> Option<TableReference> {
        dbg!(&self.tables).get(dbg!(table)).copied()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct SFNTReader {}

impl SFNTReader {
    const EXTENSIONS: &'static [&'static str] = &[];

    pub fn read_from_file(
        file: std::fs::File,
    ) -> Result<Box<(dyn SFNTSuperFont)>, crate::error::Error> {
        SFNTFile::new(None, file).parse()
    }
}
pub trait SFNTFont {
    fn get_table(&mut self, tag: &TableTag) -> Result<Box<dyn Any>, Error>;
}
pub trait SFNTSuperFont: SFNTFont + FontTrait {}
impl<T: SFNTFont + FontTrait> SFNTSuperFont for T {}
impl<T: SFNTFont> FontTrait for T {
    fn get_name(
        &mut self,
        language: &dyn tables::common_tables::LanguageType,
    ) -> Result<String, Error> {
        let table = self.get_table(&TableTag(*b"name"))?;
        let table: Box<NameTable> = table.downcast().unwrap();
        for a in table.0.into_iter() {
            if let NameType::FontFamily | NameType::PrefferedFamily | NameType::FullName = a.name_id
            {
            } else {
                continue;
            }
            if !a.platform.matches_language(language) {
                continue;
            }
            if let crate::fonts::filetypes::sfnt::tables::common_tables::NameValue::Clean(a) =
                a.value
            {
                return Ok(a);
            }
        }
        Err(Error::new_with_message(
            ErrorKind::KeyError,
            "No name found for language",
        ))

        // dbg!(table);
    }
}

#[test]
fn test() {
    let a = "/System/Library/Fonts/Monaco.ttf";
    let mut d = SFNTReader::read_from_file(std::fs::File::open(a).unwrap()).unwrap();
    dbg!(d.get_name(&Language::EnglishIntl));
}
#[cfg(test)]
use crate::{fonts::providers::osfonts::OSFontProvider, localization::Language};
#[cfg(test)]
use std::fs::OpenOptions;
#[test]
fn test_mega() {
    let f = OSFontProvider::default().unwrap();
    println!("{:?}", f);
    for file in f.files {
        if file.extension().unwrap() == "ttf" {
            let mut f =
                SFNTReader::read_from_file(OpenOptions::new().read(true).open(file).unwrap())
                    .unwrap();
            dbg!(f.get_name(&Language::EnglishIntl)).unwrap();
        }
        // println!("{:?}", file);
    }
}
