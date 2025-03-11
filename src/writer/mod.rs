use std::{fmt::Debug, fs::OpenOptions, io::Write, path::PathBuf};

pub enum Writer {
    File {
        name: PathBuf,
        writer: std::fs::File,
    },
    StdOut,
    StdErr,
    Stream {
        name: String,
        writer: Box<dyn Write>,
    },
    Dual {
        name: String,
        writer1: Box<Writer>,
        writer2: Box<Writer>,
    },
}
impl Writer {
    pub fn new_file(p: &PathBuf, append: bool) -> Result<Self, std::io::Error> {
        Ok(Self::File {
            name: p.clone(),
            writer: OpenOptions::new()
                .create(true)
                .write(true)
                .append(append)
                .open(p)?,
        })
    }
    pub fn name(&self) -> String {
        match self {
            Writer::File { name, .. } => format!("{}", name.file_name().unwrap().to_str().unwrap()),
            Writer::StdOut => "stdout".to_string(),
            Writer::StdErr => "stderr".to_string(),
            Writer::Stream { name, .. } => name.clone(),
            Writer::Dual { name, .. } => name.clone(),
        }
    }
}
impl Write for Writer {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            Writer::File { writer, .. } => writer.write(buf),
            Writer::StdOut => std::io::stdout().write(buf),
            Writer::StdErr => std::io::stderr().write(buf),
            Writer::Stream { writer, .. } => writer.write(buf),
            Writer::Dual {
                writer1, writer2, ..
            } => {
                writer1.write(buf)?;
                writer2.write(buf)
            }
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            Writer::File { writer, .. } => writer.flush(),
            Writer::StdOut => std::io::stdout().flush(),
            Writer::StdErr => std::io::stderr().flush(),
            Writer::Stream { writer, .. } => writer.flush(),
            Writer::Dual {
                writer1, writer2, ..
            } => {
                writer1.flush()?;
                writer2.flush()
            }
        }
    }
}
impl Debug for Writer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Writer({})", self.name())
    }
}
