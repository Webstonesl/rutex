//! <b>Kerning Subtable 0 implementation</b>
//!
//! Implemented using [https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6kern.html]
//! and referenced [https://learn.microsoft.com/en-us/typography/opentype/spec/kern#format-0]

use super::*;
#[derive(Clone)]
pub(super) struct Kern0 {
    values: BTreeMap<u32, i16>,
}
impl KerningSubTable for Kern0 {
    type TableType = Self;
    fn new(data: Vec<u8>) -> Result<Arc<Self>, Box<dyn Error>> {
        let read = &mut Cursor::new(&data);
        let pair_count = u16::read_from(read)?;
        let _ = u16::read_from(read)?;
        let _ = u16::read_from(read)?;
        let _ = u16::read_from(read)?;
        let mut values = BTreeMap::new();
        for _ in 0..pair_count {
            values.insert(u32::read_from(read)?, i16::read_from(read)?);
        }
        Ok(Arc::new(Self { values }))
    }
}
impl KerningSetup for Kern0 {
    fn get_values(&self, source: &[u16]) -> Vec<i16> {
        source[..(source.len() - 1)]
            .iter()
            .zip(&source[1..])
            .map(|(a, b)| ((a.to_be() as u32) << 16) | (b.to_be() as u32))
            .map(|a| *self.values.get(&a).unwrap())
            .collect()
    }
}
