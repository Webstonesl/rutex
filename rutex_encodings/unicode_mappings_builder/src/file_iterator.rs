use std::{collections::VecDeque, path::PathBuf};

pub struct FileIterator {
    root: PathBuf,
    iterators: Option<Box<FileIterator>>,
    items: VecDeque<(PathBuf, bool)>,
}
impl FileIterator {
    pub fn new(value: PathBuf) -> Result<Self, std::io::Error> {
        if value.is_file() {
            return Ok(FileIterator {
                root: value.clone(),
                iterators: None,
                items: VecDeque::from([(value, false)]),
            });
        }

        if value.is_dir() {
            let mut items = VecDeque::new();
            for f in value.read_dir()? {
                let f = f?;
                items.push_front((f.path(), f.file_type()?.is_dir()));
            }
            return Ok(FileIterator {
                root: value,
                iterators: None,
                items,
            });
        }
        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "File must be a directory or file",
        ))
    }
}
impl Iterator for FileIterator {
    type Item = PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut a) = self.iterators {
            if let Some(b) = a.next() {
                return Some(b);
            } else {
                self.iterators = None;
            }
        }
        let (path_buf, is_dir) = self.items.pop_back()?;
        if is_dir {
            self.iterators = Some(Box::new(FileIterator::new(path_buf.clone()).unwrap()));
        }

        Some(path_buf)
    }
}
#[test]
fn test() {
    for a in FileIterator::new(
        "/Users/webstones/Code/rutex/rutex_encodings/unicode_mappings_builder/MAPPINGS".into(),
    )
    .unwrap()
    .filter(|a| {
        a.is_file()
            && a.extension()
                .filter(|a| a.to_str().map(str::to_lowercase) == Some("txt".to_string()))
                .is_some()
    }) {
        eprintln!("{:?}", a);
    }
}
