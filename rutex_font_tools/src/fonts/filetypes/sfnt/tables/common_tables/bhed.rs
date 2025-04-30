pub type BHeadTable = super::head::HeadTable;

#[cfg(test)]
use super::test::*;

#[cfg(test)]
use crate::fonts::filetypes::sfnt::tables::TableTag;
#[test]
fn test() {
    let result = match test_mega::<BHeadTable>(&TableTag(*b"bhed")) {
        Ok(ok) => ok,
        Err(e) => {
            println!("Error {}", e);
            return;
        }
    };
    dbg!(result);
}
