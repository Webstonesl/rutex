use super::SFNTTable;
use super::{super::SFNTPrimitive, ReadableSFNTTable};
use crate::types::sfnt_types::{IFWord, UFWord};

#[derive(Debug, Clone)]
pub struct HorizontalHeaderTable {
    pub ascent: IFWord,
    pub descent: IFWord,
    pub line_gap: IFWord,
    pub advance_width_max: UFWord,
    pub min_left_side_bearing: IFWord,
    pub min_right_side_bearing: IFWord,
    pub x_max_extent: IFWord,
    pub caret_slope_rise: i16,
    pub caret_slope_run: i16,
    pub caret_offset: i16,
    pub metric_data_format: i16,
    pub num_of_long_horizontal_metrics: u16,
}
impl SFNTTable for HorizontalHeaderTable {
    fn into(self) -> super::AnySFNTTable {
        super::AnySFNTTable::HorizontalHeaders(Box::new(self))
    }
}
impl ReadableSFNTTable for HorizontalHeaderTable {
    const TAGS: &[&crate::types::sfnt_types::index::TableTagInner] = &[b"hhea"];

    fn read<R: std::io::Read + std::io::Seek>(
        read: &mut R,
        _: u32,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let version = u32::read_from(read)?;
        debug_assert_eq!(version, 0x00010000);
        let ascent = IFWord::read_from(read)?;
        let descent = IFWord::read_from(read)?;
        let line_gap = IFWord::read_from(read)?;
        let advance_width_max = UFWord::read_from(read)?;
        let min_left_side_bearing = IFWord::read_from(read)?;
        let min_right_side_bearing = IFWord::read_from(read)?;
        let x_max_extent = IFWord::read_from(read)?;
        let caret_slope_rise = i16::read_from(read)?;
        let caret_slope_run = i16::read_from(read)?;
        let caret_offset = i16::read_from(read)?;
        for i in 0..4 {
            let v = u16::read_from(read)?;
            debug_assert!(v == 0, "Expected 0 for {i} found {v:?}");
        }
        let metric_data_format = i16::read_from(read)?;
        let num_of_long_horizontal_metrics = u16::read_from(read)?;

        Ok(Self {
            ascent,
            descent,
            line_gap,
            advance_width_max,
            min_left_side_bearing,
            min_right_side_bearing,
            x_max_extent,
            caret_slope_rise,
            caret_slope_run,
            caret_offset,
            metric_data_format,
            num_of_long_horizontal_metrics,
        })
    }
}
