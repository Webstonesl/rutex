#![allow(missing_docs)]
use std::{
    any::Any,
    error::Error,
    io::{Read, Seek},
};

use cmap::CMAPTable;
use glyf::GlyphTable;
use head::FontHeader;
use hhea::HorizontalHeaderTable;
use hmtx::HorizontalMetrics;
use kern::KerningTable;
use loca::GlyphOffsetTable;
use maxp::MemoryManagementTable;
use name::NameTable;
use os2::OS2Table;
use post::PostTable;

use super::{SFNTFile, index::TableTagInner};

pub trait SFNTTable: Any {
    fn into(self) -> AnySFNTTable;
}
#[derive(Clone)]
pub enum AnySFNTTable {
    CharacterMap(Box<CMAPTable>),
    NameTable(Box<NameTable>),
    PostTable(Box<PostTable>),
    HorizontalHeaders(Box<HorizontalHeaderTable>),
    HorizontalMetrics(Box<HorizontalMetrics>),
    OS2Table(Box<OS2Table>),
    MemoryManagement(Box<MemoryManagementTable>),
    Glyphs(Box<GlyphTable>),
    GlyphOffset(GlyphOffsetTable),
    FontHeader(Box<FontHeader>),
    KerningTable(Box<KerningTable>),
}

pub trait ReadableSFNTTable: Sized + SFNTTable {
    const TAGS: &[&TableTagInner];
    fn read<R: Read + Seek>(read: &mut R, length: u32) -> Result<Self, Box<dyn Error>>;
}
pub trait DependentSFNTTable: Sized + SFNTTable {
    const TAGS: &[&TableTagInner];
    fn read(file: &mut SFNTFile) -> Result<Self, Box<dyn Error>>;
    fn read_option(file: &mut SFNTFile) -> Result<Option<Self>, Box<dyn Error>> {
        Self::read(file).map(Some)
    }
    fn tags_to_string() -> String {
        format!(
            "[{}]",
            Self::TAGS
                .iter()
                .map(|a| str::from_utf8(*a).unwrap().to_string())
                .reduce(|a, b| format!("{a}, {b}"))
                .unwrap_or("".to_string())
        )
    }
}
impl<T: ReadableSFNTTable> DependentSFNTTable for T {
    const TAGS: &[&TableTagInner] = <Self as ReadableSFNTTable>::TAGS;

    fn read(file: &mut SFNTFile) -> Result<Self, Box<dyn Error>> {
        let t = file.seek_table(Self::TAGS)?;
        <Self as ReadableSFNTTable>::read(&mut file.file, t.length)
    }
    fn read_option(file: &mut SFNTFile) -> Result<Option<Self>, Box<dyn Error>> {
        let t = file.seek_table_optional(Self::TAGS)?;
        Ok(if let Some(t) = t {
            Some(<Self as ReadableSFNTTable>::read(&mut file.file, t.length)?)
        } else {
            None
        })
    }
}
pub enum Platform {
    Unicode,
    MacOs,
    Microsoft,
}
pub struct Language(pub u16);
pub struct SFNTString {
    pub platform: Platform,
    pub language: Language,
}

pub mod acnt;
pub mod ankr;
pub mod avar;
pub mod bdat;
pub mod bhed;
pub mod bloc;
pub mod bsln;
pub mod cmap;
pub mod cvar;
pub mod cvt;
pub mod ebsc;
pub mod fdsc;
pub mod feat;
pub mod fmtx;
pub mod fond;
pub mod fpgm;
pub mod fvar;
pub mod gasp;
pub mod gcid;
pub mod glyf;
pub mod gvar;
pub mod hdmx;
pub mod head;
pub mod hhea;
pub mod hmtx;
pub mod just;
pub mod kern;
pub mod kerx;
pub mod lcar;
pub mod loca;
pub mod ltag;
pub mod maxp;
pub mod meta;
pub mod mort;
pub mod morx;
pub mod name;
pub mod opbd;
pub mod os2;
pub mod post;
pub mod prep;
pub mod prop;
pub mod sbix;
pub mod trak;
pub mod vhea;
pub mod vmtx;
pub mod xref;
pub mod zapf;
