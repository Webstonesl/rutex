use utility_macros::gen_error_types;
#[gen_error_types]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(usize)]
pub enum ErrorKind {
    ConversionError,
    MathsError,
    OverflowError,
}

// type Error =
#[test]
fn test() {
    let a = new_error!(ConversionError, format!("Apple {e}", e = "e"));
    dbg!(a);
}
