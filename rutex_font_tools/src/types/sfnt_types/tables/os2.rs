use std::error::Error;

use crate::types::sfnt_types::IFWord;
use crate::types::sfnt_types::index::SFNTError;

use super::super::SFNTPrimitive;
use super::{ReadableSFNTTable, SFNTTable};

#[derive(Debug, Clone)]
pub struct OS2Table {
    pub x_avg_char_width: i16,
    pub us_weight_class: u16,
    pub us_width_class: u16,
    pub fs_type: i16,
    pub y_subscript_x_size: i16,
    pub y_subscript_y_size: i16,
    pub y_subscript_x_offset: i16,
    pub y_subscript_y_offset: i16,
    pub y_superscript_x_size: i16,
    pub y_superscript_y_size: i16,
    pub y_superscript_x_offset: i16,
    pub y_superscript_y_offset: i16,
    pub y_strikeout_size: i16,
    pub y_strikeout_position: i16,
    pub s_family_class: i16,
    pub panose: [u8; 10],
    pub ul_unicode_range: [u32; 4],
    pub ach_vend_i_d: [i8; 4],
    pub fs_selection: u16,
    pub fs_first_char_index: u16,
    pub fs_last_char_index: u16,
    pub open_type_additional: OpenTypeAdditional,
}
impl SFNTTable for OS2Table {
    fn into(self) -> super::AnySFNTTable {
        super::AnySFNTTable::OS2Table(Box::new(self))
    }
}
impl ReadableSFNTTable for OS2Table {
    const TAGS: &[&crate::types::sfnt_types::index::TableTagInner] = &[b"OS/2"];

    fn read<R: std::io::Read + std::io::Seek>(
        read: &mut R,
        _: u32,
    ) -> Result<Self, Box<dyn Error>> {
        let version = u16::read_from(read)?;
        Ok(Self {
            x_avg_char_width: i16::read_from(read)?,
            us_weight_class: u16::read_from(read)?,
            us_width_class: u16::read_from(read)?,
            fs_type: i16::read_from(read)?,
            y_subscript_x_size: i16::read_from(read)?,
            y_subscript_y_size: i16::read_from(read)?,
            y_subscript_x_offset: i16::read_from(read)?,
            y_subscript_y_offset: i16::read_from(read)?,
            y_superscript_x_size: i16::read_from(read)?,
            y_superscript_y_size: i16::read_from(read)?,
            y_superscript_x_offset: i16::read_from(read)?,
            y_superscript_y_offset: i16::read_from(read)?,
            y_strikeout_size: i16::read_from(read)?,
            y_strikeout_position: i16::read_from(read)?,
            s_family_class: i16::read_from(read)?,
            panose: u8::read_array(read)?,
            ul_unicode_range: u32::read_array(read)?,
            ach_vend_i_d: i8::read_array(read)?,
            fs_selection: u16::read_from(read)?,
            fs_first_char_index: u16::read_from(read)?,
            fs_last_char_index: u16::read_from(read)?,
            open_type_additional: OpenTypeAdditional::read(read, version)?,
        })
    }
}
#[derive(Clone, Debug)]
pub struct OpenTypeAddtional2t4 {
    pub ul_code_page_range_1: u32,
    pub ul_code_page_range_2: u32,
    pub sx_height: IFWord,
    pub s_cap_height: IFWord,
    pub us_default_char: u16,
    pub us_break_char: u16,
    pub us_max_context: u16,
}

#[derive(Clone, Debug)]
pub enum OpenTypeAdditional {
    Version0,
    Version1 {
        ul_code_page_range_1: u32,
        ul_code_page_range_2: u32,
    },
    Version2(OpenTypeAddtional2t4),
    Version3(OpenTypeAddtional2t4),
    Version4(OpenTypeAddtional2t4),
    Version5 {
        ul_code_page_range1: u32,
        ul_code_page_range2: u32,
        sx_height: IFWord,
        s_cap_height: IFWord,
        us_default_char: u16,
        us_break_char: u16,
        us_max_context: u16,
        us_lower_optical_point_size: u16,
        us_upper_optical_point_size: u16,
    },
}
impl OpenTypeAdditional {
    fn read<R: std::io::Read + std::io::Seek>(
        read: &mut R,
        version: u16,
    ) -> Result<Self, Box<dyn Error>> {
        Ok(match version {
            0 => Self::Version0,
            1 => Self::Version1 {
                ul_code_page_range_1: u32::read_from(read)?,
                ul_code_page_range_2: u32::read_from(read)?,
            },
            a @ (2..=4) => {
                let inner = OpenTypeAddtional2t4 {
                    ul_code_page_range_1: u32::read_from(read)?,
                    ul_code_page_range_2: u32::read_from(read)?,
                    sx_height: IFWord::read_from(read)?,
                    s_cap_height: IFWord::read_from(read)?,
                    us_default_char: u16::read_from(read)?,
                    us_break_char: u16::read_from(read)?,
                    us_max_context: u16::read_from(read)?,
                };
                match a {
                    2 => Self::Version2(inner),
                    3 => Self::Version3(inner),
                    4 => Self::Version4(inner),
                    _ => unreachable!(),
                }
            }
            5 => Self::Version5 {
                ul_code_page_range1: u32::read_from(read)?,
                ul_code_page_range2: u32::read_from(read)?,
                sx_height: IFWord::read_from(read)?,
                s_cap_height: IFWord::read_from(read)?,
                us_default_char: u16::read_from(read)?,
                us_break_char: u16::read_from(read)?,
                us_max_context: u16::read_from(read)?,
                us_lower_optical_point_size: u16::read_from(read)?,
                us_upper_optical_point_size: u16::read_from(read)?,
            },
            6.. => {
                return Err(SFNTError::InvalidVersionTag(format!(
                    "Invalid version number for TrueType/OpenType OS/2 Table: {version}"
                ))
                .into());
            }
        })
    }
}
