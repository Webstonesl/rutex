use easybitflags::{derive_flags, FlagValueCollection};

use crate::fonts::filetypes::sfnt::sfnt_primitives::{FWord, Fixed, FromSFNTBytes, LongDateTime};

use super::CommonTable;

derive_flags!(
    #[flag]
    #[derive(Debug)]
    pub enum HeaderFlag {
        Y0isBaseline,
        XPosLMBBLSB,
        ScaledAndActualNotEq,
        UseIntegerScaling,
        MicrosoftTag,
        VerticalFont,
        MustBe0,
        RequiresLayout,
        AATWithMetamorphis,
        StrongRightToLeft,
        IndicStyleRearrangement,
        Adobe1,
        Adobe2,
        Adobe3,
        GenericFont,
    }
    #[flags]
    pub struct HeaderFlags(u16);
);

derive_flags! {
    #[flag]
    #[derive(Debug)]
    pub enum MacStyle {
        Bold,
        Italic,
        Underline,
        Outline,
        Condensed,
        Extend,
    }
    #[flags]
    pub struct MacStyles(u8);
}

#[derive(Debug, Clone, Copy)]
pub struct HeadTable {
    pub version: Fixed,
    pub fontrevision: Fixed,
    pub checksumadjust: u32,
    pub magicnumber: u32,
    pub flags: HeaderFlags,
    pub units_per_em: u16,
    pub created: LongDateTime,
    pub modified: LongDateTime,
    pub x_min: FWord,
    pub y_min: FWord,
    pub x_max: FWord,
    pub y_max: FWord,
    pub mac_style: MacStyles,
    pub lowest_rec_ppem: u16,
    pub font_direction_hint: i16,
    pub index_to_loc_format: i16,
    pub glyph_data_format: i16,
}
impl CommonTable for HeadTable {
    fn read_table<T: crate::fonts::filetypes::sfnt::SeekAndBufRead>(
        file: &mut T,
        offset: u32,
    ) -> Result<Self, crate::error::Error> {
        file.seek(std::io::SeekFrom::Start(offset as u64))?;
        let version: Fixed = Fixed::read_from(file)?;
        let fontrevision: Fixed = Fixed::read_from(file)?;
        let checksumadjust: u32 = u32::read_from(file)?;
        let magicnumber: u32 = u32::read_from(file)?;
        let flags: HeaderFlags = HeaderFlags::from_primitive(&u16::read_from(file)?);
        let units_per_em: u16 = u16::read_from(file)?;
        let created: LongDateTime = LongDateTime::read_from(file)?;
        let modified: LongDateTime = LongDateTime::read_from(file)?;
        let x_min: FWord = FWord::read_from(file)?;
        let y_min: FWord = FWord::read_from(file)?;
        let x_max: FWord = FWord::read_from(file)?;
        let y_max: FWord = FWord::read_from(file)?;
        let mac_style: MacStyles = MacStyles::from_primitive(&(u16::read_from(file)? as u8));
        let lowest_rec_ppem: u16 = u16::read_from(file)?;
        let font_direction_hint: i16 = i16::read_from(file)?;
        let index_to_loc_format: i16 = i16::read_from(file)?;
        let glyph_data_format: i16 = i16::read_from(file)?;

        assert!(magicnumber == 0x5F0F3CF5);
        assert!(glyph_data_format == 0);

        Ok(HeadTable {
            version,
            fontrevision,
            checksumadjust,
            magicnumber,
            flags,
            units_per_em,
            created,
            modified,
            x_min,
            y_min,
            x_max,
            y_max,
            mac_style,
            lowest_rec_ppem,
            font_direction_hint,
            index_to_loc_format,
            glyph_data_format,
        })
    }
}
#[cfg(test)]
use super::test::*;

#[cfg(test)]
use crate::fonts::filetypes::sfnt::tables::TableTag;
#[test]
fn test() {
    let result = match test_mega::<HeadTable>(&TableTag(*b"head")) {
        Ok(ok) => ok,
        Err(e) => {
            println!("Error {}", e);
            return;
        }
    };
    dbg!(result);
}
