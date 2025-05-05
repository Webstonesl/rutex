#[macro_use]
pub mod error;

pub mod convert;
pub mod traits;
#[test]
fn test() {
    dbg!(new_error!(ConversionError));
}
