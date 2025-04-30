#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum MacintoshLanguageCode {
    English = 0,
    French = 1,
    German = 2,
    Italian = 3,
    Dutch = 4,
    Swedish = 5,
    Spanish = 6,
    Danish = 7,
    Portuguese = 8,
    Norwegian = 9,
    Hebrew = 10,
    Japanese = 11,
    Arabic = 12,
    Finnish = 13,
    Greek = 14,
    Icelandic = 15,
    Maltese = 16,
    Turkish = 17,
    Croatian = 18,
    TraditionalChinese = 19,
    Urdu = 20,
    Hindi = 21,
    Thai = 22,
    Korean = 23,
    Lithuanian = 24,
    Polish = 25,
    Hungarian = 26,
    Estonian = 27,
    Latvian = 28,
    Sami = 29,
    Faroese = 30,
    FarsiOrPersian = 31,
    Russian = 32,
    SimplifiedChinese = 33,
    Flemish = 34,
    IrishGaelic = 35,
    Albanian = 36,
    Romanian = 37,
    Czech = 38,
    Slovak = 39,
    Slovenian = 40,
    Yiddish = 41,
    Serbian = 42,
    Macedonian = 43,
    Bulgarian = 44,
    Ukrainian = 45,
    Byelorussian = 46,
    Uzbek = 47,
    Kazakh = 48,
    AzerbaijaniInCyrillicScript = 49,
    AzerbaijaniInArabicScript = 50,
    Armenian = 51,
    Georgian = 52,
    Moldavian = 53,
    Kirghiz = 54,
    Tajiki = 55,
    Turkmen = 56,
    MongolianInMongolianScript = 57,
    MongolianInCyrillicScript = 58,
    Pashto = 59,
    Kurdish = 60,
    Kashmiri = 61,
    Sindhi = 62,
    Tibetan = 63,
    Nepali = 64,
    Sanskrit = 65,
    Marathi = 66,
    Bengali = 67,
    Assamese = 68,
    Gujarati = 69,
    Punjabi = 70,
    Oriya = 71,
    Malayalam = 72,
    Kannada = 73,
    Tamil = 74,
    Telugu = 75,
    Sinhalese = 76,
    Burmese = 77,
    Khmer = 78,
    Lao = 79,
    Vietnamese = 80,
    Indonesian = 81,
    Tagalog = 82,
    MalayInRomanScript = 83,
    MalayInArabicScript = 84,
    Amharic = 85,
    Tigrinya = 86,
    Galla = 87,
    Somali = 88,
    Swahili = 89,
    KinyarwandaOrRuanda = 90,
    Rundi = 91,
    NyanjaOrChewa = 92,
    Malagasy = 93,
    Esperanto = 94,
    Welsh = 128,
    Basque = 129,
    Catalan = 130,
    Latin = 131,
    Quechua = 132,
    Guarani = 133,
    Aymara = 134,
    Tatar = 135,
    Uighur = 136,
    Dzongkha = 137,
    JavaneseInRomanScript = 138,
    SundaneseInRomanScript = 139,
    Galician = 140,
    Afrikaans = 141,
    Breton = 142,
    Inuktitut = 143,
    ScottishGaelic = 144,
    ManxGaelic = 145,
    IrishGaelicWithDotAbove = 146,
    Tongan = 147,
    PolytonicGreek = 148,
    Greenlandic = 149,
    AzerbaijaniInRomanScript = 150,
    Unknown = 65535,
}
impl From<u16> for MacintoshLanguageCode {
    fn from(value: u16) -> Self {
        match value {
            0..95 | 128..150 => unsafe {
                // Safety: types are the same sizes and are checked for validity
                transmute::<u16, MacintoshLanguageCode>(value)
            },
            _ => Self::Unknown,
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub struct WindowsLCID(&'static str);
include!("windowslcid.rs");
const WINDOWS_UNKNOWN_LCID: &str = "Unknown Type";

impl From<u16> for WindowsLCID {
    fn from(value: u16) -> Self {
        if let Ok(index) = WINDOWS_LCID.binary_search_by(|(a, _)| a.cmp(&value)) {
            WindowsLCID(WINDOWS_LCID[index].1)
        } else {
            WindowsLCID(WINDOWS_UNKNOWN_LCID)
        }
    }
}

#[repr(u16)]
#[derive(Clone, Copy, Debug)]
pub enum MacintoshPSID {
    Roman,
    Japanese,
    TraditionalChinese,
    Korean,
    Arabic,
    Hebrew,
    Greek,
    Russian,
    RSymbol,
    Devanagari,
    Gurmukhi,
    Gujarati,
    Oriya,
    Bengali,
    Tamil,
    Telugu,
    Kannada,
    Malayalam,
    Sinhalese,
    Burmese,
    Khmer,
    Thai,
    Laotian,
    Georgian,
    Armenian,
    SimplifiedChinese,
    Tibetan,
    Mongolian,
    Geez,
    Slavic,
    Vietnamese,
    Sindhi,
    Uninterpreted,
}

impl TryFrom<u16> for MacintoshPSID {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MacintoshPSID::Roman),
            1 => Ok(MacintoshPSID::Japanese),
            2 => Ok(MacintoshPSID::TraditionalChinese),
            3 => Ok(MacintoshPSID::Korean),
            4 => Ok(MacintoshPSID::Arabic),
            5 => Ok(MacintoshPSID::Hebrew),
            6 => Ok(MacintoshPSID::Greek),
            7 => Ok(MacintoshPSID::Russian),
            8 => Ok(MacintoshPSID::RSymbol),
            9 => Ok(MacintoshPSID::Devanagari),
            10 => Ok(MacintoshPSID::Gurmukhi),
            11 => Ok(MacintoshPSID::Gujarati),
            12 => Ok(MacintoshPSID::Oriya),
            13 => Ok(MacintoshPSID::Bengali),
            14 => Ok(MacintoshPSID::Tamil),
            15 => Ok(MacintoshPSID::Telugu),
            16 => Ok(MacintoshPSID::Kannada),
            17 => Ok(MacintoshPSID::Malayalam),
            18 => Ok(MacintoshPSID::Sinhalese),
            19 => Ok(MacintoshPSID::Burmese),
            20 => Ok(MacintoshPSID::Khmer),
            21 => Ok(MacintoshPSID::Thai),
            22 => Ok(MacintoshPSID::Laotian),
            23 => Ok(MacintoshPSID::Georgian),
            24 => Ok(MacintoshPSID::Armenian),
            25 => Ok(MacintoshPSID::SimplifiedChinese),
            26 => Ok(MacintoshPSID::Tibetan),
            27 => Ok(MacintoshPSID::Mongolian),
            28 => Ok(MacintoshPSID::Geez),
            29 => Ok(MacintoshPSID::Slavic),
            30 => Ok(MacintoshPSID::Vietnamese),
            31 => Ok(MacintoshPSID::Sindhi),
            32 => Ok(MacintoshPSID::Uninterpreted),
            _ => Err(Error::new_with_message(
                ErrorKind::ArgumentError,
                format!("Illegal Macintosh Platform Specific identifier {value:?}"),
            )),
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub enum MicrosoftPSID {
    Symbol = 0,
    UnicodeBMP = 1,
    ShiftJIS = 2,
    PRC = 3,
    Big5 = 4,
    Wansung = 5,
    Johab = 6,
    Reserved = 7,
    UnicodeFullRepertoire = 10,
}
impl TryFrom<u16> for MicrosoftPSID {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MicrosoftPSID::Symbol),
            1 => Ok(MicrosoftPSID::UnicodeBMP),
            2 => Ok(MicrosoftPSID::ShiftJIS),
            3 => Ok(MicrosoftPSID::PRC),
            4 => Ok(MicrosoftPSID::Big5),
            5 => Ok(MicrosoftPSID::Wansung),
            6 => Ok(MicrosoftPSID::Johab),
            7 => Ok(MicrosoftPSID::Reserved),
            10 => Ok(MicrosoftPSID::UnicodeFullRepertoire),
            _ => Err(Error::new_with_message(
                ErrorKind::ArgumentError,
                format!("{value:?} illegal Microsoft Platform Specifiic ID"),
            )),
        }
    }
}
#[derive(Clone, Copy, Debug)]
#[allow(dead_code)]
pub enum Platform {
    Unicode(u16),
    Macintosh(MacintoshPSID, MacintoshLanguageCode),
    Microsoft(MicrosoftPSID, WindowsLCID),
}
impl Platform {
    pub fn matches_language(&self, language: &dyn LanguageType) -> bool {
        match self {
            Platform::Macintosh(_, macintosh_language_code) => {
                if let Some(a) = language.macos_language_code() {
                    &a == macintosh_language_code
                } else {
                    false
                }
            }
            Platform::Microsoft(_, windows_lcid) => {
                if let Some(b) = language.windows_code() {
                    windows_lcid.0.to_lowercase().starts_with(&b.to_lowercase())
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum NameType {
    CopyRightNotice,
    FontFamily,
    FontSubFamily,
    UniqueSFID,
    FullName,
    Version,
    PostScriptName,
    TradeMark,
    Manufacturer,
    Designer,
    Description,
    VendorURL,
    DesignerURL,
    LicenseDesc,
    LicenseInfURL,
    Reserved,
    PrefferedFamily,
    PrefferedSubfamily,
    CompatibleFull,
    SampleText,
    PostScriptCID,
    WWSFamilyName,
    WWSSubfamilyName,
    LightBackgroundPalleteName,
    DarkBackgroundPalleteName,
    PostScriptVariations,
    Future(u16),
    FontSpecific(u16),
    // Expansion 16 -- 255
}
impl From<u16> for NameType {
    fn from(value: u16) -> Self {
        match value {
            0 => NameType::CopyRightNotice,
            1 => NameType::FontFamily,
            2 => NameType::FontSubFamily,
            3 => NameType::UniqueSFID,
            4 => NameType::FullName,
            5 => NameType::Version,
            6 => NameType::PostScriptName,
            7 => NameType::TradeMark,
            8 => NameType::Manufacturer,
            9 => NameType::Designer,
            10 => NameType::Description,
            11 => NameType::VendorURL,
            12 => NameType::DesignerURL,
            13 => NameType::LicenseDesc,
            14 => NameType::LicenseInfURL,
            15 => NameType::Reserved,
            16 => NameType::PrefferedFamily,
            17 => NameType::PrefferedSubfamily,
            18 => NameType::CompatibleFull,
            19 => NameType::SampleText,
            20 => NameType::PostScriptCID,
            21 => NameType::WWSFamilyName,
            22 => NameType::WWSSubfamilyName,
            23 => NameType::LightBackgroundPalleteName,
            24 => NameType::DarkBackgroundPalleteName,
            25 => NameType::PostScriptVariations,
            a @ 26..=255 => NameType::Future(a),
            a @ 256.. => NameType::FontSpecific(a),
        }
    }
}
impl TryFrom<(u16, u16, u16)> for Platform {
    type Error = Error;

    fn try_from(
        (platform_id, platform_specific_id, language_id): (u16, u16, u16),
    ) -> Result<Self, Self::Error> {
        Ok(match platform_id {
            0 | 2 => {
                if language_id != 0 {
                    return Err(Error::new_with_message(
                        ErrorKind::ArgumentError,
                        "Invalid Language code for Unicode Platform",
                    ));
                };
                Platform::Unicode(platform_specific_id)
            }
            1 => Platform::Macintosh(platform_specific_id.try_into()?, language_id.into()),
            3 => Platform::Microsoft(platform_specific_id.try_into()?, language_id.into()),
            _ => {
                return Err(Error::new_with_message(
                    ErrorKind::ArgumentError,
                    format!("Invalid Platform ID: {platform_id:?}"),
                ))
            }
        })
    }
}
use std::mem::transmute;

use crate::{
    error::{Error, ErrorKind},
    fonts::filetypes::sfnt::{
        encodings::{Encoding, MacRoman},
        sfnt_primitives::FromSFNTBytes,
    },
};
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum NameValue {
    Clean(String),
    UnknownPlatform(Platform),
}

impl NameValue {
    fn try_from(p: Platform, s: &[u8]) -> Result<Self, Error> {
        match p {
            Platform::Microsoft(
                MicrosoftPSID::UnicodeBMP | MicrosoftPSID::UnicodeFullRepertoire,
                _,
            )
            | Platform::Unicode(_) => {
                let s = s
                    .chunks(2)
                    .map(|a| u16::from_be_bytes(a.try_into().unwrap()));
                let mut v = String::new();
                for c in char::decode_utf16(s) {
                    v.push(c?);
                }
                Ok(NameValue::Clean(v))
            }
            Platform::Macintosh(MacintoshPSID::Roman, _) => {
                let mut v = String::new();
                let mr = MacRoman;
                for c in s {
                    v.push(mr.decode(*c)?);
                }
                Ok(NameValue::Clean(v))
            }
            a => Ok(NameValue::UnknownPlatform(a)),
        }
    }
}

use super::CommonTable;
struct NameRef {
    name_id: NameType,
    platform: Platform,
    offset: u64,
    length: u64,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct NameInfo {
    pub name_id: NameType,
    pub platform: Platform,
    pub value: NameValue,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct NameTable(pub Vec<NameInfo>);

impl CommonTable for NameTable {
    fn read_table<T: crate::fonts::filetypes::sfnt::SeekAndBufRead>(
        file: &mut T,
        offset: u32,
    ) -> Result<Self, crate::error::Error> {
        let offset = offset as u64;
        file.seek(std::io::SeekFrom::Start(offset))?;
        let format = u16::read_from(file)?;
        let count = u16::read_from(file)?;
        let string_offset = u16::read_from(file)? as u64 + offset;
        debug_assert!(format == 0);
        let mut v = Vec::new();
        for _ in 0..count {
            let platform: Platform = (
                u16::read_from(file)?,
                u16::read_from(file)?,
                u16::read_from(file)?,
            )
                .try_into()?;
            let name_id: NameType = u16::read_from(file)?.into();
            let length = u16::read_from(file)? as u64;
            let offset = u16::read_from(file)? as u64 + string_offset;
            v.push(NameRef {
                platform,
                name_id,
                offset,
                length,
            });
        }
        v.sort_by_key(|a| a.offset);
        let mut name_buf = Vec::new();
        let mut results = vec![];
        for name in v {
            file.seek(std::io::SeekFrom::Start(name.offset))?;
            let length = name.length as usize;
            if name_buf.len() < length {
                name_buf.extend((name_buf.len()..length).map(|_| 0u8));
            }
            if length != file.read(&mut name_buf[0..length])? {
                return Err(Error::new_with_message(
                    ErrorKind::IoError,
                    "Insufficient bytes read",
                ));
            }
            match NameValue::try_from(name.platform, &name_buf[0..length]) {
                Ok(a) => {
                    results.push(NameInfo {
                        name_id: name.name_id,
                        platform: name.platform,
                        value: a,
                    });
                }
                Err(e) if e.kind == ErrorKind::NotImplemented => {
                    if let Some(s) = e.message.as_ref() {
                        eprintln!("{}", s);
                    }
                }
                Err(e) => return Err(e),
            }
        }
        Ok(Self(results))
    }
}
impl IntoIterator for NameTable {
    type Item = NameInfo;

    type IntoIter = <std::vec::Vec<NameInfo> as std::iter::IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
impl NameTable {
    pub fn iter(&self) -> std::slice::Iter<'_, NameInfo> {
        self.0.iter()
    }
}

pub trait LanguageType {
    fn windows_code(&self) -> Option<&str> {
        None
    }
    fn macos_language_code(&self) -> Option<MacintoshLanguageCode> {
        None
    }
}
impl LanguageType for MacintoshLanguageCode {
    fn macos_language_code(&self) -> Option<MacintoshLanguageCode> {
        Some(*self)
    }
}
impl LanguageType for &str {
    fn windows_code(&self) -> Option<&str> {
        Some(self)
    }
}

#[cfg(test)]
use super::test::*;
#[cfg(test)]
use crate::fonts::filetypes::sfnt::tables::TableTag;
#[test]
fn test() {
    let result = match test_mega::<NameTable>(&TableTag(*b"name")) {
        Ok(ok) => ok,
        Err(e) => {
            println!("Error {}", e);
            return;
        }
    };
    dbg!(result);
}
