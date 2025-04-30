use chrono::{Duration, NaiveDate, NaiveDateTime, NaiveTime};
use num::PrimInt;

use crate::{
    error::{Error, ErrorKind},
    pdf::maths::{One, Zero},
};
use std::{
    fmt::{Debug, Display},
    io::Read,
    ops::{Add, Deref},
};

pub trait FromSFNTBytes<const N: usize>: Sized + Copy {
    const DEFAULT: Self;

    fn convert_from_array(buf: &[u8; N]) -> Self;
    /// # Safety
    /// Slice must be the correct length
    unsafe fn convert_from_slice(buf: &[u8]) -> Self {
        Self::convert_from_array(buf.try_into().expect("Slice with incorrect length"))
    }

    fn read_from_into_array<T: Read, const COUNT: usize>(
        read: &mut T,
    ) -> Result<[Self; COUNT], Error> {
        let mut v = vec![0u8; N * COUNT];
        let mut a = 0;
        while a < N * COUNT {
            a += read.read(&mut v)?;
        }
        let mut result = [Self::DEFAULT; COUNT];
        for (range, result) in result
            .iter_mut()
            .enumerate()
            .map(|(index, result)| ((index * N)..((index + 1) * N), result))
        {
            *result =
                Self::convert_from_array(&v[range].try_into().expect("Should not be a problem"));
        }
        Ok(result)
    }
    fn read_n_into_vec<T: Read, I: Add<I, Output = I> + Ord + Copy + Sized + One + Zero>(
        read: &mut T,
        count: I,
    ) -> Result<Vec<Self>, Error> {
        let mut buf = [0u8; N];
        let mut vec = Vec::new();
        let mut i = I::ZERO;
        while i < count {
            if read.read(&mut buf)? != N {
                return Err(Error::new_with_message(
                    ErrorKind::IoError,
                    "Insufficient Bytes read",
                ));
            }
            vec.push(Self::convert_from_array(&buf));
            i = i + I::ONE;
        }
        Ok(vec)
    }

    fn read_from<T: Read>(read: &mut T) -> Result<Self, Error> {
        assert!(N > 0);
        let mut buf = [0u8; N];
        let mut a = 0;
        while a < N {
            a += match read.read(&mut buf[a..])? {
                0 => return Err(Error::new_with_message(ErrorKind::IoError, "No Bytes Read")),
                a => a,
            }
        }
        Ok(Self::convert_from_array(&buf))
    }
}

impl FromSFNTBytes<1> for u8 {
    const DEFAULT: Self = 0;

    fn convert_from_array(buf: &[u8; 1]) -> Self {
        buf[0]
    }
}
impl FromSFNTBytes<2> for i16 {
    const DEFAULT: Self = 0;

    fn convert_from_array(buf: &[u8; 2]) -> Self {
        i16::from_be_bytes(*buf)
    }
}
impl FromSFNTBytes<2> for u16 {
    const DEFAULT: Self = 0;
    fn convert_from_array(buf: &[u8; 2]) -> Self {
        u16::from_be_bytes(*buf)
    }
}
impl FromSFNTBytes<4> for u32 {
    const DEFAULT: Self = 0;
    fn convert_from_array(buf: &[u8; 4]) -> Self {
        u32::from_be_bytes(*buf)
    }
}
impl FromSFNTBytes<4> for i32 {
    const DEFAULT: Self = 0;
    fn convert_from_array(buf: &[u8; 4]) -> Self {
        i32::from_be_bytes(*buf)
    }
}
impl FromSFNTBytes<8> for i64 {
    const DEFAULT: Self = 0;
    fn convert_from_array(buf: &[u8; 8]) -> Self {
        i64::from_be_bytes(*buf)
    }
}

macro_rules! A {
    ($default:expr, $bytes:literal, $between:ty, struct $name:ident($tp:ty)) => {
        #[repr(transparent)]
        #[derive(Clone, Copy)]
        pub struct $name($tp);
        impl Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                Debug::fmt(&self.0, f)
            }
        }
        impl Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                Display::fmt(&self.0, f)
            }
        }
        impl Into<$tp> for $name {
            fn into(self) -> $tp {
                self.0
            }
        }
        impl Deref for $name {
            type Target = $tp;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl FromSFNTBytes<$bytes> for $name {
            const DEFAULT: Self = Self($default);
            fn convert_from_array(buf: &[u8; $bytes]) -> Self {
                Self::new(<$between>::convert_from_array(buf))
            }
        }
    };
}
A!(0.0, 2, i16, struct ShortFrac(f64));

impl ShortFrac {
    const MULTIPLIER: f64 = 16384.0;
    fn new(value: i16) -> Self {
        Self((value as f64) / Self::MULTIPLIER)
    }
}

impl From<u16> for ShortFrac {
    fn from(value: u16) -> Self {
        Self::new(i16::from_ne_bytes(value.to_ne_bytes()))
    }
}
impl TryFrom<f64> for ShortFrac {
    type Error = Error;
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if (-2.0 <= value) && (value < 2.0) {
            Ok(ShortFrac(value))
        } else {
            Err(Error::new_with_message(
                ErrorKind::IllegalParameter,
                format!("Short frac can only represent [-2.0,2) not {value}"),
            ))
        }
    }
}
#[cfg(test)]
const VERBOSE: bool = true;
#[test]
fn test_short_frac() -> Result<(), ()> {
    let mut success = true;
    let lower = [
        (1.999, 0x7fffu16, 0.001),
        (-2.0, 0x8000, 0.001),
        (1.0, 0x4000, 0.001),
        (-1.0, 0xc000, 0.001),
    ];
    for (target, raw, discr) in lower {
        let actual = *ShortFrac::from(raw);
        let off = (target - actual).abs();
        if off >= discr {
            success = false;
            eprintln!("{raw:04x?} should give {target} but gave {actual}")
        } else if VERBOSE {
            eprintln!("{raw:04x?} gave {target} within {off}")
        }
    }
    if success {
        Result::Ok(())
    } else {
        Result::Err(())
    }
}

A!(0.0,4,i32, struct Fixed(f64));
impl Fixed {
    const MULTIPLIER: f64 = u16::MAX as f64;
    pub fn new(value: i32) -> Self {
        Self((value as f64) / Self::MULTIPLIER)
    }
}
A!(0,2,i16, struct FWord(i16));
impl FWord {
    #[inline(always)]
    fn new(value: i16) -> Self {
        Self(value)
    }
}
A!(0,2,u16, struct UFWord(u16));
impl UFWord {
    fn new(value: u16) -> Self {
        Self(value)
    }
}
pub type F2Dot14 = ShortFrac;
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct LongDateTime(NaiveDateTime);
impl LongDateTime {
    const EPOCH: NaiveDateTime = NaiveDateTime::new(
        NaiveDate::from_ymd_opt(1904, 1, 1).unwrap(),
        NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
    );
    pub fn new(a: i64) -> Self {
        let v = Self::EPOCH + Duration::seconds(a);
        Self(v)
    }
}
impl Deref for LongDateTime {
    type Target = NaiveDateTime;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl FromSFNTBytes<8> for LongDateTime {
    const DEFAULT: Self = LongDateTime(Self::EPOCH);

    fn convert_from_array(buf: &[u8; 8]) -> Self {
        Self::new(i64::convert_from_array(buf))
    }
}
