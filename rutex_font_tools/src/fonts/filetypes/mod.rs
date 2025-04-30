use sfnt::tables::common_tables::LanguageType;

use crate::error::Error;

pub mod sfnt;
pub trait FontTrait {
    fn get_name(&mut self, language: &dyn LanguageType) -> Result<String, Error>;
}
