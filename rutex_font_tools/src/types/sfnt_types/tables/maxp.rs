use crate::types::sfnt_types::index::SFNTError;

use super::super::SFNTPrimitive;
use super::ReadableSFNTTable;

#[derive(Clone, Debug)]
pub enum MemoryManagementTable {
    TrueType {
        num_glyphs: u16,
        max_points: u16,
        max_contours: u16,
        max_component_points: u16,
        max_component_contours: u16,
        max_zones: u16,
        max_twilight_points: u16,
        max_storage: u16,
        max_function_defs: u16,
        max_instruction_defs: u16,
        max_stack_elements: u16,
        max_size_of_instructions: u16,
        max_component_elements: u16,
        max_component_depth: u16,
    },
    OpenType {
        num_glyphs: u16,
    },
}
impl super::SFNTTable for MemoryManagementTable {
    fn into(self) -> super::AnySFNTTable {
        super::AnySFNTTable::MemoryManagement(Box::new(self))
    }
}
impl ReadableSFNTTable for MemoryManagementTable {
    const TAGS: &[&crate::types::sfnt_types::index::TableTagInner] = &[b"maxp"];

    fn read<R: std::io::Read + std::io::Seek>(
        read: &mut R,
        _: u32,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        match u32::read_from(read)? {
            0x00010000 => Ok(Self::TrueType {
                num_glyphs: u16::read_from(read)?,
                max_points: u16::read_from(read)?,
                max_contours: u16::read_from(read)?,
                max_component_points: u16::read_from(read)?,
                max_component_contours: u16::read_from(read)?,
                max_zones: u16::read_from(read)?,
                max_twilight_points: u16::read_from(read)?,
                max_storage: u16::read_from(read)?,
                max_function_defs: u16::read_from(read)?,
                max_instruction_defs: u16::read_from(read)?,
                max_stack_elements: u16::read_from(read)?,
                max_size_of_instructions: u16::read_from(read)?,
                max_component_elements: u16::read_from(read)?,
                max_component_depth: u16::read_from(read)?,
            }),
            0x00005000 => Ok(Self::OpenType {
                num_glyphs: u16::read_from(read)?,
            }),
            a => Err(SFNTError::InvalidVersionTag(format!(
                "Invalid Version Tag for maxp: {a:08x}"
            ))
            .into()),
        }
    }
}
