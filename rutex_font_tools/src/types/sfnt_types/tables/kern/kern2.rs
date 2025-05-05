use super::*;

#[derive(Clone)]
pub(super) struct Kern2 {}
impl KerningSetup for Kern2 {
    fn get_values(&self, source: &[u16]) -> Vec<i16> {
        todo!()
    }
}
impl KerningSubTable for Kern2 {
    type TableType = Self;

    fn new(data: Vec<u8>) -> Result<Arc<Self::TableType>, Box<dyn Error>> {
        let mut read = Cursor::new(data);
        todo!()
    }
}
