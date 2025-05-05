use super::*;

#[derive(Clone)]
pub(super) struct Kern3 {}
impl KerningSetup for Kern3 {
    fn get_values(&self, source: &[u16]) -> Vec<i16> {
        todo!()
    }
}
impl KerningSubTable for Kern3 {
    type TableType = Kern3;

    fn new(data: Vec<u8>) -> Result<Arc<Self::TableType>, Box<dyn Error>> {
        todo!()
    }
}
