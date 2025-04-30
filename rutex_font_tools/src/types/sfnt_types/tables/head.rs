use std::ops::BitOr;

use super::super::SFNTPrimitive;
use super::{DependentSFNTTable, ReadableSFNTTable, SFNTTable};
use crate::types::sfnt_types::{Fixed, IFWord, index::TableReference};
#[derive(Clone, Copy, Debug, Default)]
pub struct FontHeaderFlags(u16);
impl From<u16> for FontHeaderFlags {
    fn from(value: u16) -> Self {
        Self(value)
    }
}
impl<T: Into<FontHeaderFlags>> BitOr<T> for FontHeaderFlags {
    type Output = FontHeaderFlags;

    fn bitor(self, rhs: T) -> Self::Output {
        Self(rhs.into().0 | self.0)
    }
}
#[derive(Clone, Copy, Debug)]
pub struct MacStyle(u16);
impl From<u16> for MacStyle {
    fn from(value: u16) -> Self {
        MacStyle(value)
    }
}
#[derive(Clone, Copy, Debug)]
pub enum FontBase {
    Glyph,
    BitMap,
}

#[derive(Clone)]
pub struct FontHeader {
    pub base: FontBase,
    pub font_revision: Fixed,
    pub flags: FontHeaderFlags,
    pub f_units_per_em: u16,
    pub created: u64,
    pub modified: u64,
    pub x_min: IFWord,
    pub y_min: IFWord,
    pub x_max: IFWord,
    pub y_max: IFWord,
    pub mac_style: MacStyle,
    pub lowest_rec_ppem: u16,
    pub fontdirection: i16,
    pub index_to_loc_format: i16,
}
impl SFNTTable for FontHeader {
    fn into(self) -> super::AnySFNTTable {
        super::AnySFNTTable::FontHeader(Box::new(self))
    }
}
impl DependentSFNTTable for FontHeader {
    const TAGS: &[&crate::types::sfnt_types::index::TableTagInner] = &[b"bhed", b"head"];

    fn read(
        file: &mut crate::types::sfnt_types::SFNTFile,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let TableReference {
            tag,
            offset,
            length,
            ..
        } = file.get_table_reference_any_error(Self::TAGS)?;
        let base = match &tag.0 {
            b"bhed" => FontBase::BitMap,
            b"head" => FontBase::Glyph,
            _ => unreachable!(),
        };
        let read = &mut file.file;
        {
            let v = u16::read_from(read)?;
            debug_assert_eq!(v, 1);
        }
        {
            let v = u16::read_from(read)?;
            debug_assert_eq!(v, 0);
        }
        let font_revision = Fixed::read_from(read)?;
        u32::read_from(read)?;
        let flags = FontHeaderFlags(u16::read_from(read)?);

        let f_units_per_em = u16::read_from(read)?;
        let created = u64::read_from(read)?;
        let modified = u64::read_from(read)?;
        let x_min = IFWord::read_from(read)?;
        let y_min = IFWord::read_from(read)?;
        let x_max = IFWord::read_from(read)?;
        let y_max = IFWord::read_from(read)?;
        let mac_style = MacStyle(u16::read_from(read)?);
        let lowest_rec_ppem = u16::read_from(read)?;
        let fontdirection = i16::read_from(read)?;
        let index_to_loc_format = i16::read_from(read)?;

        Ok(FontHeader {
            base,
            font_revision,
            flags,
            f_units_per_em,
            created,
            modified,
            x_min,
            y_min,
            x_max,
            y_max,
            mac_style,
            lowest_rec_ppem,
            fontdirection,
            index_to_loc_format,
        })
    }
}
