use std::io::Read;

use super::*;

struct StateTableHeader {
    state_size: u16,
    class_table_offset: u16,
    state_array_offset: u16,
    entry_table_offset: u16,
}
impl StateTableHeader {
    fn read<R: Read>(read: &mut R) -> Result<StateTableHeader, Box<dyn Error>> {
        let state_size = u16::read_from(read)?;
        let class_table_offset = u16::read_from(read)?;
        let state_array_offset = u16::read_from(read)?;
        let entry_table_offset = u16::read_from(read)?;
        Ok(StateTableHeader {
            state_size,
            class_table_offset,
            state_array_offset,
            entry_table_offset,
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

impl KerningSubTable for Kern1 {
    type TableType = Self;

    fn new(data: Vec<u8>) -> Result<Arc<Self::TableType>, Box<dyn Error>> {
        let mut cur = Cursor::new(data);
        let read = &mut cur;
        let table = StateTableHeader::read(read)?;
        let mut value_table_offset = u16::read_from(read)?;

        todo!()
    }
}
