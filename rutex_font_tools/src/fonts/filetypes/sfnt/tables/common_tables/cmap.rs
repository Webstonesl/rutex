use std::{
    collections::{BTreeMap, VecDeque},
    io::Cursor,
    ops::{RangeBounds, Sub},
    u16,
};

use super::{CommonTable, MicrosoftPSID, Platform};
use crate::{
    error::Error,
    fonts::{self, filetypes::sfnt::FromSFNTBytes},
};

#[repr(u8)]
pub enum _CMAPSubtableFormat {
    Format0 {
        glyph_index: Box<[u8; 256]>,
        language: u16,
    } = 0,
    Format2 {} = 2,
    Format4 = 4,
    Format6 = 6,
    Format8 = 8,
    Format10 = 10,
    Format12 = 12,
    Format13 = 13,
    Format14 = 14,
}
// struct SubHeader {

// }
impl _CMAPSubtableFormat {
    fn try_read<T: crate::fonts::filetypes::sfnt::SeekAndBufRead>(
        read: &mut T,
    ) -> Result<_CMAPSubtableFormat, Error> {
        let start_position = read.stream_position()?;
        let (format_code, length, length_read) = match u16::read_from(read)? {
            format_code @ (0 | 2 | 4 | 6) => (format_code, u16::read_from(read)? as u64, 4),
            format_code @ (8 | 10 | 12 | 13) => (
                format_code,
                {
                    assert_eq!(u16::read_from(read)?, 0);
                    u32::read_from(read)? as u64
                },
                8,
            ),
            14 => (14, { u32::read_from(read)? as u64 }, 6),

            a => {
                return Err(Error::new_with_message(
                    crate::error::ErrorKind::DuplicateParameters,
                    format!("Invalid CMAP Format: {a}"),
                ));
            }
        };
        // let target = length - length_read + read.stream_position().unwrap();

        let v = match format_code {
            0 => {
                let lang = u16::read_from(read)?;
                _CMAPSubtableFormat::Format0 {
                    language: lang,
                    glyph_index: Box::new(u8::read_from_into_array(read)?),
                }
            }
            2 => todo!(),
            4 => {
                let lang = u16::read_from(read)?;
                let seg_count = u16::read_from(read)? / 2;
                let _ = u16::read_from(read)?;
                let _ = u16::read_from(read)?;
                let _ = u16::read_from(read)?;
                let end_code = u16::read_n_into_vec(read, seg_count)?;
                assert_eq!(0, u16::read_from(read)?);
                let start_code = u16::read_n_into_vec(read, seg_count)?;
                let id_delta = u16::read_n_into_vec(read, seg_count)?;
                let id_range_offset = u16::read_n_into_vec(read, seg_count)?;
                let mut map_info = start_code
                    .into_iter()
                    .zip(end_code)
                    .zip(id_delta)
                    .zip(id_range_offset)
                    .map(|(((a, b), c), d)| (a, b, c, d))
                    .collect::<Vec<_>>();
                let glyph_id_array_offset = read.stream_position()?;
                let length = (glyph_id_array_offset - start_position) / 2;
                let glyph_id_array = u16::read_n_into_vec(read, length)?;
                let mut map = BTreeMap::new();
                let mut i = u16::MIN;
                for (start_code, end_code, id_delta, id_range_offset) in map_info {
                    i = end_code;
                    if id_range_offset == 0 {
                        for nr in start_code..=end_code {
                            map.insert(nr, glyph_id_array[nr.wrapping_add(id_delta) as usize]);
                        }
                    } else {
                        for nr in start_code..=end_code {
                            let delta = (((nr as u32) - (start_code as u32)) * 2) as u16;

                            // id_range_offset/2
                            dbg!(n);
                        }
                    };
                }

                // let map =

                todo!()
            }
            6 => todo!(),
            8 => todo!(),
            10 => todo!(),
            12 => todo!(),
            13 => todo!(),
            14 => todo!(),
            _ => unreachable!(),
        };
        // assert_eq!(target, read.stream_position().unwrap());
        Ok(v)
    }
}
impl _CMAPSubtableFormat {}
pub struct CMAPSubtable {
    platform: Platform,
    offset: u32,
}
#[derive(Debug)]
pub struct CMAPTable {}
enum SubTableType {
    Unicode,
    UnicodeVariation,
    NonUnicode,
}
impl CMAPTable {}
impl CommonTable for CMAPTable {
    fn read_table<T: crate::fonts::filetypes::sfnt::SeekAndBufRead>(
        read: &mut T,
        offset: u32,
    ) -> Result<Self, crate::error::Error> {
        read.seek(std::io::SeekFrom::Start(offset as u64))?;
        let version = u16::read_from(read)?;
        let subtable_count = u16::read_from(read)?;
        let mut subtables = vec![];
        for _ in 0..subtable_count {
            let platform_id = u16::read_from(read)?;
            let platform_specific_id = u16::read_from(read)?;
            let offset = u32::read_from(read)?;
            let platform = Platform::try_from((platform_id, platform_specific_id, 0))?;
            subtables.push(CMAPSubtable { platform, offset });
        }
        subtables.sort_unstable_by_key(|a| a.offset);
        for subtable in subtables {
            read.seek(std::io::SeekFrom::Start((subtable.offset + offset) as u64))?;
            _CMAPSubtableFormat::try_read(read)?;
        }
        Ok(CMAPTable {})
    }
}

#[cfg(test)]
use super::test::*;
#[cfg(test)]
use crate::fonts::filetypes::sfnt::tables::TableTag;
#[test]
fn test() {
    let result = match test_mega::<CMAPTable>(&TableTag(*b"cmap")) {
        Ok(ok) => ok,
        Err(e) => {
            println!("Error {}", e);
            return;
        }
    };
    dbg!(result);
}
