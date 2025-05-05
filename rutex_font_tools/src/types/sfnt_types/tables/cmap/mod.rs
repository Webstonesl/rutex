use crate::types::sfnt_types::{SFNTPrimitive, SFNTReadable};

use crate::types::sfnt_types::index::SFNTError;

use super::super::SFNTStream;
use super::name::{PlatformID, macintosh};
use super::{ReadableSFNTTable, SFNTTable};
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::Debug;
use std::io::{Cursor, Read, Seek};
use std::ops::{Add, Range, RangeInclusive, Sub};
impl PlatformID {
    pub fn read_cmap<R: Read + Seek>(&self, read: &mut R) -> Result<PlatformCmap, Box<dyn Error>> {
        match self {
            PlatformID::Unicode => Ok(PlatformCmap::Unicode(read.get_prim()?)),
            PlatformID::Macintosh => Ok(PlatformCmap::Macintosh(
                read.get_prim::<2, u16>()?.try_into()?,
            )),
            PlatformID::Microsoft => Ok(PlatformCmap::Microsoft(
                read.get_prim::<2, u16>()?.try_into()?,
            )),
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub enum PlatformCmap {
    Unicode(u16),
    Macintosh(macintosh::ScriptCode),
    Microsoft(macintosh::ScriptCode),
}
#[derive(Clone)]
pub struct CMAPTable(BTreeMap<u32, u32>);
impl AsRef<BTreeMap<u32, u32>> for CMAPTable {
    fn as_ref(&self) -> &BTreeMap<u32, u32> {
        &self.0
    }
}

impl CMAPTable {}
impl SFNTTable for CMAPTable {
    fn into(self) -> super::AnySFNTTable {
        super::AnySFNTTable::CharacterMap(Box::new(self))
    }
}
pub trait Step: Ord + Sized + Copy + Add<Self, Output = Self> {
    fn next(&self) -> Option<Self>;
    fn previous(&self) -> Option<Self>;
}
impl Step for u32 {
    fn next(&self) -> Option<Self> {
        if Self::MAX == *self {
            None
        } else {
            Some(self + 1)
        }
    }
    fn previous(&self) -> Option<Self> {
        if Self::MIN == *self {
            None
        } else {
            Some(self - 1)
        }
    }
}
impl Step for usize {
    fn next(&self) -> Option<Self> {
        if Self::MAX == *self {
            None
        } else {
            Some(self + 1)
        }
    }
    fn previous(&self) -> Option<Self> {
        if Self::MIN == *self {
            None
        } else {
            Some(self - 1)
        }
    }
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Range_<Idx: Ord + Copy + Step> {
    Range(Idx, Idx),
    Singular(Idx),
    None,
}

impl<Idx: Ord + Copy + Step> From<Idx> for Range_<Idx> {
    fn from(value: Idx) -> Self {
        Range_::Singular(value)
    }
}

impl<Idx: Ord + Copy + Step> PartialOrd for Range_<Idx> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<Idx: Step + Ord + Copy> Ord for Range_<Idx> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let vself = if let Range_::None = self { 1 } else { 0 };
        let vother = if let Range_::None = other { 1 } else { 0 };
        if vself != vother {
            return vself.cmp(&vother);
        }
        let self_start = match self {
            Range_::Range(a, _) => a,
            Range_::Singular(a) => a,
            Range_::None => unreachable!(),
        };
        let other_start = match other {
            Range_::Range(a, _) => a,
            Range_::Singular(a) => a,
            Range_::None => unreachable!(),
        };
        match self_start.cmp(other_start) {
            std::cmp::Ordering::Equal => {}
            a => return a,
        }
        let self_end = match self {
            Range_::Range(_, a) => Some(*a),
            Range_::Singular(a) => a.next(),
            Range_::None => todo!(),
        };
        let other_end = match other {
            Range_::Range(_, a) => Some(*a),
            Range_::Singular(a) => a.next(),
            Range_::None => todo!(),
        };
        match (self_end, other_end) {
            (None, None) => std::cmp::Ordering::Equal,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (Some(_), None) => std::cmp::Ordering::Less,
            (Some(a), Some(b)) => a.cmp(&b),
        }
    }
}
pub enum SubTableType {
    Unicode,
    UnicodeVariation,
    NonUnicode,
}
// pub struct RangeMap<Idx: Ord + Copy + Step>(BTreeSet<Range_<Idx>>);

impl ReadableSFNTTable for CMAPTable {
    const TAGS: &[&crate::types::sfnt_types::index::TableTagInner] = &[b"cmap"];

    fn read<R: Read + Seek>(read: &mut R, length: u32) -> Result<Self, Box<dyn Error>> {
        let initial_offset = read.stream_position().unwrap();
        let version: u16 = read.get_prim()?;
        debug_assert_eq!(version, 0, "Version must be set to 0");
        let nr_subtables: u16 = read.get_prim()?;
        let mut v = Vec::new();
        for _ in 0..nr_subtables {
            let platform = match PlatformID::read_from(read) {
                Ok(a) => a,
                Err(_) => continue,
            }
            .read_cmap(read)?;
            let offset: u32 = read.get_prim()?;
            match platform {
                PlatformCmap::Unicode(_) => {}
                PlatformCmap::Macintosh(_) => {}
                PlatformCmap::Microsoft(_) => {}
            }
            v.push((platform, offset as u64));
        }
        let other_offset = read.stream_position().unwrap();
        let mut data = vec![0u8; length as usize - ((other_offset - initial_offset) as usize)];
        read.read_exact(&mut data)?;
        let mut cursor = Cursor::new(&data);
        let mut map = BTreeMap::new();
        for (platform, offset) in v {
            let offset = offset + initial_offset - other_offset;
            cursor.set_position(offset);
            if let Some(data) = match _CmapSubTablePrelim::read_from(&mut cursor) {
                Ok(a) => a,
                Err(b) => match b.downcast_ref::<SFNTError>() {
                    Some(SFNTError::InvalidCMAPTable(_)) => continue,
                    _ => return Err(b),
                },
            }
            .with(platform, offset as usize)
            .read_data(&data)?
            {
                data.write_into(&mut map);
            }
        }

        Ok(CMAPTable(map))
    }
}
#[derive(Clone, Copy, Debug)]
#[allow(dead_code)]
pub struct _CmapSubTablePrelim {
    format: u16,
    language: u16,
    length: usize,
}
impl _CmapSubTablePrelim {
    fn with(&self, platform: PlatformCmap, offset: usize) -> CmapSubTablePrelim {
        CmapSubTablePrelim {
            format: self.format,
            platform,
            range: (offset)..(offset + self.length),
            language: self.language,
        }
    }
}
#[derive(Clone, Debug)]
pub struct CmapSubTablePrelim {
    pub format: u16,
    pub platform: PlatformCmap,
    pub range: Range<usize>,
    pub language: u16,
}
impl CmapSubTablePrelim {
    fn read_data(&self, data: &[u8]) -> Result<Option<CMAPSubTable>, Box<dyn Error>> {
        // let glyph_index_address = id_range_offset[i] + 2 * (c - start_code[i]) + 2 * i;
        CMAPSubTable::read_no_err(&data[self.range.clone()], self)
    }
}
pub trait SubTableTrait<Input: Ord + Copy + Into<u32>> {
    fn get_element(&self, c: &Input) -> u16;
    fn get_mapping(&self) -> BTreeMap<Input, u16>;
    fn write_into(&self, vec: &mut BTreeMap<u32, u32>);
}

pub(crate) trait SubTableReadableTrait<Input: Ord + Copy + Into<u32>>:
    Sized + Clone
{
    fn read(data: &[u8], extrainfo: &CmapSubTablePrelim) -> Result<Self, Box<dyn Error>>;
    fn get_element(&self, c: &Input) -> u16;
    fn get_mapping(&self) -> BTreeMap<Input, u16>;
    fn write_into(&self, map: &mut BTreeMap<u32, u32>);
}
impl<Input: Ord + Copy + Into<u32>, T: SubTableReadableTrait<Input>> SubTableTrait<Input> for T {
    fn get_element(&self, c: &Input) -> u16 {
        SubTableReadableTrait::get_element(self, c)
    }

    fn get_mapping(&self) -> BTreeMap<Input, u16> {
        SubTableReadableTrait::get_mapping(self)
    }

    fn write_into(&self, map: &mut BTreeMap<u32, u32>) {
        SubTableReadableTrait::write_into(self, map);
    }
}
impl SFNTReadable for _CmapSubTablePrelim {
    fn read_from<R: Read + ?Sized + Seek>(mut read: &mut R) -> Result<Self, Box<dyn Error>> {
        let format: u16 = read.get_prim()?;
        match format {
            format @ (0 | 2 | 4 | 6) => {
                let length: usize = read.get_prim::<2, u16>()? as usize;
                let language: u16 = read.get_prim()?;
                Ok(Self {
                    format,
                    language,
                    length,
                })
            }
            format @ (8 | 10 | 12 | 13) => {
                let language: u16 = read.get_prim()?;
                let length = read.get_prim::<4, u32>()? as usize;
                Ok(Self {
                    format,
                    language,
                    length,
                })
            }
            14 => Ok(Self {
                format,
                language: 0,
                length: read.get_prim::<4, u32>()? as usize,
            }),
            a @ (1 | 3 | 5 | 7 | 9 | 11 | 15..) => Err(SFNTError::InvalidCMAPTable(a).into()),
        }
    }
}

#[derive(Clone)]
enum CMAPSubTable {
    Format0(format00::CMAPSubTable0),
    Format4(format04::CMAPSubTable4),
    Format6(format06::CMAPSubTable6),
    Format12(format12::CMAPSubTable12),
    Format13(format13::CMAPSubTable13),
}
impl CMAPSubTable {
    fn write_into(&self, map: &mut BTreeMap<u32, u32>) {
        match self {
            CMAPSubTable::Format0(cmapsub_table) => {
                SubTableReadableTrait::write_into(cmapsub_table, map)
            }
            CMAPSubTable::Format4(cmapsub_table) => {
                SubTableReadableTrait::write_into(cmapsub_table, map)
            }
            CMAPSubTable::Format6(cmapsub_table) => {
                SubTableReadableTrait::write_into(cmapsub_table, map)
            }
            CMAPSubTable::Format12(cmapsub_table) => {
                SubTableReadableTrait::write_into(cmapsub_table, map)
            }
            CMAPSubTable::Format13(cmapsub_table) => {
                SubTableReadableTrait::write_into(cmapsub_table, map)
            }
        }
    }
    fn read_no_err(
        data: &[u8],
        extrainfo: &CmapSubTablePrelim,
    ) -> Result<Option<Self>, Box<dyn Error>> {
        match extrainfo.format {
            0 => Ok(Some(Self::Format0(format00::CMAPSubTable0::read(
                data, extrainfo,
            )?))),
            4 => Ok(Some(Self::Format4(format04::CMAPSubTable4::read(
                data, extrainfo,
            )?))),
            6 => Ok(Some(Self::Format6(format06::CMAPSubTable6::read(
                data, extrainfo,
            )?))),
            12 => Ok(Some(Self::Format12(format12::CMAPSubTable12::read(
                data, extrainfo,
            )?))),
            13 => Ok(Some(Self::Format13(format13::CMAPSubTable13::read(
                data, extrainfo,
            )?))),

            1 | 3 | 5 | 7 | 9 | 11 => Ok(None),
            a => Err(SFNTError::InvalidCMAPTable(a).into()),
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) struct IncRange<Idx: Clone + Copy + Debug> {
    pub start: Idx,
    pub end: Idx,
}

impl<Idx: Clone + Copy + Debug> IncRange<Idx> {
    fn into_range(self) -> RangeInclusive<Idx> {
        self.start..=self.end
    }
}
impl<Idx: Clone + Copy + Debug> Debug for IncRange<Idx> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}..={:?}", self.start, self.end)
    }
}
impl<Idx: Clone + Copy + Debug + Step + Sub<Idx, Output = Idx>> IncRange<Idx> {
    fn len(&self) -> Idx {
        (self.end - self.start).next().unwrap()
    }
}
pub mod format00;
pub mod format04;
pub mod format06;
pub mod format12;
pub mod format13;
