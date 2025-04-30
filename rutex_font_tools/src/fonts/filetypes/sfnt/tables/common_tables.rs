use crate::{error::Error, fonts::filetypes::sfnt::SeekAndBufRead};

pub trait CommonTable: Sized {
    fn read_table<T: SeekAndBufRead>(file: &mut T, offset: u32) -> Result<Self, Error>;
}

pub mod name;
pub use name::*;

pub mod fond;
pub use fond::*;

pub mod bhed;
pub mod cmap;
pub mod head;
#[cfg(test)]
pub mod test {

    use crate::error::{Error, ErrorKind};
    use crate::fonts::filetypes::sfnt::tables::TableTag;
    use crate::fonts::filetypes::sfnt::SFNTReader;

    use crate::fonts::providers::osfonts::OSFontProvider;

    use std::any::Any;
    use std::fs::OpenOptions;

    use super::CommonTable;

    pub fn test_mega<T: Any + CommonTable>(t: &TableTag) -> Result<Vec<T>, Error> {
        let f = OSFontProvider::default().unwrap();
        let mut results = Vec::new();
        for file in f.files {
            if file.extension().unwrap() == "ttf" {
                results.push(
                    match SFNTReader::read_from_file(
                        OpenOptions::new().read(true).open(file).unwrap(),
                    )
                    .unwrap()
                    .get_table(t)
                    {
                        Ok(t) => *t.downcast().unwrap(),
                        Err(Error {
                            kind: ErrorKind::DoesNotExistError,
                            ..
                        }) => continue,
                        Err(e) => return Err(e),
                    },
                );
            }
            // println!("{:?}", file);
        }
        Ok(results)
    }
}
