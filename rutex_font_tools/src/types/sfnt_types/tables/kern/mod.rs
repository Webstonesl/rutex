//! <b>Kerning Table implementation</b>
//!
//! Implemented using
//! [https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6kern.html]
//! and referenced
//! [https://learn.microsoft.com/en-us/typography/opentype/spec/kern#format-0]
//! Subtable traits are mostly there for standardisation of subtables. Also
//! helps to use them dynamically.

use std::{any::Any, collections::BTreeMap, error::Error, io::Cursor, sync::Arc};

use crate::types::sfnt_types::index::SFNTError;

use super::{super::SFNTPrimitive, ReadableSFNTTable, SFNTTable};
#[derive(Clone)]
pub struct KerningTable {
    results: Vec<(u16, Arc<dyn KerningSetup>)>,
}
impl SFNTTable for KerningTable {
    fn into(self) -> super::AnySFNTTable {
        super::AnySFNTTable::KerningTable(Box::new(self))
    }
}
impl ReadableSFNTTable for KerningTable {
    const TAGS: &[&crate::types::sfnt_types::index::TableTagInner] = &[b"kern"];

    fn read<R: std::io::Read + std::io::Seek>(
        read: &mut R,
        length: u32,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let version = u16::read_from(read)?;
        let use_ms_format = match version {
            0 => true,
            1 => false,
            a => {
                return Err(SFNTError::InvalidVersionTag(format!(
                    "Invalid version for kerning table expeceted 0 or 1 got {a}"
                ))
                .into());
            }
        };

        let table_count = if use_ms_format {
            u16::read_from(read)? as u32
        } else {
            assert_eq!(u16::read_from(read)?, 0);
            u32::read_from(read)?
        };
        eprintln!("version = {version}, table count = {table_count}");
        let mut results = Vec::new();

        for _ in 0..table_count {
            let (version, length, coverage, tupleindex) = if use_ms_format {
                (
                    u16::read_from(read)?,
                    u16::read_from(read)? as u32,
                    u16::read_from(read)?,
                    0,
                )
            } else {
                (
                    0,
                    u32::read_from(read)?,
                    u16::read_from(read)?,
                    u16::read_from(read)?,
                )
            };
            let length = length - 8;
            let mut buf = vec![0; length as usize];
            read.read_exact(&mut buf)?;
            results.push((
                coverage & !0xFF,
                match coverage & 0xFF {
                    // ! Getting the format
                    0 => Kern0::dyn_new(buf)?,
                    1 => Kern1::dyn_new(buf)?,
                    2 => Kern2::dyn_new(buf)?,
                    3 => Kern3::dyn_new(buf)?,
                    format @ 4.. => {
                        return Err(SFNTError::Invalid(
                            "kerning subtable format",
                            format.to_string(),
                        )
                        .into());
                    }
                },
            ));
        }
        Ok(KerningTable { results })
    }
}
pub trait KerningSetup: Any {
    fn get_values(&self, source: &[u16]) -> Vec<i16>;
}
pub trait KerningSubTable: Any + Clone {
    type TableType: KerningSetup;
    fn new(data: Vec<u8>) -> Result<Arc<Self::TableType>, Box<dyn Error>>;
    fn dyn_new(data: Vec<u8>) -> Result<Arc<dyn KerningSetup>, Box<dyn Error>> {
        Self::new(data).map(|a| a as Arc<dyn KerningSetup>)
    }
}

pub mod kern0;
pub mod kern1;
pub mod kern2;
pub mod kern3;
use kern0::*;
use kern1::*;
use kern2::*;
use kern3::*;
