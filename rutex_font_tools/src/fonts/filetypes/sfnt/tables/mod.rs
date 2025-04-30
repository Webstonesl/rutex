pub mod common_tables;
use std::fmt::Debug;

use super::{sfnt_primitives::FromSFNTBytes, SFNTFile};
use crate::error::*;
pub trait SFNTType: Sized {
    fn new(file: SFNTFile) -> Result<Self, Error>;
}

pub trait SFNTTableLike: Sized {
    fn read(tag: TableTag, reader: &mut SFNTFile) -> Result<Self, Error>;
}
pub enum SFNTTable<T: SFNTTableLike> {
    CustomTable(T),
}

impl<T: SFNTTableLike> SFNTTableLike for SFNTTable<T> {
    fn read(tag: TableTag, reader: &mut SFNTFile) -> Result<Self, Error> {
        match tag {
            _ => return Ok(SFNTTable::CustomTable(T::read(tag, reader)?)),
        }
    }
}

#[derive(Clone, Copy, PartialOrd, Ord, Hash, PartialEq, Eq)]
#[repr(transparent)]
pub struct TableTag(pub [u8; 4]);

impl FromSFNTBytes<4> for TableTag {
    const DEFAULT: Self = TableTag([0, 0, 0, 0]);

    fn convert_from_array(buf: &[u8; 4]) -> Self {
        TableTag(*buf)
    }
}
impl Debug for TableTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}",
            String::from_iter(self.0.iter().map(|a| *a as char))
        )
    }
}
