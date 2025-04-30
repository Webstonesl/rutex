use super::*;
struct CmapSegment {
    range: IncRange<u32>,
    value: u32,
}
pub struct CMAPSubTable13 {
    groups: Vec<CmapSegment>,
}
impl SubTableTrait<u32> for CMAPSubTable13 {

    fn read(data: &[u8], info: &CmapSubTablePrelim) -> Result<Self, Box<dyn Error>> {
        let mut cursor = Cursor::new(data);
        if cfg!(test) {
            let format = u16::read_from(&mut cursor)?;
            let language = u16::read_from(&mut cursor)?;
            let length = u32::read_from(&mut cursor)?;
            let _ = u32::read_from(&mut cursor)?;
            debug_assert_eq!(format, info.format);
            debug_assert_eq!(language, info.language);
            debug_assert_eq!(length as usize, data.len());
            debug_assert_eq!(cursor.position(), 12);
        } else {
            cursor.set_position(12);
        }

        let count = u32::read_from(&mut cursor)?;
        let mut groups = Vec::new();
        for _ in 0..count {
            let start = u32::read_from(&mut cursor)?;
            let end = u32::read_from(&mut cursor)?;
            let offset = u32::read_from(&mut cursor)?;
            groups.push(CmapSegment {
                range: IncRange { start, end },
                value: offset,
            });
        }

        Ok(CMAPSubTable13 { groups })
    }

    fn get_element(&self, c: &u32) -> u16 {
        match self.groups.binary_search_by(
            |CmapSegment {
                 range: IncRange { start, end },
                 ..
             }| {
                if c < start {
                    Ordering::Greater
                } else if c > end {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            },
        ) {
            Ok(a) => {
                if let Some(CmapSegment { value, .. }) = self.groups.get(a) {
                    *value as u16
                } else {
                    unreachable!()
                }
            }
            Err(_) => 0,
        }
    }

    fn get_mapping(&self) -> BTreeMap<u32, u16> {
        let mut map = BTreeMap::new();
        for CmapSegment { range, value } in self.groups.iter() {
            for k in range.into_range() {
                map.insert(k, *value as u16);
            }
        }
        map
    }
}
