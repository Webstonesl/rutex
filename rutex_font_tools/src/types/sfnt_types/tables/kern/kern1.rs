use std::{
    io::{Read, Seek},
    ops::Range,
};

use super::*;

struct StateTableHeader {
    starting_position: u64,
    state_size: u16,
    class_table_offset: u16,
    state_array_offset: u16,
    entry_table_offset: u16,
}
impl StateTableHeader {
    fn read<R: Read + Seek>(read: &mut R) -> Result<StateTableHeader, Box<dyn Error>> {
        let starting_position = read.stream_position()?;
        let state_size = u16::read_from(read)?;
        let class_table_offset = u16::read_from(read)?;
        let state_array_offset = u16::read_from(read)?;
        let entry_table_offset = u16::read_from(read)?;
        Ok(StateTableHeader {
            starting_position,
            state_size,
            class_table_offset,
            state_array_offset,
            entry_table_offset,
        })
    }
    fn seek_class<R: Seek>(&self, read: &mut R) -> Result<(), Box<dyn Error>> {
        read.seek(std::io::SeekFrom::Start(
            self.starting_position + (self.class_table_offset as u64),
        ))?;
        Ok(())
    }
}
#[derive(Clone, Debug)]
struct ClassSubTable {
    range: Range<u16>,
    values: Vec<u8>,
}
impl ClassSubTable {
    fn read<R: Read + Seek>(read: &mut R) -> Result<Self, Box<dyn Error>> {
        let start = u16::read_from(read)?;
        let n_glyphs = u16::read_from(read)?;
        let values = u8::read_into_vec(read, n_glyphs as usize)?;
        Ok(ClassSubTable {
            range: start..(start + n_glyphs),
            values,
        })
    }
}

#[derive(Clone)]
pub(super) struct Kern1 {}
impl KerningSetup for Kern1 {
    fn get_values(&self, source: &[u16]) -> Vec<i16> {
        todo!()
    }
}
impl Kern1 {}

impl KerningSubTable for Kern1 {
    type TableType = Self;

    fn new(data: Vec<u8>) -> Result<Arc<Self::TableType>, Box<dyn Error>> {
        let mut cur = Cursor::new(data);
        let read = &mut cur;
        let table = StateTableHeader::read(read)?;
        dbg!(table.state_size);
        let mut value_table_offset = u16::read_from(read)?;

        table.seek_class(read)?;
        let cst = dbg!(ClassSubTable::read(read)?);
        if cfg!(test) {
            for c in cst.values.iter().copied() {
                assert!(c < table.state_size as u8);
            }
        }
        todo!()
    }
}
