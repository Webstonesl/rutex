use std::{
    any::Any,
    collections::BTreeMap,
    error::Error,
    fmt::{Debug, Display},
    fs::File,
    io::{Read, Seek},
    mem::MaybeUninit,
};

use index::{SFNTError, SFNTHeader, TableReference, TableTagInner};
use tables::{DependentSFNTTable, SFNTTable};
pub mod index;

#[derive(Clone, Copy)]
pub struct Fixed(i32);
impl Fixed {
    const INT_ONE: i32 = 1 << 16;
}
impl SFNTPrimitive<4> for Fixed {
    fn convert_from(buf: &[u8; 4]) -> Self {
        Fixed(i32::from_be_bytes(*buf))
    }
}
impl Debug for Fixed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&f64::from(*self), f)
    }
}
impl Display for Fixed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&f64::from(*self), f)
    }
}
impl From<Fixed> for f64 {
    fn from(val: Fixed) -> Self {
        (val.0 as f64) / (Fixed::INT_ONE as f64)
    }
}
pub type IFWord = i16;
pub type UFWord = u16;

pub trait SFNTReadable: Sized {
    fn read_from<R: Read + ?Sized + Seek>(read: &mut R) -> Result<Self, Box<dyn Error>>;
}
impl<T: SFNTReadable + Copy, const N: usize> SFNTReadable for [T; N] {
    fn read_from<R: Read + ?Sized + Seek>(read: &mut R) -> Result<Self, Box<dyn Error>> {
        let mut a = MaybeUninit::<[T; N]>::uninit().transpose();
        for item in a.iter_mut() {
            item.write(T::read_from(read)?);
        }
        Ok(unsafe { MaybeUninit::array_assume_init(a) })
    }
}
// impl<const ELEM_LEN: usize, T: SFNTPrimitive<ELEM_LEN>, const ARRAY_LENGTH: usize>
//     SFNTPrimitive<{ ELEM_LEN * ARRAY_LENGTH }> for [T; ARRAY_LENGTH]
// {
// }

/// SFNT Readable allows for easy reading from an sfnt stream
pub trait SFNTPrimitive<const BYTE_COUNT: usize>: Sized
where
    [(); BYTE_COUNT]:,
{
    fn convert_from(buf: &[u8; BYTE_COUNT]) -> Self;
    #[inline]
    fn convert_from_slice(buf: &[u8]) -> Self {
        Self::convert_from(buf.try_into().unwrap())
    }
    fn read_from<R: Read + ?Sized>(read: &mut R) -> Result<Self, Box<dyn Error>> {
        let mut buf = [0u8; BYTE_COUNT];
        read.read_exact(&mut buf)?;
        Ok(Self::convert_from(&buf))
    }
    fn read_into_array<R: Read + ?Sized>(
        read: &mut R,
        slice: &mut [Self],
    ) -> Result<(), Box<dyn Error>> {
        let mut buf = [0u8; BYTE_COUNT];
        for s in slice.iter_mut() {
            read.read_exact(&mut buf)?;
            *s = Self::convert_from(&buf);
        }
        Ok(())
    }
    fn read_into_vec<R: Read + ?Sized>(
        read: &mut R,
        len: usize,
    ) -> Result<Vec<Self>, Box<dyn Error>> {
        let mut result = Vec::with_capacity(len);

        while result.len() < len {
            let mut buf = vec![0u8; BYTE_COUNT * (len - result.len()).min(32)];
            read.read_exact(&mut buf)?;
            for b in buf.chunks_exact(BYTE_COUNT) {
                result.push(Self::convert_from_slice(b));
            }
        }
        Ok(result)
    }
    fn read_array<R: Read + ?Sized, const ARRAY_LENGTH: usize>(
        read: &mut R,
    ) -> Result<[Self; ARRAY_LENGTH], Box<dyn Error>>
    where
        Self: Default + Copy,
    {
        let mut buffer = [Self::default(); ARRAY_LENGTH];
        Self::read_into_array(read, &mut buffer)?;
        Ok(buffer)
    }
}

pub trait SFNTStream: Read + Seek {
    fn get_prim<const BYTE_COUNT: usize, T: SFNTPrimitive<BYTE_COUNT>>(
        &mut self,
    ) -> Result<T, Box<dyn Error>> {
        T::read_from(self)
    }
    fn get_prim_array<const ARRAY_LENGTH: usize, const BYTE_COUNT: usize, T>(
        &mut self,
    ) -> Result<[T; ARRAY_LENGTH], Box<dyn Error>>
    where
        T: Copy + Default,
        T: SFNTPrimitive<BYTE_COUNT>,
    {
        T::read_array(self)
    }
    fn get<T: SFNTReadable>(&mut self) -> Result<T, Box<dyn Error>> {
        T::read_from(self)
    }
}
impl<T: Read + Seek> SFNTStream for T {}

macro_rules! prim {
    ($tp:ty, $bc:expr) => {
        #[allow(unused_braces)]
        impl SFNTPrimitive<$bc> for $tp {
            fn convert_from(buf: &[u8; $bc]) -> Self {
                <$tp>::from_be_bytes(*buf)
            }
        }
    };
}
prim!(u8, 1);
prim!(i8, 1);
prim!(u16, 2);
prim!(i16, 2);
prim!(u32, 4);
prim!(i32, 4);
prim!(u64, 8);
prim!(i64, 8);
prim!(usize, { std::mem::size_of::<usize>() });
prim!(isize, { std::mem::size_of::<isize>() });

pub struct SFNTFile {
    pub header: SFNTHeader,
    pub file: File,
    pub table_cache: BTreeMap<[u8; 4], Box<dyn SFNTTable>>,
}
impl TryFrom<File> for SFNTFile {
    type Error = Box<dyn Error>;
    fn try_from(mut file: File) -> Result<Self, Self::Error> {
        let header = file.get()?;
        Ok(Self {
            header,
            file,
            table_cache: BTreeMap::default(),
        })
    }
}
impl SFNTFile {
    pub(crate) fn seek_table(
        &mut self,
        tags: &[&TableTagInner],
    ) -> Result<TableReference, Box<dyn Error>> {
        for a in tags {
            if let Some(b) = self.header.tables.get(a) {
                self.file.seek(std::io::SeekFrom::Start(b.offset as u64))?;
                return Ok(b);
            }
        }
        let tags = tags
            .iter()
            .map(|a| str::from_utf8(*a).unwrap().to_string())
            .reduce(|a, b| format!("{a}, {b}"))
            .unwrap();
        Err(SFNTError::SomethingIsNotFound(
            "Table",
            format!("[{:}]", tags),
        ))?
    }
    #[inline(always)]
    pub fn get_table<T: DependentSFNTTable>(&mut self) -> Result<&T, Box<dyn Error>> {
        let tag = *T::TAGS.first().unwrap();
        let contains = self.table_cache.contains_key(tag);
        if !contains {
            let table = T::read(self)?;
            self.table_cache
                .insert(*tag, Box::new(table) as Box<dyn SFNTTable>);
        }

        let v = self.table_cache.get(tag).unwrap().as_ref() as &dyn Any;
        Ok(v.downcast_ref::<T>().unwrap())
    }

    #[inline(always)]
    fn get_table_reference(&self, tag: &[u8; 4]) -> Option<TableReference> {
        self.header.tables.get(tag)
    }

    fn get_table_reference_error(&self, tag: &[u8; 4]) -> Result<TableReference, SFNTError> {
        self.header.tables.get(tag).ok_or_else(|| {
            SFNTError::SomethingIsNotFound("Table", str::from_utf8(tag).unwrap().to_string())
        })
    }
    fn get_table_reference_any_error(
        &self,
        iterator: &[&TableTagInner],
    ) -> Result<TableReference, SFNTError> {
        for item in iterator {
            if let Some(a) = self.header.tables.get(item) {
                return Ok(a);
            }
        }
        Err(SFNTError::SomethingIsNotFound(
            "Table",
            format!(
                "[{}]",
                iterator
                    .iter()
                    .map(|a| str::from_utf8(*a).unwrap().to_string())
                    .reduce(|a, b| format!("{a}, {b}"))
                    .unwrap_or("".to_string()),
            ),
        ))
    }
}
pub mod tables;
#[macro_export]
macro_rules! sfnt_table {
    ($name:ident) => {
        impl super::SFNTTable for $name {
            fn into(self) -> super::AnySFNTTable {
                super::AnySFNTTable::$name(Box::new(self))
            }
        }
    };
    ($name:ident,$tp:ty) => {
        impl super::SFNTTable for $tp {
            fn into(self) -> super::AnySFNTTable {
                super::AnySFNTTable::$name(Box::new(self))
            }
        }
    };
}
#[cfg(test)]
#[cfg(feature = "os_fonts")]
#[allow(unused)]
pub mod test {
    use std::{
        collections::{BTreeMap, BTreeSet},
        error::Error,
        fs::{File, OpenOptions},
        io::Seek,
        time::Instant,
    };

    use super::SFNTFile;
    use super::tables::name::NameTable;
    use crate::{
        config::Config,
        providers::{FontProvider, FontReference, os::OSProvider},
        types::sfnt_types::{
            index::{SFNTError, TableTag},
            tables::{
                DependentSFNTTable, cmap::CMAPTable, hmtx::HorizontalMetrics,
                maxp::MemoryManagementTable, post::PostTable,
            },
        },
    };
    fn get_fonts()
    -> Result<Vec<crate::providers::filebasedprovider::FileFontReference>, Box<dyn Error>> {
        let mut provider = OSProvider::new(&Config::default())?;
        provider.find_fonts(&())
    }
    #[test]
    fn table_count() -> Result<(), Box<dyn Error>> {
        let mut map = BTreeMap::new();
        let mut c = 0;
        for a in get_fonts()? {
            let name = a.get_font(&())?;
            let file = OpenOptions::new().read(true).open(&name)?;
            let mut sfntfile: SFNTFile = file.try_into()?;
            for k in sfntfile.header.tables.keys() {
                if let Some(m) = map.get_mut(k) {
                    *m += 1;
                } else {
                    map.insert(*k, 1);
                }
            }
            c += 1;
        }
        dbg!(map.get(&TableTag(*b"head")));
        let mut reverse_map: BTreeMap<i32, BTreeSet<TableTag>> = BTreeMap::new();
        for (key, value) in map {
            if let std::collections::btree_map::Entry::Vacant(e) = reverse_map.entry(value) {
                e.insert(BTreeSet::new());
                reverse_map.get_mut(&value).unwrap()
            } else {
                reverse_map.get_mut(&value).unwrap()
            }
            .insert(key);
        }
        eprintln!(" Tag    | Count");
        for (value, keys) in reverse_map.into_iter().rev() {
            for key in keys {
                eprintln!(" {key:?} | {value:3} ")
            }
        }

        eprintln!(" Total  | {c:3}");
        Ok(())
    }
    #[allow(clippy::all)]
    #[test]
    fn single_test() -> Result<(), Box<dyn Error>> {
        let name = "/System/Library/Fonts/Supplemental/NISC18030.ttf";
        let file = OpenOptions::new().read(true).open(name)?;
        eprintln!("{name:?}");
        // let start = Instant::now();
        let mut sfntfile: SFNTFile = file.try_into()?;
        let table: &HorizontalMetrics = match sfntfile.get_table() {
            Ok(a) => a,
            Err(e) => return Err(e),
        };
        Ok(())
    }
    #[test]
    fn alltest() -> Result<(), Box<dyn Error>> {
        let mut provider = OSProvider::new(&Config::default())?;
        // let mut set = BTreeSet::new();
        for a in provider.find_fonts(&())? {
            let name = a.get_font(&())?;
            let file = OpenOptions::new().read(true).open(&name)?;
            eprintln!("{name:?}");
            // let start = Instant::now();
            let mut sfntfile: SFNTFile = file.try_into()?;
            let table: &MemoryManagementTable = match sfntfile.get_table() {
                Ok(a) => a,
                Err(e) => {
                    return Err(if e.is::<SFNTError>() {
                        let mut error: SFNTError = *e.downcast().unwrap();
                        if let SFNTError::SomethingIsNotFound("Table", ref s) = error {
                            if &MemoryManagementTable::tags_to_string() == s {
                                continue;
                            }
                        }
                        dbg!(sfntfile.header);
                        Box::new(error) as Box<dyn Error>
                    } else {
                        e
                    });
                }
            };
        }
        // set.remove(&TableTag(*b"NAME"));
        // println!("{set:?}");
        Ok(())
    }
}
