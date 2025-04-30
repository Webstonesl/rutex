use crate::{error::Error, fonts::filetypes::FontTrait};

use super::{
    tables::{
        common_tables::{
            bhed::BHeadTable, cmap::CMAPTable, head::HeadTable, CommonTable, LanguageType,
            NameTable,
        },
        SFNTType,
    },
    SFNTFile, SFNTFont, TableReference,
};

pub struct OpenType {
    file: SFNTFile,
}
impl SFNTFont for OpenType {
    fn get_table(
        &mut self,
        tag: &super::tables::TableTag,
    ) -> Result<Box<dyn std::any::Any>, crate::error::Error> {
        let TableReference { offset, .. } = self.file.get_table(tag).ok_or_else(|| {
            Error::new_with_message(
                crate::error::ErrorKind::DoesNotExistError,
                format!("Table {:?} does not exist", tag),
            )
        })?;
        match &tag.0 {
            b"name" => NameTable::read_table(&mut self.file.file, offset)
                .map(|table| Box::new(table) as Box<dyn std::any::Any>),
            b"head" => HeadTable::read_table(&mut self.file.file, offset)
                .map(|table| Box::new(table) as Box<dyn std::any::Any>),
            b"bhed" => BHeadTable::read_table(&mut self.file.file, offset)
                .map(|table| Box::new(table) as Box<dyn std::any::Any>),
            b"cmap" => CMAPTable::read_table(&mut self.file.file, offset)
                .map(|table| Box::new(table) as Box<dyn std::any::Any>),
            a => Err(Error::new_with_message(
                crate::error::ErrorKind::DoesNotExistError,
                format!("Invalid Table tag {a:?}"),
            )),
        }
    }
}

impl SFNTType for OpenType {
    fn new(file: SFNTFile) -> Result<Self, crate::error::Error> {
        Ok(Self { file })
    }
}
