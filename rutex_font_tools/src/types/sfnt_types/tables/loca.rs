use std::io::Seek;
use std::sync::Arc;

use crate::types::sfnt_types::index::{SFNTError, TableReference};
use crate::types::sfnt_types::tables::head::FontHeader;

use super::super::SFNTPrimitive;
use super::{DependentSFNTTable, SFNTTable};
#[derive(Clone)]
pub struct GlyphOffsetTable(Arc<GlyphOffsetTableInner>);
impl GlyphOffsetTable {
    pub fn get_offset<T: From<u16> + From<u32>>(&self, index: usize) -> Option<T> {
        self.0.get_offset(index)
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
pub enum GlyphOffsetTableInner {
    Short(Vec<u16>),
    Long(Vec<u32>),
}

impl GlyphOffsetTableInner {
    pub fn get_offset<T: From<u16> + From<u32>>(&self, index: usize) -> Option<T> {
        match self {
            GlyphOffsetTableInner::Short(items) => items.get(index).map(|a| a * 2).map(Into::into),
            GlyphOffsetTableInner::Long(items) => items.get(index).copied().map(Into::into),
        }
    }
    pub fn len(&self) -> usize {
        match self {
            GlyphOffsetTableInner::Short(items) => items.len(),
            GlyphOffsetTableInner::Long(items) => items.len(),
        }
    }
    pub fn is_empty(&self) -> bool {
        match self {
            GlyphOffsetTableInner::Short(items) => items.is_empty(),
            GlyphOffsetTableInner::Long(items) => items.is_empty(),
        }
    }
}
impl AsRef<GlyphOffsetTableInner> for GlyphOffsetTable {
    fn as_ref(&self) -> &GlyphOffsetTableInner {
        self.0.as_ref()
    }
}
impl SFNTTable for GlyphOffsetTable {
    fn into(self) -> super::AnySFNTTable {
        super::AnySFNTTable::GlyphOffset(self)
    }
}
impl DependentSFNTTable for GlyphOffsetTable {
    const TAGS: &[&crate::types::sfnt_types::index::TableTagInner] = &[b"glyf"];

    fn read(
        mut file: &mut crate::types::sfnt_types::SFNTFile,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let TableReference { offset, length, .. } = match file.get_table_reference(b"glyf") {
            Some(a) => a,
            None => {
                return Err(SFNTError::SomethingIsNotFound("Table", "[glyf]".to_string()).into());
            }
        };
        let length = length as usize;

        let header = file.get_table::<FontHeader>()?.clone();
        let read = &mut file.file;
        read.seek(std::io::SeekFrom::Start(offset as u64));

        let value = match header.index_to_loc_format {
            0 => GlyphOffsetTableInner::Short(u16::read_into_vec(read, length / size_of::<u16>())?),
            1 => GlyphOffsetTableInner::Long(u32::read_into_vec(read, length / size_of::<u32>())?),
            _ => todo!("Write an error",),
        };

        Ok(Self(Arc::new(value)))
    }
}
