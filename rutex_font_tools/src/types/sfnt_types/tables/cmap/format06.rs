use super::*;
pub struct CMAPSubTable6 {
    range: Range<u16>,
    index_array: Vec<u16>,
}
impl SubTableTrait<u16> for CMAPSubTable6 {
    fn read(data: &[u8], extrainfo: &CmapSubTablePrelim) -> Result<Self, Box<dyn Error>> {
        let mut cursor = Cursor::new(data);
        if cfg!(test) {
            let format = u16::read_from(&mut cursor)?;
            let length = u16::read_from(&mut cursor)?;
            let language = u16::read_from(&mut cursor)?;
            debug_assert_eq!(format, 6);
            debug_assert_eq!(length, extrainfo.range.len() as u16);
            debug_assert_eq!(extrainfo.language, language);
            debug_assert_eq!(cursor.position(), 6);
        } else {
            cursor.set_position(6);
        }
        let first_code = u16::read_from(&mut cursor)?;
        let num_entries = u16::read_from(&mut cursor)?;
        let index_array = u16::read_into_vec(&mut cursor, num_entries as usize)?;
        Ok(Self {
            range: first_code..(first_code + num_entries),
            index_array,
        })
    }

    fn get_element(&self, c: &u16) -> u16 {
        if self.range.contains(c) {
            self.index_array[(c - self.range.start) as usize]
        } else {
            0
        }
    }

    fn get_mapping(&self) -> BTreeMap<u16, u16> {
        BTreeMap::from_iter(self.range.clone().zip(self.index_array.iter().copied()))
    }
}
