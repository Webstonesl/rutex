//! TODO: COMPLETE
use crate::types::sfnt_types::{Fixed, IFWord, SFNTPrimitive};

use super::{ReadableSFNTTable, SFNTTable};

#[derive(Clone)]
pub struct PostTable {
    pub italic_angle: Fixed,
    pub underline_position: IFWord,
    pub underline_thickness: IFWord,
    pub is_fixed_pitch: u32,
    pub min_mem_type_42: u32,
    pub max_mem_type_42: u32,
    pub min_mem_type_1: u32,
    pub max_mem_type_1: u32,
    pub subtable: Subtable,
}

impl SFNTTable for PostTable {
    fn into(self) -> super::AnySFNTTable {
        super::AnySFNTTable::PostTable(Box::new(self))
    }
}
impl ReadableSFNTTable for PostTable {
    const TAGS: &[&crate::types::sfnt_types::index::TableTagInner] = &[b"post"];

    fn read<R: std::io::Read + std::io::Seek>(
        read: &mut R,
        length: u32,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        // let mut buf = [0u8; 4];
        // read.read_exact(&mut buf)?;
        // eprintln!("{:02x?} {:?}", buf, i32::from_be_bytes(buf) / (1 << 16));
        let format = Fixed::read_from(read)?;
        let italic_angle = Fixed::read_from(read)?;
        let underline_position = IFWord::read_from(read)?;
        let underline_thickness = IFWord::read_from(read)?;
        let is_fixed_pitch = u32::read_from(read)? != 0;
        let min_mem_type_42 = u32::read_from(read)?;
        let max_mem_type_42 = u32::read_from(read)?;
        let min_mem_type_1 = u32::read_from(read)?;
        let max_mem_type_1 = u32::read_from(read)?;
        dbg!(format);
        match format.into() {
            1.0 | 2.0 | 2.5 | 3.0 | 4.0 => {
                todo!()
            }
            _ => {
                unreachable!()
            }
        }
    }
}
#[derive(Clone, Copy)]
pub enum Subtable {
    Format1,
    Format2(Format2),
    Format25(),
    Format3(),
    Format4(),
}
#[derive(Clone, Copy)]
pub struct Format2 {}
#[derive(Clone, Copy)]
pub struct Format25 {}
#[derive(Clone, Copy)]
pub struct Format3 {}
#[derive(Clone, Copy)]
pub struct Format4 {}
