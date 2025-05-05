use super::{FontProvider, filebasedprovider::FileProvider};

pub struct LocalProvider(FileProvider);
impl FontProvider for LocalProvider {
    type Criteria = <FileProvider as FontProvider>::Criteria;

    type FontReference = <FileProvider as FontProvider>::FontReference;
    const NAME: &str = "local";

    fn new(cfg: &crate::config::Config) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self(FileProvider::try_new([cfg
            .current_dir
            .to_str()
            .unwrap()])?))
    }

    fn find_fonts(
        &mut self,
        criteria: &Self::Criteria,
    ) -> Result<Vec<Self::FontReference>, Box<dyn std::error::Error>> {
        self.0.find_fonts(criteria)
    }
}
