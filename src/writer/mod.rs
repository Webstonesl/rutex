use std::{
    fmt::Debug,
    fs::{File, OpenOptions},
    io::Write,
    path::PathBuf,
};

pub trait WriterTrait: Write {
    fn get_name(&self) -> String;
}
#[derive(Debug)]
pub struct FileWriter {
    path: PathBuf,
    file: File,
}
impl FileWriter {
    pub fn new(result_path: PathBuf, append: bool) -> Result<Self, std::io::Error> {
        Ok(Self {
            path: result_path.clone(),
            file: OpenOptions::new()
                .append(append)
                .create(true)
                .write(true)
                .open(result_path)?,
        })
    }
}
impl Write for FileWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.file.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.file.flush()
    }
}
impl WriterTrait for FileWriter {
    fn get_name(&self) -> String {
        self.path.file_name().unwrap().to_str().unwrap().to_string()
    }
}
pub struct NWriter<const N: usize> {
    pub name: String,
    pub children: [Box<dyn Write>; N],
}
impl NWriter<2> {
    pub fn new<R1: Write + 'static, R2: Write + 'static>(name: String, r1: R1, r2: R2) -> Self {
        Self {
            name,
            children: [Box::new(r1), Box::new(r2)],
        }
    }
}
impl<const N: usize> Write for NWriter<N> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        for c in self.children.iter_mut() {
            c.write_all(buf)?
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        for c in self.children.iter_mut() {
            c.flush()?
        }
        Ok(())
    }
}
impl<const N: usize> WriterTrait for NWriter<N> {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

pub struct StreamWriter {
    name: String,
    writer: Box<dyn Write>,
}
impl Write for StreamWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.writer.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}
impl WriterTrait for StreamWriter {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}
pub struct Writer(Box<dyn WriterTrait>);
impl Writer {
    pub fn new<R: WriterTrait + 'static>(result_path: R) -> Self {
        Self(Box::new(result_path))
    }
}
impl WriterTrait for Writer {
    fn get_name(&self) -> String {
        self.0.get_name()
    }
}
impl Write for Writer {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.0.flush()
    }
}
// pub enum Writer {
//     File {
//         name: PathBuf,
//         writer: std::fs::File,
//     },
//     StdOut,
//     StdErr,
//     Stream {
//         name: String,
//         writer: Box<dyn Write>,
//     },
//     Dual {
//         name: String,
//         writer1: Box<Writer>,
//         writer2: Box<Writer>,
//     },
// }
