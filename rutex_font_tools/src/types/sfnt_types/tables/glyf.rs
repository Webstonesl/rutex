use core::num;
use std::{
    error::Error,
    fmt::Debug,
    io::{Cursor, Read, Seek},
    ops::Add,
};

use crate::types::sfnt_types::{
    IFWord,
    index::TableReference,
    tables::{head::FontHeader, loca::GlyphOffsetTable},
};

use super::super::SFNTPrimitive;
use super::{DependentSFNTTable, SFNTTable};

#[derive(Clone)]
pub struct GlyphTable {
    data: Vec<u8>,
}
impl SFNTTable for GlyphTable {
    fn into(self) -> super::AnySFNTTable {
        super::AnySFNTTable::Glyphs(Box::new(self))
    }
}
impl DependentSFNTTable for GlyphTable {
    const TAGS: &[&crate::types::sfnt_types::index::TableTagInner] = &[b"glyf"];

    fn read(
        file: &mut crate::types::sfnt_types::SFNTFile,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let TableReference { offset, length, .. } =
            file.get_table_reference_any_error(Self::TAGS)?;
        let header: &FontHeader = file.get_table()?;
        let mapping: GlyphOffsetTable = file.get_table::<GlyphOffsetTable>()?.clone();
        let read = &mut file.file;
        read.seek(std::io::SeekFrom::Start(offset as u64))?;
        let data = u8::read_into_vec(read, length as usize)?;
        let mut cur = Cursor::new(data);
        for i in 0..mapping.len() {
            let value = mapping.get_offset::<u64>(i).unwrap();
            cur.set_position(value);
            let number_of_contours = i16::read_from(&mut cur)?;
            let x_min = IFWord::read_from(read)?;
            let y_min = IFWord::read_from(read)?;
            let x_max = IFWord::read_from(read)?;
            let y_max = IFWord::read_from(read)?;
            if number_of_contours > 0 {
                GlyphTypes::read_simple(&mut cur, number_of_contours as u16)
            } else {
                GlyphTypes::read_compound(&mut cur, number_of_contours.unsigned_abs())
            }?;
            // let i16::read_from(cur)?;
        }
        todo!()
    }
}
#[derive(Clone, Copy, Debug)]
pub struct Point<T: Copy> {
    x: T,
    y: T,
}
// Custom conversion method to avoid conflicting with the blanket implementation of `From<T> for T`.
impl<T: Copy> Point<T> {
    pub fn convert<J: Copy + From<T>>(self) -> Point<J> {
        Point {
            x: J::from(self.x),
            y: J::from(self.y),
        }
    }
}
#[derive(Clone, Debug)]
pub enum GlyphTypes {
    Simple {},
    Compound {},
}
impl GlyphTypes {
    fn read_simple<R: Read + Seek>(
        read: &mut R,
        contours_count: u16,
    ) -> Result<Self, Box<dyn Error>> {
        let end_of_points = u16::read_into_vec(read, contours_count as usize)?;
        let instruction_length = u16::read_from(read)?;
        let instructions = u8::read_into_vec(read, instruction_length as usize);
        dbg!(end_of_points);
        todo!()
    }
    fn read_compound<R: Read + Seek>(
        read: &mut R,
        contours_count: u16,
    ) -> Result<Self, Box<dyn Error>> {
        todo!()
    }
}
