use super::CommonTable;
use crate::fonts::filetypes::sfnt::FromSFNTBytes;
use num::PrimInt;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum FondTableEntryDescription {
    Fond { family_id: i16 },
    Nfnt,
}
#[derive(Clone, Copy, Debug)]
#[allow(dead_code)]
pub enum TableValue<T: Sized, E: PrimInt> {
    UnInit { offset: E, length: u32 },
    Value(T),
}
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct FondTableEntry {
    pub resource_id: i16,
    pub flags: u16,
    pub style: u16,
    pub script: i16,
    pub language: i16,
    pub name: String,
    pub value: TableValue<(), u32>,
    pub description: FondTableEntryDescription,
}
impl FondTableEntry {
    pub fn read_entry<T: crate::fonts::filetypes::sfnt::SeekAndBufRead>(&mut self, _: &mut T) {}
}

#[allow(dead_code)]
pub struct FondTable(Vec<FondTableEntry>);
impl FondTable {}
impl CommonTable for FondTable {
    #[allow(unused_variables)]
    fn read_table<T: crate::fonts::filetypes::sfnt::SeekAndBufRead>(
        file: &mut T,
        table_offset: u32,
    ) -> Result<Self, crate::error::Error> {
        file.seek(std::io::SeekFrom::Start(table_offset as u64))?;
        let version = u16::read_from(file)?;
        let n_fond = u16::read_from(file)?;
        let n_nfnt = u16::read_from(file)?;

        let a = u16::read_from(file)?;
        assert!(a == 0);
        let mut entries = Vec::new();
        for _ in 0..n_fond {
            let resource_id: i16 = i16::read_from(file)?;
            let family_id: i16 = i16::read_from(file)?;
            let flags: u16 = u16::read_from(file)?;
            let style: u16 = u16::read_from(file)?;
            let script: i16 = i16::read_from(file)?;
            let language: i16 = i16::read_from(file)?;
            let offset: u32 = u32::read_from(file)?;
            let length: u32 = u32::read_from(file)?;
            let name_length: u8 = u8::read_from(file)?;
            let name: [u8; 255] = u8::read_from_into_array::<T, 255>(file)?;
            let name: String =
                String::from_iter(name[..(name_length as usize)].iter().map(|a| *a as char));
            entries.push(FondTableEntry {
                resource_id,
                flags,
                style,
                script,
                language,
                name,
                value: TableValue::UnInit {
                    offset: offset + table_offset,
                    length,
                },
                description: FondTableEntryDescription::Fond { family_id },
            });
        }
        for _ in 0..n_nfnt {
            let resource_id = i16::read_from(file)?;
            u16::read_from(file)?;
            let flags = u16::read_from(file)?;
            let style = u16::read_from(file)?;
            let script = i16::read_from(file)?;
            let language = i16::read_from(file)?;
            let offset = u32::read_from(file)?;
            let length = u32::read_from(file)?;
            let name_length = u8::read_from(file)?;
            let name = u8::read_from_into_array::<T, 255>(file)?;
            let name = String::from_iter(name[..(name_length as usize)].iter().map(|a| *a as char));
            entries.push(FondTableEntry {
                resource_id,
                flags,
                style,
                script,
                language,
                name,
                value: TableValue::UnInit {
                    offset: offset + table_offset,
                    length,
                },
                description: FondTableEntryDescription::Nfnt,
            });
        }
        Ok(FondTable(entries))
    }
}
