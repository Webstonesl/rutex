use super::*;
#[derive(Clone)]
pub struct CMAPSubTable0(Box<[u8; 256]>);
impl SubTableReadableTrait<u8> for CMAPSubTable0 {
    fn read(data: &[u8], extrainfo: &CmapSubTablePrelim) -> Result<Self, Box<dyn Error>> {
        let mut cursor = Cursor::new(data);
        if cfg!(test) {
            let format = u16::read_from(&mut cursor)?;
            let length = u16::read_from(&mut cursor)?;
            let language = u16::read_from(&mut cursor)?;
            debug_assert_eq!(format, 0);
            debug_assert_eq!(length, extrainfo.range.len() as u16);
            debug_assert_eq!(extrainfo.language, language);
            debug_assert_eq!(length, 262);
            debug_assert_eq!(cursor.position(), 6);
        } else {
            cursor.set_position(6);
        }
        let mut value = [0u8; 256];
        cursor.read_exact(&mut value)?;
        Ok(Self(Box::new(value)))
    }

    fn get_element(&self, c: &u8) -> u16 {
        self.0[*c as usize] as u16
    }

    fn get_mapping(&self) -> BTreeMap<u8, u16> {
        let mut value = BTreeMap::new();
        for i in 0..=0xFF {
            value.insert(i, self.0[i as usize] as u16);
        }

        value
    }

    fn write_into(&self, map: &mut BTreeMap<u32, u32>) {
        for (i, v) in self.0.iter().copied().enumerate() {
            map.insert(i as u32, v as u32);
        }
    }
}
