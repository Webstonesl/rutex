use regex::Regex;

use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader, Write},
    process::{self, Stdio},
    str::FromStr,
};
use thiserror::Error;
#[derive(Error, Debug)]
pub enum MappingError {
    #[error("Error while parsing {0}, {1}")]
    ParseError(&'static str, String),
    #[error("Cannot find codepoint {0:04x}")]
    MappingError(u32),
}

pub const MAPPING_FILES: &[&str] = &[
    // "MAPPINGS/VENDORS/APPLE/FARSI.TXT",
    "MAPPINGS/VENDORS/APPLE/ARABIC.TXT",
    // "MAPPINGS/VENDORS/APPLE/JAPANESE.TXT",
    // "MAPPINGS/VENDORS/APPLE/DEVANAGA.TXT",
    // "MAPPINGS/VENDORS/APPLE/TURKISH.TXT",
    // "MAPPINGS/VENDORS/APPLE/ICELAND.TXT",
    // "MAPPINGS/VENDORS/APPLE/GAELIC.TXT",
    // "MAPPINGS/VENDORS/APPLE/INUIT.TXT",
    // "MAPPINGS/VENDORS/APPLE/CORPCHAR.TXT",
    // "MAPPINGS/VENDORS/APPLE/GUJARATI.TXT",
    // "MAPPINGS/VENDORS/APPLE/UKRAINE.TXT",
    // "MAPPINGS/VENDORS/APPLE/CHINTRAD.TXT",
    // "MAPPINGS/VENDORS/APPLE/CYRILLIC.TXT",
    // "MAPPINGS/VENDORS/APPLE/GURMUKHI.TXT",
    // "MAPPINGS/VENDORS/APPLE/KEYBOARD.TXT",
    // "MAPPINGS/VENDORS/APPLE/ROMAN.TXT",
    // "MAPPINGS/VENDORS/APPLE/SYMBOL.TXT",
    // "MAPPINGS/VENDORS/APPLE/CHINSIMP.TXT",
    // "MAPPINGS/VENDORS/APPLE/CENTEURO.TXT",
    // "MAPPINGS/VENDORS/APPLE/ROMANIAN.TXT",
    // "MAPPINGS/VENDORS/APPLE/DINGBATS.TXT",
    // "MAPPINGS/VENDORS/APPLE/THAI.TXT",
    // "MAPPINGS/VENDORS/APPLE/GREEK.TXT",
    // "MAPPINGS/VENDORS/APPLE/CELTIC.TXT",
    // "MAPPINGS/VENDORS/APPLE/HEBREW.TXT",
    // "MAPPINGS/VENDORS/APPLE/KOREAN.TXT",
    // "MAPPINGS/VENDORS/APPLE/CROATIAN.TXT",
    // "MAPPINGS/VENDORS/MISC/KZ1048.TXT",
    // "MAPPINGS/VENDORS/MISC/CP424.TXT",
    // "MAPPINGS/VENDORS/MISC/IBMGRAPH.TXT",
    // "MAPPINGS/VENDORS/MISC/KPS9566.TXT",
    // "MAPPINGS/VENDORS/MISC/KOI8-R.TXT",
    // "MAPPINGS/VENDORS/MISC/APL-ISO-IR-68.TXT",
    // "MAPPINGS/VENDORS/MISC/KOI8-U.TXT",
    // "MAPPINGS/VENDORS/MISC/CP1006.TXT",
    // "MAPPINGS/VENDORS/MISC/SGML.TXT",
    // "MAPPINGS/VENDORS/MISC/CP856.TXT",
    // "MAPPINGS/VENDORS/MISC/US-ASCII-QUOTES.TXT",
    // "MAPPINGS/VENDORS/MISC/ATARIST.TXT",
    // "MAPPINGS/VENDORS/MISC/DatedVersions/APL-ISO-IR-68-2004.TXT",
    // "MAPPINGS/VENDORS/NEXT/NEXTSTEP.TXT",
    // "MAPPINGS/VENDORS/ADOBE/stdenc.txt",
    // "MAPPINGS/VENDORS/ADOBE/zdingbat.txt",
    // "MAPPINGS/VENDORS/ADOBE/symbol.txt",
    // "MAPPINGS/VENDORS/MICSFT/PC/CP437.TXT",
    // "MAPPINGS/VENDORS/MICSFT/PC/CP865.TXT",
    // "MAPPINGS/VENDORS/MICSFT/PC/CP864.TXT",
    // "MAPPINGS/VENDORS/MICSFT/PC/CP737.TXT",
    // "MAPPINGS/VENDORS/MICSFT/PC/CP866.TXT",
    // "MAPPINGS/VENDORS/MICSFT/PC/CP863.TXT",
    // "MAPPINGS/VENDORS/MICSFT/PC/CP862.TXT",
    // "MAPPINGS/VENDORS/MICSFT/PC/CP874.TXT",
    // "MAPPINGS/VENDORS/MICSFT/PC/CP860.TXT",
    // "MAPPINGS/VENDORS/MICSFT/PC/CP861.TXT",
    // "MAPPINGS/VENDORS/MICSFT/PC/CP850.TXT",
    // "MAPPINGS/VENDORS/MICSFT/PC/CP852.TXT",
    // "MAPPINGS/VENDORS/MICSFT/PC/CP857.TXT",
    // "MAPPINGS/VENDORS/MICSFT/PC/CP855.TXT",
    // "MAPPINGS/VENDORS/MICSFT/PC/CP869.TXT",
    // "MAPPINGS/VENDORS/MICSFT/PC/CP775.TXT",
    // "MAPPINGS/VENDORS/MICSFT/EBCDIC/CP037.TXT",
    // "MAPPINGS/VENDORS/MICSFT/EBCDIC/CP1026.TXT",
    // "MAPPINGS/VENDORS/MICSFT/EBCDIC/CP875.TXT",
    // "MAPPINGS/VENDORS/MICSFT/EBCDIC/CP500.TXT",
    // "MAPPINGS/VENDORS/MICSFT/MAC/TURKISH.TXT",
    // "MAPPINGS/VENDORS/MICSFT/MAC/ICELAND.TXT",
    // "MAPPINGS/VENDORS/MICSFT/MAC/CYRILLIC.TXT",
    // "MAPPINGS/VENDORS/MICSFT/MAC/ROMAN.TXT",
    // "MAPPINGS/VENDORS/MICSFT/MAC/LATIN2.TXT",
    // "MAPPINGS/VENDORS/MICSFT/MAC/GREEK.TXT",
    // "MAPPINGS/VENDORS/MICSFT/WINDOWS/CP1252.TXT",
    // "MAPPINGS/VENDORS/MICSFT/WINDOWS/CP1253.TXT",
    // "MAPPINGS/VENDORS/MICSFT/WINDOWS/CP1251.TXT",
    // "MAPPINGS/VENDORS/MICSFT/WINDOWS/CP1250.TXT",
    // "MAPPINGS/VENDORS/MICSFT/WINDOWS/CP1254.TXT",
    // "MAPPINGS/VENDORS/MICSFT/WINDOWS/CP1255.TXT",
    // "MAPPINGS/VENDORS/MICSFT/WINDOWS/CP949.TXT",
    // "MAPPINGS/VENDORS/MICSFT/WINDOWS/CP1257.TXT",
    // "MAPPINGS/VENDORS/MICSFT/WINDOWS/CP1256.TXT",
    // "MAPPINGS/VENDORS/MICSFT/WINDOWS/CP874.TXT",
    // "MAPPINGS/VENDORS/MICSFT/WINDOWS/CP932.TXT",
    // "MAPPINGS/VENDORS/MICSFT/WINDOWS/CP936.TXT",
    // "MAPPINGS/VENDORS/MICSFT/WINDOWS/CP950.TXT",
    // "MAPPINGS/VENDORS/MICSFT/WINDOWS/CP1258.TXT",
    // "MAPPINGS/ISO8859/8859-10.TXT",
    // "MAPPINGS/ISO8859/8859-9.TXT",
    // "MAPPINGS/ISO8859/8859-8.TXT",
    // "MAPPINGS/ISO8859/8859-11.TXT",
    // "MAPPINGS/ISO8859/8859-13.TXT",
    // "MAPPINGS/ISO8859/8859-16.TXT",
    // "MAPPINGS/ISO8859/8859-15.TXT",
    // "MAPPINGS/ISO8859/8859-14.TXT",
    // "MAPPINGS/ISO8859/8859-1.TXT",
    // "MAPPINGS/ISO8859/8859-3.TXT",
    // "MAPPINGS/ISO8859/8859-2.TXT",
    // "MAPPINGS/ISO8859/8859-6.TXT",
    // "MAPPINGS/ISO8859/8859-7.TXT",
    // "MAPPINGS/ISO8859/DatedVersions/8859-2-1999v2.TXT",
    // "MAPPINGS/ISO8859/DatedVersions/8859-15-1999v2.TXT",
    // "MAPPINGS/ISO8859/DatedVersions/8859-11-2001v2.TXT",
    // "MAPPINGS/ISO8859/DatedVersions/8859-2-1999.TXT",
    // "MAPPINGS/ISO8859/DatedVersions/8859-3-1999.TXT",
    // "MAPPINGS/ISO8859/DatedVersions/8859-14-1998.TXT",
    // "MAPPINGS/ISO8859/DatedVersions/8859-8-1999.TXT",
    // "MAPPINGS/ISO8859/DatedVersions/8859-9-1999.TXT",
    // "MAPPINGS/ISO8859/DatedVersions/8859-15-1999.TXT",
    // "MAPPINGS/ISO8859/DatedVersions/8859-7-1987b.TXT",
    // "MAPPINGS/ISO8859/DatedVersions/8859-4-1998.TXT",
    // "MAPPINGS/ISO8859/DatedVersions/8859-5-1999.TXT",
    // "MAPPINGS/ISO8859/DatedVersions/8859-13-1998.TXT",
    // "MAPPINGS/ISO8859/DatedVersions/8859-7-1987a.TXT",
    // "MAPPINGS/ISO8859/DatedVersions/8859-8-1999v2.TXT",
    // "MAPPINGS/ISO8859/DatedVersions/8859-10-1998v2.TXT",
    // "MAPPINGS/ISO8859/DatedVersions/8859-5-1999v2.TXT",
    // "MAPPINGS/ISO8859/DatedVersions/8859-16-2001v2.TXT",
    // "MAPPINGS/ISO8859/DatedVersions/8859-4-1998v2.TXT",
    // "MAPPINGS/ISO8859/DatedVersions/8859-13-1998v2.TXT",
    // "MAPPINGS/ISO8859/DatedVersions/8859-1-1998.TXT",
    // "MAPPINGS/ISO8859/DatedVersions/8859-6-1999v2.TXT",
    // "MAPPINGS/ISO8859/DatedVersions/8859-7-2003.TXT",
    // "MAPPINGS/ISO8859/DatedVersions/8859-3-1999v2.TXT",
    // "MAPPINGS/ISO8859/DatedVersions/8859-11-2001.TXT",
    // "MAPPINGS/ISO8859/DatedVersions/8859-1-1998v2.TXT",
    // "MAPPINGS/ISO8859/DatedVersions/8859-7-2003v3.TXT",
    // "MAPPINGS/ISO8859/DatedVersions/8859-9-1999v2.TXT",
    // "MAPPINGS/ISO8859/DatedVersions/8859-16-2001.TXT",
    // "MAPPINGS/ISO8859/DatedVersions/8859-10-1998.TXT",
    // "MAPPINGS/ISO8859/DatedVersions/8859-14-1998v2.TXT",
    // "MAPPINGS/ISO8859/DatedVersions/8859-6-1999.TXT",
    // "MAPPINGS/ISO8859/8859-5.TXT",
    // "MAPPINGS/ISO8859/8859-4.TXT",
    // "MAPPINGS/ETSI/GSM0338.TXT",
];

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    LeftToRight,
    RightToLeft,
}
impl FromStr for Direction {
    type Err = MappingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "LR" => Ok(Direction::LeftToRight),
            "RL" => Ok(Direction::RightToLeft),
            a => Err(MappingError::ParseError(
                "Direction",
                format!("expected ('LR' | 'RL') found {a:?}"),
            )),
        }
    }
}

#[derive(Clone, Copy)]
pub struct UnicodeValue {
    width: usize,
    value: u32,
}
impl Debug for UnicodeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:04x}]", self.value)
    }
}
impl FromStr for UnicodeValue {
    type Err = Box<dyn Error>;

    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("0x") {
            s = &s[2..];
        }
        let value: u32 = u32::from_str_radix(s, 16)?;
        let width = s.len();

        Ok(UnicodeValue { width, value })
    }
}
#[derive(Clone)]
pub enum MappingType {
    Unicode(UnicodeValue),
    Direction(Direction),
}

impl Debug for MappingType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unicode(arg0) => write!(f, "{:?}", arg0),
            Self::Direction(Direction::LeftToRight) => write!(f, "(->)"),
            Self::Direction(Direction::RightToLeft) => write!(f, "(<-)"),
        }
    }
}
impl FromStr for MappingType {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('<') && s.ends_with('>') {
            return Ok(Self::Direction(Direction::from_str(&s[1..s.len() - 1])?));
        }
        if s.starts_with(|a: char| a.is_ascii_hexdigit()) {
            return Ok(Self::Unicode(UnicodeValue::from_str(s)?));
        }
        Err(MappingError::ParseError(
            "Mapping Type",
            format!("expected '<.*>' or unicode value, got {s:?}"),
        )
        .into())
    }
}
#[derive(Debug, Clone, Default)]
pub struct Target(Vec<MappingType>);
impl FromStr for Target {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = Vec::new();
        for a in s.split('+') {
            result.push(a.parse()?);
        }
        Ok(Target(result))
    }
}
#[derive(Clone)]
pub struct UnicodeMap(BTreeMap<u32, MapValue>, BTreeSet<usize>);
impl Debug for UnicodeMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{{")?;
        for (k, v) in &self.0 {
            write!(f, "0x{:04x} -> {:?}, ", k, v)?;
            if f.alternate() {
                writeln!(f)?;
            }
        }
        write!(f, "}}")
    }
}
impl UnicodeMap {
    pub fn fill_one_to_one<T: IntoIterator<Item = u32>>(&mut self, range: T) {
        for r in range {
            self.0.entry(r).or_insert_with(|| MapValue {
                directionality: None,
                value: vec![(r as u8) as char],
            });
        }
    }
    pub fn build_string(
        &self,
        source: &[u32],
        mut direction: Direction,
    ) -> Result<String, Box<dyn Error>> {
        let mut result = String::new();
        for s in source {
            if let Some(MapValue {
                directionality,
                value,
            }) = self.0.get(s)
            {
                if let Some(a) = directionality {
                    if a != &direction {
                        result.push(match *a {
                            Direction::LeftToRight => unsafe { char::from_u32_unchecked(0x200e) },
                            Direction::RightToLeft => unsafe { char::from_u32_unchecked(0x2002) },
                        });
                        direction = *a;
                    }
                }
                result.extend(value);
            } else if let Some(a) = char::from_u32(*s) {
                result.push(a);
            } else {
                return Err(MappingError::MappingError(*s).into());
            }
        }
        Ok(result)
    }
    pub fn to_rs_file(&self, target: &mut dyn Write) -> std::io::Result<()> {
        if self.1.len() != 1 {
            panic!("Really?!? {:?}", self.1);
        }
        let v = *self.1.first().unwrap() * 4;
        writeln!(target, "pub static MAPPING: &[(u{v},&[char])] = &[")?;
        for (source_value, target_value) in self.0.iter() {
            let f = format!("{:x}", source_value);
            writeln!(
                target,
                "\t(0x{}{},&{:?}),",
                "0".repeat(v / 4 - f.len()),
                f,
                target_value.value
            )?;
        }
        writeln!(target, "];")?;
        Ok(())
    }
    pub fn copy_to_clipboard(&self) -> Result<(), Box<dyn Error>> {
        let mut child = process::Command::new("clipcopy")
            .stdin(Stdio::piped())
            .spawn()?;
        let mut out = child.stdin.as_mut().unwrap();
        self.to_rs_file(&mut out)?;
        child.wait_with_output()?;
        Ok(())
    }
}
impl AsRef<BTreeMap<u32, MapValue>> for UnicodeMap {
    fn as_ref(&self) -> &BTreeMap<u32, MapValue> {
        &self.0
    }
}
impl AsMut<BTreeMap<u32, MapValue>> for UnicodeMap {
    fn as_mut(&mut self) -> &mut BTreeMap<u32, MapValue> {
        &mut self.0
    }
}
impl UnicodeMap {
    fn try_from_iterator<T: IntoIterator<Item = (UnicodeValue, Target)>>(
        iter: T,
    ) -> Result<Self, Box<dyn Error>> {
        let mut set: BTreeSet<usize> = BTreeSet::new();
        let mut map: BTreeMap<u32, MapValue> = BTreeMap::new();
        for (value, target) in iter {
            set.insert(value.width);
            map.insert(value.value, target.into());
        }
        Ok(Self(map, set))
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MapValue {
    directionality: Option<Direction>,
    value: Vec<char>,
}
impl Debug for MapValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(d) = self.directionality {
            write!(f, "<{:?}>", d)?;
        }
        for v in &self.value {
            write!(f, "{:?}", v)?
        }
        Ok(())
    }
}

impl From<Target> for MapValue {
    fn from(value: Target) -> Self {
        let mut value = value.0.as_slice();
        let directionality = if let Some(MappingType::Direction(e)) = value.first() {
            value = &value[1..];
            Some(*e)
        } else {
            None
        };
        let value = value
            .iter()
            .map(|a| match a {
                MappingType::Direction(_) => panic!(),
                MappingType::Unicode(UnicodeValue { value, .. }) => char::from_u32(*value).unwrap(),
            })
            .collect();
        Self {
            directionality,
            value,
        }
    }
}
pub fn read_mapping_file(path: &str) -> Result<UnicodeMap, Box<dyn Error>> {
    // println!("PATH: {path}");
    let mut file = BufReader::new(File::open(path)?);
    let mut line = 0;

    let mut mappings = Vec::new();
    loop {
        // println!("{line}");
        line += 1;
        let mut buffer = vec![];
        if file.read_until(0xA, &mut buffer)? == 0 {
            break;
        }
        if let Some(pound) = buffer
            .iter()
            .enumerate()
            .find(|(_, v)| **v == b'#')
            .map(|(a, _)| a)
        {
            buffer.drain(pound..);
        } else {
            buffer.pop();
        }
        let s: String = match buffer.is_ascii() {
            true => buffer.iter().map(|a| *a as char).collect(),
            false => {
                println!("{line}: Not asci {:02x?}", buffer);
                panic!()
            }
        };
        let s = s.trim();
        if s.is_empty() {
            continue;
        }
        let v = s
            .split_once(|a: char| a.is_whitespace())
            .map(|(a, b)| (a.trim(), b.trim()));
        if let Some((source, target)) = v {
            let source: UnicodeValue = source.parse()?;
            let target: Target = target.parse()?;
            mappings.push((source, target));
        };
    }
    UnicodeMap::try_from_iterator(mappings)
}
#[derive(Default)]
pub struct MappingMap(BTreeMap<(u16, u16), UnicodeMap>);

#[test]
fn read_map_files() {
    let map = MappingMap::default();
    let regex = Regex::new(r"/(\w+)/(\w+)\.TXT$").unwrap();

    for (i, file) in MAPPING_FILES.iter().enumerate() {
        let v = regex.captures(file).unwrap();
        let vendor = v.get(1).unwrap().as_str().to_lowercase();
        let script = v.get(2).unwrap().as_str().to_lowercase();

        // eprintln!("{:?} {:?}", vendor, script);
        let mut mapping = read_mapping_file(file).unwrap();
        mapping.fill_one_to_one(0..=0x20);
        let mut v = Vec::new();
        mapping.to_rs_file(&mut v).unwrap();
        println!("{:}", String::from_utf8(v).unwrap());
    }
}
