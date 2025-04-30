//! # `name`-table
//! `name` implemented as from the various specifications:
//!  - [Microsoft](https://learn.microsoft.com/en-us/typography/opentype/spec/name)
//!  - [Apple](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6name.html)
//!
pub mod macintosh;
pub mod microsoft;
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt::Debug;
use std::io::{Read, Seek};
use std::ops::{BitAnd, BitOr, Deref, DerefMut, Range};

use microsoft::LanguageTag;

use super::{ReadableSFNTTable, SFNTTable};
use crate::string_encoding::{Decoder, UTF16BE_DECODER};
use crate::types::sfnt_types::index::{SFNTError, SFNTScalarType};
use crate::types::sfnt_types::{SFNTReadable, SFNTStream};

// fn string_from_utf16be(value: &[u8]) -> Result<String, Box<dyn Error>> {
//     if value.len() % 2 != 0 {
//         return Err(SFNTError::ParsingError.into());
//     }
//     let value = value
//         .chunks(2)
//         .map(|a| u16::from_be_bytes(a.try_into().unwrap()))
//         .collect::<Vec<_>>();
//     Ok(String::from_utf16(&value)?)
// }

#[derive(Debug, Clone, Copy)]
pub enum DiscoverableValue<Key: Sized + Eq, V: Sized> {
    Undiscovered(Key),
    Discovered(V),
}

#[derive(Clone, Copy, Debug)]
pub enum PlatformID {
    Unicode = 0,
    Macintosh = 1,
    Microsoft = 3,
}
impl TryFrom<u16> for PlatformID {
    type Error = SFNTError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PlatformID::Unicode),
            1 => Ok(PlatformID::Macintosh),
            3 => Ok(PlatformID::Microsoft),
            a => Err(SFNTError::InvalidPlatformNumber(a)),
        }
    }
}
impl SFNTReadable for PlatformID {
    fn read_from<R: Read + ?Sized + std::io::Seek>(
        mut read: &mut R,
    ) -> Result<Self, Box<dyn Error>> {
        Ok(read.get_prim::<2, u16>()?.try_into()?)
    }
}
#[derive(Clone, Debug)]
pub enum Platform {
    Unicode(u16),
    Macintosh(macintosh::ScriptCode, macintosh::LanguageCode),
    Microsoft(microsoft::ScriptCode, microsoft::LanguageTag),
}
impl PlatformID {
    fn read_for_name<R: Read + Seek + ?Sized>(
        &self,
        mut read: &mut R,
    ) -> Result<Platform, Box<dyn Error>> {
        let [script, language]: [u16; 2] = read.get_prim_array()?;
        match self {
            PlatformID::Unicode => {
                if (matches!(script, 0..5) && language == 0) {
                    Ok(Platform::Unicode(script))
                } else if language == 0 {
                    Err(SFNTError::InvalidPlatformSpecificNumber("Unicode", script).into())
                } else {
                    Err(SFNTError::InvalidLanguageNumber("Unicode", language).into())
                }
            }
            PlatformID::Macintosh => Ok(Platform::Macintosh(script.try_into()?, language.into())),
            PlatformID::Microsoft => Ok(Platform::Microsoft(
                script.try_into()?,
                language.try_into()?,
            )),
        }
    }
}
/// Different types of names as in the specifications.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum NameID {
    CopyrightNotice,
    FontFamilyName,
    FontSubfamilyName,
    UniqueFontIdentifier,
    FullFontName,
    VersionString,
    PostscriptName,
    Trademark,
    ManufacturerName,
    DesignerName,
    Description,
    VendorURL,
    DesignerURL,
    LicenseDescription,
    LicenseInfoURL,
    Reserved,
    TypographicFamilyName,
    TypographicSubfamilyName,
    CompatibleFull,
    SampleText,
    PostscriptCIDName,
    WWSFamilyName,
    WWSSubfamilyName,
    LightBackgroundPallete,
    DarkbackgroundPallete,
    VariationsPostScriptNamePrefix,
    Unknown(u16),
}
#[derive(Default)]
pub struct NameIDs(BTreeSet<NameID>);
impl Deref for NameIDs {
    type Target = BTreeSet<NameID>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for NameIDs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl AsRef<BTreeSet<NameID>> for NameIDs {
    fn as_ref(&self) -> &BTreeSet<NameID> {
        &self.0
    }
}
impl AsMut<BTreeSet<NameID>> for NameIDs {
    fn as_mut(&mut self) -> &mut BTreeSet<NameID> {
        &mut self.0
    }
}
impl BitOr for NameIDs {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(rhs.0.union(&rhs.0).copied().collect())
    }
}
impl BitAnd for NameIDs {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(rhs.0.intersection(&rhs.0).copied().collect())
    }
}
impl PartialEq for NameIDs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl PartialOrd for NameIDs {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl SFNTReadable for NameID {
    fn read_from<R: Read + ?Sized + Seek>(mut read: &mut R) -> Result<Self, Box<dyn Error>> {
        Ok(match read.get_prim::<2, u16>()? {
            0 => NameID::CopyrightNotice,
            1 => NameID::FontFamilyName,
            2 => NameID::FontSubfamilyName,
            3 => NameID::UniqueFontIdentifier,
            4 => NameID::FullFontName,
            5 => NameID::VersionString,
            6 => NameID::PostscriptName,
            7 => NameID::Trademark,
            8 => NameID::ManufacturerName,
            9 => NameID::DesignerName,
            10 => NameID::Description,
            11 => NameID::VendorURL,
            12 => NameID::DesignerURL,
            13 => NameID::LicenseDescription,
            14 => NameID::LicenseInfoURL,
            15 => NameID::Reserved,
            16 => NameID::TypographicFamilyName,
            17 => NameID::TypographicSubfamilyName,
            18 => NameID::CompatibleFull,
            19 => NameID::SampleText,
            20 => NameID::PostscriptCIDName,
            21 => NameID::WWSFamilyName,
            22 => NameID::WWSSubfamilyName,
            23 => NameID::LightBackgroundPallete,
            24 => NameID::DarkbackgroundPallete,
            25 => NameID::VariationsPostScriptNamePrefix,
            a => NameID::Unknown(a),
        })
    }
}
#[derive(Clone)]
pub struct NameRecord {
    pub platform: Platform,
    pub name_id: NameID,
    pub location: Range<usize>,
}
impl NameRecord {
    pub fn data_len(&self) -> usize {
        self.location.len()
    }
}
impl Debug for NameRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NameRecord")
            .field("platform", &self.platform)
            .field("name_id", &self.name_id)
            .field_with("location", |f| {
                write!(
                    f,
                    "(0x{:x} .. 0x{:x})",
                    self.location.start, self.location.end
                )
            })
            .finish()
    }
}
impl NameRecord {
    unsafe fn offset_unchecked(&self) -> usize {
        self.location.start
    }
    pub fn set_lang_tag(&mut self, map: &BTreeMap<u16, String>) -> Result<(), Box<dyn Error>> {
        if let Platform::Microsoft(.., b) = &mut self.platform {
            let key = if let microsoft::LanguageTag::Unset(k) = b {
                *k
            } else {
                return Ok(());
            };
            if let Some(a) = map.get(&key) {
                *b = LanguageTag::Variable(a.clone());
            } else {
                return Err(SFNTError::InvalidLanguageNumber("Microsoft", key).into());
            }
        }
        Ok(())
    }
    #[allow(non_contiguous_range_endpoints)]
    pub fn get_value(&self, mut buf: &[u8]) -> Result<String, Box<dyn Error>> {
        let NameRecord {
            platform, location, ..
        } = self;
        buf = &buf[location.clone()];

        match platform {
            Platform::Unicode(_) => UTF16BE_DECODER.decode(buf),
            Platform::Macintosh(macintosh::ScriptCode::Roman, _) => {
                macintosh::ROMAN_DECODER.decode(buf)
            }

            Platform::Microsoft(microsoft::ScriptCode::PRC, _) => {
                microsoft::CODE_PAGE_936.decode(buf)
            }
            Platform::Microsoft(microsoft::ScriptCode::Big5, _) => {
                microsoft::CODE_PAGE_950.decode(buf)
            }
            Platform::Microsoft(microsoft::ScriptCode::Wansung, _) => {
                microsoft::CODE_PAGE_949.decode(buf)
            }
            Platform::Microsoft(..) => UTF16BE_DECODER.decode(buf),
            Platform::Macintosh(a, _) => {
                return Err(SFNTError::UnsupportedPlatform(format!("Macintosh {:?}", a)).into());
            }
        }
        .map_err(Into::into)
    }
    pub fn get_language(&self) -> Option<&str> {
        match &self.platform {
            Platform::Unicode(_) => None,
            Platform::Macintosh(.., language_code) => Some(language_code.to_lang_string()),
            Platform::Microsoft(.., language_tag) => match language_tag {
                LanguageTag::Unset(_) => None,
                LanguageTag::Variable(s) => Some(s),
                LanguageTag::Preset(s) => Some(s),
            },
        }
    }

    pub fn is_supported(&self) -> bool {
        match &self.platform {
            Platform::Unicode(_) => true,
            Platform::Macintosh(macintosh::ScriptCode::Roman, _) => true,
            Platform::Macintosh(_, _) => false,
            Platform::Microsoft(_, _) => true,
        }
    }
}

impl SFNTReadable for NameRecord {
    fn read_from<R: std::io::Read + ?Sized + std::io::Seek>(
        mut read: &mut R,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let platform_id: PlatformID = read.get()?;
        let platform = platform_id.read_for_name(read)?;
        let name_id = read.get()?;
        let length = read.get_prim::<2, u16>()? as usize;
        let offset = read.get_prim::<2, u16>()? as usize;
        let value = offset..(length + offset);
        Ok(Self {
            platform,
            name_id,
            location: value,
        })
    }
}

#[derive(Clone)]
pub struct NameTable {
    records: Vec<NameRecord>,
    buf: Vec<u8>,
}
impl AsRef<Vec<NameRecord>> for NameTable {
    fn as_ref(&self) -> &Vec<NameRecord> {
        &self.records
    }
}
impl Debug for NameTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NameTable")
            .field("records", &self.records)
            .field("buffer len", &self.buf.len())
            .finish()
    }
}
impl NameTable {
    pub fn get_name(
        &self,
        name_id: NameID,
        language: Option<&str>,
    ) -> Result<Option<String>, Box<dyn Error>> {
        for record in self.records.iter() {
            if record.name_id != name_id {
                continue;
            }

            if language.is_some() && record.get_language() == language {
                continue;
            }
            return record.get_value(&self.buf).map(Some);
        }
        Ok(None)
    }
    pub fn combinations(&self) -> impl Iterator<Item = (NameID, Option<&str>)> {
        self.records.iter().map(|a| (a.name_id, a.get_language()))
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
impl SFNTTable for NameTable {
    fn into(self) -> super::AnySFNTTable {
        super::AnySFNTTable::NameTable(Box::new(self))
    }
}

impl ReadableSFNTTable for NameTable {
    const TAGS: &[&crate::types::sfnt_types::index::TableTagInner] = &[b"name"];

    fn read<R: std::io::Read + std::io::Seek>(
        read: &mut R,
        length: u32,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let start = read.stream_position()?;
        let version: u16 = read.get_prim::<2, u16>()?;
        let count: u16 = read.get_prim()?;
        let offset = read.get_prim::<2, u16>()? as u64;
        let length = (length as u64) - offset;
        let offset = offset + start;

        let mut records = Vec::new();
        for _ in 0..count {
            let record = NameRecord::read_from(read)?;
            if record.is_supported() {
                records.push(record);
            }
        }
        records.sort_by_key(|a| unsafe { a.offset_unchecked() });
        let mut map = Vec::new();
        match version {
            0 => {}
            1 => {
                let lang_count: u16 = read.get_prim()?;

                for i in 0..lang_count {
                    let [length, rel_offset]: [u16; 2] = read.get_prim_array()?;
                    map.push((i, offset + (rel_offset as u64), length as usize));
                    dbg!(offset, length);
                }
            }
            a => {
                return Err(Box::new(SFNTError::InvalidVersionTag(format!(
                    "Invalid version tag {a:?} for Name Table"
                ))));
            }
        }
        let mut language_map = BTreeMap::new();
        for (nr, offset, length) in map.into_iter() {
            read.seek(std::io::SeekFrom::Start(offset))?;
            let mut buf = vec![0u8; length];
            read.read_exact(&mut buf)?;
            language_map.insert(nr, UTF16BE_DECODER.decode(&buf)?);
        }
        for r in records.iter_mut() {
            r.set_lang_tag(&language_map)?;
        }
        read.seek(std::io::SeekFrom::Start(offset))?;
        let mut buf = vec![0u8; length as usize];
        read.read_exact(&mut buf)?;

        Ok(NameTable { records, buf })
    }
}
