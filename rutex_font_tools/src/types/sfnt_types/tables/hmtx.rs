use std::io::Seek;

use super::super::SFNTPrimitive;
use super::DependentSFNTTable;
use super::hhea::HorizontalHeaderTable;
use crate::sfnt_table;
use crate::types::sfnt_types::index::{SFNTError, TableReference};
use crate::types::sfnt_types::{IFWord, SFNTReadable};

#[derive(Debug, Clone, Copy)]
pub struct LongHorizontalMetric {
    pub advanced_width: u16,
    pub left_side_bearing: i16,
}
impl SFNTReadable for LongHorizontalMetric {
    fn read_from<R: std::io::Read + ?Sized + std::io::Seek>(
        read: &mut R,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(LongHorizontalMetric {
            advanced_width: u16::read_from(read)?,
            left_side_bearing: i16::read_from(read)?,
        })
    }
}
#[derive(Debug, Clone)]
pub struct HorizontalMetrics {
    pub long_horizontal_metrics: Vec<LongHorizontalMetric>,
    pub left_side_bearings: Vec<IFWord>,
}
sfnt_table!(HorizontalMetrics);

impl DependentSFNTTable for HorizontalMetrics {
    const TAGS: &[&crate::types::sfnt_types::index::TableTagInner] = &[b"htmx"];

    fn read(
        file: &mut crate::types::sfnt_types::SFNTFile,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let TableReference { length, offset, .. } = match file.get_table_reference(Self::TAGS[0]) {
            Some(o) => o,
            None => {
                return Err(SFNTError::SomethingIsNotFound("Table", Self::tags_to_string()).into());
            }
        };
        let num_of_long_horizontal_metrics = file
            .get_table::<HorizontalHeaderTable>()?
            .num_of_long_horizontal_metrics;

        let read = &mut file.file;
        read.seek(std::io::SeekFrom::Start(length as u64))?;
        let end_offset = (length + offset) as u64;
        let mut long_horizontal_metrics = Vec::new();
        for _ in 0..num_of_long_horizontal_metrics {
            long_horizontal_metrics.push(LongHorizontalMetric::read_from(read)?);
        }
        let position = read.stream_position()?;
        let len = (end_offset - position) / (size_of::<IFWord>() as u64);
        let left_side_bearings = IFWord::read_into_vec(read, len as usize)?;
        Ok(HorizontalMetrics {
            long_horizontal_metrics,
            left_side_bearings,
        })
    }
}
