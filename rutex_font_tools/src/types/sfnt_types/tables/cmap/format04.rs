use super::*;
use std::fmt::Debug;

#[derive(Clone, Copy, Debug)]
struct CMapSegment {
    range: IncRange<u16>,
    delta: u16,
    range_offset: u16,
}
impl From<(((u16, u16), u16), u16)> for CMapSegment {
    #[inline(always)]
    fn from((((start, end), delta), range_offset): (((u16, u16), u16), u16)) -> Self {
        Self {
            range: IncRange { start, end },
            delta,
            range_offset,
        }
    }
}
#[derive(Clone)]
pub struct CMAPSubTable4 {
    segments: Vec<CMapSegment>,
    data: Vec<u16>,
}

impl CMAPSubTable4 {
    fn new<
        T1: IntoIterator<Item = u16>,
        T2: IntoIterator<Item = u16>,
        T3: IntoIterator<Item = u16>,
        T4: IntoIterator<Item = u16>,
    >(
        start_codes: T1,
        end_codes: T2,
        delta: T3,
        range_offset: T4,
        data: Vec<u16>,
    ) -> Self {
        Self {
            segments: start_codes
                .into_iter()
                .zip(end_codes)
                .zip(delta)
                .zip(range_offset)
                .map(CMapSegment::from)
                .collect(),
            data,
        }
    }
}
impl SubTableReadableTrait<u16> for CMAPSubTable4 {
    fn read(data: &[u8], extrainfo: &CmapSubTablePrelim) -> Result<Self, Box<dyn Error>> {
        let mut cursor = Cursor::new(&data);
        if cfg!(test) {
            let format = u16::read_from(&mut cursor)?;
            let length = u16::read_from(&mut cursor)?;
            let language = u16::read_from(&mut cursor)?;
            debug_assert_eq!(format, 4);
            debug_assert_eq!(length as usize, data.len());
            debug_assert_eq!(language, extrainfo.language);
            debug_assert_eq!(cursor.position(), 6);
        } else {
            cursor.set_position(6);
        }
        let seg_count = cursor.get_prim::<2, u16>()? / 2;

        let _ = cursor.get_prim::<2, u16>()?;
        let _ = cursor.get_prim::<2, u16>()?;
        let _ = cursor.get_prim::<2, u16>()?;
        //
        let end_code = u16::read_into_vec(&mut cursor, seg_count as usize)?;
        let padding = cursor.get_prim::<2, u16>()?;
        debug_assert_eq!(padding, 0);
        let start_code = u16::read_into_vec(&mut cursor, seg_count as usize)?;
        let id_delta = u16::read_into_vec(&mut cursor, seg_count as usize)?;
        let id_range_offset = u16::read_into_vec(&mut cursor, seg_count as usize)?;
        let glyph_data: Vec<u16> = data[cursor.position() as usize..]
            .chunks_exact(2)
            .map(|a| u16::from_be_bytes(*a.as_array::<2>().unwrap()))
            .collect();
        Ok(Self::new(
            start_code,
            end_code,
            id_delta,
            id_range_offset,
            glyph_data,
        ))
    }
    fn get_element(&self, c: &u16) -> u16 {
        let (
            segment_index,
            CMapSegment {
                range: IncRange { start, .. },
                delta,
                range_offset,
            },
        ) = match self.segments.binary_search_by(
            |CMapSegment {
                 range: IncRange { start, end },
                 ..
             }| {
                if c > end {
                    Ordering::Less
                } else if c < start {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            },
        ) {
            Ok(a) => (a, self.segments.get(a).unwrap()),
            Err(_) => return 0,
        };
        let c = *c;
        if *range_offset == 0 {
            return delta.wrapping_add(c);
        };
        let character_offset = c - *start;
        let offset =
            (*range_offset / 2 + character_offset) - (self.segments.len() - segment_index) as u16;
        self.data[offset as usize]
    }

    fn get_mapping(&self) -> BTreeMap<u16, u16> {
        let mut result = BTreeMap::new();
        let segment_count = self.segments.len();
        for (
            segment_index,
            CMapSegment {
                range,
                delta,
                range_offset,
            },
        ) in self.segments.iter().enumerate()
        {
            if range_offset == &0 {
                result.extend(range.into_range().map(|a| (a, delta.wrapping_add(a))))
            } else {
                for (j, character_offset) in range.into_range().map(|a| (a, a - range.start)) {
                    let offset = (*range_offset / 2 + character_offset)
                        - (segment_count - segment_index) as u16;
                    result.insert(j, self.data[offset as usize]);
                }
            }
        }
        result
    }

    fn write_into(&self, map: &mut BTreeMap<u32, u32>) {
        map.extend(
            SubTableReadableTrait::<u16>::get_mapping(self)
                .into_iter()
                .map(|(a, b)| (a as u32, b as u32)),
        )
    }
}
