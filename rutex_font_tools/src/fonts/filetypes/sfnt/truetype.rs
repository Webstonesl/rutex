use crate::{
    error::{Error, ErrorKind},
    fonts::filetypes::{
        sfnt::{
            tables::common_tables::{CommonTable, FondTable, NameInfo, NameTable},
            TableReference,
        },
        FontTrait,
    },
};

use super::{
    tables::{
        common_tables::{cmap::CMAPTable, head::HeadTable, LanguageType, NameType},
        SFNTTableLike, SFNTType, TableTag,
    },
    SFNTFile, SFNTFont,
};
pub enum TrueTypeTables {}

impl SFNTFont for TrueType {
    fn get_table(
        &mut self,
        tag: &super::tables::TableTag,
    ) -> Result<Box<dyn std::any::Any>, crate::error::Error> {
        let tag_ = tag;
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
            b"bhed" => HeadTable::read_table(&mut self.file.file, offset)
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
impl SFNTTableLike for TrueTypeTables {
    fn read(tag: TableTag, reader: &mut super::SFNTFile) -> Result<Self, crate::error::Error> {
        todo!()
    }
}
pub struct TrueType {
    file: SFNTFile,
}
impl SFNTType for TrueType {
    fn new(file: super::SFNTFile) -> Result<Self, crate::error::Error> {
        Ok(Self { file })
    }
}
