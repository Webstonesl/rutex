use std::fmt::Debug;

use crate::reading::ReadAndSeek;
/// A Struct for referencing a table tag which is a 4 byte string.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[repr(transparent)]
pub struct SFNTTag([u8; 4]);
pub type SFNTScalar = SFNTTag;

impl AsRef<[u8; 4]> for SFNTTag {
    fn as_ref(&self) -> &[u8; 4] {
        &self.0
    }
}

impl Debug for SFNTTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "b{:?}",
            self.0.iter().map(|a| *a as char).collect::<String>()
        )
    }
}
pub struct SFNTReference {
    pub checksum: u64,
    pub offset: u64,
    pub length: u64,
}
impl SFNTReference {
    /// Goes to the location of the table in the file.
    #[inline]
    pub fn seek(&self, file: &mut impl ReadAndSeek) -> Result<u64, std::io::Error> {
        file.seek(std::io::SeekFrom::Start(self.offset))
    }
}
