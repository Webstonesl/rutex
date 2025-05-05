use std::{
    collections::{BTreeMap, btree_map::Keys},
    fmt::Debug,
    io,
    mem::transmute,
};

use thiserror::Error;

use super::{SFNTReadable, SFNTStream};
#[allow(missing_docs)]
#[derive(Error, Debug)]
pub enum SFNTError {
    #[error("Invalid header found ({0:?})")]
    InvalidHeader([u8; 4]),
    #[error("Tag Not found for <{0}")]
    TagNotFound(&'static str),
    #[error("{0}")]
    InvalidVersionTag(String),
    #[error("Invalid platform pumber {0}")]
    InvalidPlatformNumber(u16),
    #[error("Invalid encoding number ({1}) for {0} ")]
    InvalidPlatformSpecificNumber(&'static str, u16),
    #[error("Invalid language number ({1}) for {0} ")]
    InvalidLanguageNumber(&'static str, u16),
    #[error("Parsing error")]
    ParsingError,
    #[error("Could not found codepoint: {0}")]
    MappingError(u16),
    #[error("Invalid Name ID: {0}")]
    InvalidNameID(u16),
    #[error("Parsing from {0} is not allowed.")]
    UnsupportedPlatform(String),
    #[error("Invalid cmap type: {0}")]
    InvalidCMAPTable(u16),
    #[error("{0} is not yet implemented")]
    NotYetImplemented(String),
    #[error("{0} {1:?} is not found")]
    SomethingIsNotFound(&'static str, String),
    #[error("Invalid {0}: {1:?}")]
    Invalid(&'static str, String),
}

impl SFNTError {
    /// A helpermethod to get a not yet implemented error
    pub fn not_yet_implemented<T: ToString>(t: T) -> Self {
        SFNTError::NotYetImplemented(t.to_string())
    }
}

/// Different Type of SFNT Scalar Types
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy)]
pub enum SFNTScalarType {
    TrueType,
    PostScript,
    OpenType,
}
impl SFNTReadable for SFNTScalarType {
    fn read_from<R: io::Read + ?Sized + io::Seek>(
        mut read: &mut R,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let x: [u8; 4] = read.get_prim_array()?;
        match &x {
            b"true" | &[0x00, 0x001, 0x00, 0x00] => Ok(Self::TrueType),
            b"type" => Ok(Self::PostScript),
            b"OTTO" => Ok(Self::OpenType),
            a => Err(SFNTError::InvalidHeader(*a).into()),
        }
    }
}
pub type TableTagInner = [u8; 4];
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct TableTag(pub TableTagInner);

impl Debug for TableTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.is_ascii() {
            write!(f, "{:?}", str::from_utf8(&self.0).unwrap())
        } else {
            write!(f, "{:x?}", self.0)
        }
    }
}
impl SFNTReadable for TableTag {
    fn read_from<R: io::Read + ?Sized + io::Seek>(
        mut read: &mut R,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self(read.get_prim_array()?))
    }
}
impl AsRef<[u8; 4]> for TableTag {
    fn as_ref(&self) -> &[u8; 4] {
        &self.0
    }
}
/// A struct representing a table in an SFNT table
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TableReference {
    /// The label of the tag.
    pub tag: TableTag,
    /// Checksum of the table reference.
    pub checksum: u32,
    /// Offset from the start of the file
    pub offset: u32,
    /// Length of the table
    pub length: u32,
}

impl SFNTReadable for TableReference {
    fn read_from<R: io::Read + ?Sized + io::Seek>(
        mut read: &mut R,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(TableReference {
            tag: read.get()?,
            checksum: read.get_prim()?,
            offset: read.get_prim()?,
            length: read.get_prim()?,
        })
    }
}

#[derive(Clone, Debug)]
pub struct TableTagMap<T>(BTreeMap<TableTag, T>);
impl<T> Default for TableTagMap<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}
impl TableTagMap<TableReference> {
    #[inline(always)]
    pub fn push(&mut self, tr: TableReference) {
        self.0.insert(tr.tag, tr);
    }
    pub fn keys(&self) -> Keys<TableTag, TableReference> {
        self.0.keys()
    }
}
impl<T: Copy> TableTagMap<T> {
    pub fn get(&self, b: &TableTagInner) -> Option<T> {
        self.0
            .get(unsafe { transmute::<&[u8; 4], &TableTag>(b) })
            .copied()
    }
}

impl Extend<TableReference> for TableTagMap<TableReference> {
    fn extend<T: IntoIterator<Item = TableReference>>(&mut self, iter: T) {
        self.0.extend(iter.into_iter().map(|a| (a.tag, a)))
    }
}
impl<T> AsRef<BTreeMap<TableTag, T>> for TableTagMap<T> {
    fn as_ref(&self) -> &BTreeMap<TableTag, T> {
        &self.0
    }
}
impl FromIterator<TableReference> for TableTagMap<TableReference> {
    fn from_iter<T: IntoIterator<Item = TableReference>>(iter: T) -> Self {
        Self(iter.into_iter().map(|a| (a.tag, a)).collect())
    }
}
#[derive(Clone, Debug)]
pub struct SFNTHeader {
    pub scalar: SFNTScalarType,
    pub tables: TableTagMap<TableReference>,
}
impl SFNTReadable for SFNTHeader {
    fn read_from<R: io::Read + ?Sized + io::Seek>(
        mut read: &mut R,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        read.seek(io::SeekFrom::Start(0))?;
        let scalar: SFNTScalarType = read.get()?;
        let table_count: u16 = read.get_prim()?;
        for _ in 0..3 {
            let _: u16 = read.get_prim()?;
        }
        let mut tables = TableTagMap::default();
        for _ in 0..table_count {
            tables.push(read.get()?);
        }
        Ok(Self { scalar, tables })
    }
}
