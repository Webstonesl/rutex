use std::io::Read;

pub trait ReadAndSeek: std::io::Read + std::io::Seek {}

impl<T: std::io::Read + std::io::Seek> ReadAndSeek for T {}
pub trait Readable: Sized {
    type Error: std::error::Error + From<std::io::Error>;
    fn read_from<R: Read>(source: &mut R) -> Result<Self, Self::Error>;
    fn read_from_into<R: Read>(
        source: &mut R,
        destination: &mut [Self],
    ) -> Result<(), Self::Error> {
        for a in destination.iter_mut() {
            *a = Self::read_from(source)?;
        }
        Ok(())
    }
}
macro_rules! primitive_readable {
    ($tp:ty) => {
        impl Readable for $tp {
            type Error = std::io::Error;
            fn read_from<R: Read>(source: &mut R) -> Result<Self, Self::Error> {
                let mut buf = [0u8; size_of::<$tp>()];
                source.read_exact(&mut buf)?;
                Ok(<$tp>::from_be_bytes(buf))
            }
        }
    };
}
primitive_readable!(i8);
primitive_readable!(u8);
primitive_readable!(i16);
primitive_readable!(u16);
primitive_readable!(i32);
primitive_readable!(u32);
primitive_readable!(i64);
primitive_readable!(u64);
primitive_readable!(i128);
primitive_readable!(u128);
primitive_readable!(isize);
primitive_readable!(usize);

// impl Readable for u8 {
//     type Error = std::io::Error;
//     fn read_from<R: Read>(source: &mut R) -> Result<Self, Self::Error> {
//         let mut buf = [0u8; 1];
//         source.read_exact(&mut buf)?;
//         Ok(u8::from_be_bytes(buf))
//     }
// }
