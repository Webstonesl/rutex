use std::{any::Any, collections::BTreeMap};

use error::SFNTError;
use header::{SFNTReference, SFNTScalar, SFNTTag};
use reading::ReadAndSeek;

// #![warn(missing_docs)]
pub mod error;
pub mod header;
pub mod reading;

pub trait SFNTTable: Any {
    fn first_tag(&self) -> SFNTTag;
}

pub struct SFNTFile {
    file: Box<dyn ReadAndSeek>,
    index: BTreeMap<SFNTTag, SFNTReference>,
}
impl SFNTFile {
    pub fn new<R: ReadAndSeek + 'static>(file: R) -> Result<SFNTFile, SFNTError> {
        let mut result = Self {
            file: Box::new(file),
            index: Default::default(),
        };
        result.read_index()?;
        Ok(result)
    }
    fn read_index(&mut self) -> Result<(), SFNTError> {
        self.file.seek(std::io::SeekFrom::Start(0))?;

        Ok(())
    }
}
