use std::{
    fmt::{Debug, Display},
    path::PathBuf,
};

use utf8streamreader::utf::{Lookahead, Utf8Reader};

use crate::error::Error;
pub enum Reader {
    StdIn,
    File {
        name: PathBuf,
        file: std::fs::File,
    },
    Stream {
        name: String,
        read: Box<dyn std::io::Read>,
    },
}
impl Reader {
    pub fn name(&self) -> String {
        match self {
            Reader::StdIn => "stdin".to_string(),
            Reader::File { name, .. } => format!("{}", name.display()),
            Reader::Stream { name, .. } => name.clone(),
        }
    }
}
impl std::io::Read for Reader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            Reader::StdIn => std::io::stdin().read(buf),
            Reader::File { file, .. } => file.read(buf),
            Reader::Stream { read, .. } => read.read(buf),
        }
    }
}
pub struct SourceReader {
    source: Lookahead<Result<char, utf8streamreader::errors::Error>, Utf8Reader<Reader>>,
    name: String,
    position: usize,
    line: usize,
    column: usize,
}
impl SourceReader {
    pub fn new(source: Reader) -> Self {
        SourceReader {
            name: source.name(),
            source: Lookahead::new(Utf8Reader::new(Box::new(source))),
            position: 0,
            line: 1,
            column: 0,
        }
    }

    pub fn consume(&mut self) -> Option<Result<char, Error>> {
        match self.source.consume() {
            Some(Ok(a)) => {
                if a == '\n' {
                    self.line += 1;
                    self.column = 0
                } else {
                    self.column += 1;
                };
                Some(Ok(a))
            }
            Some(Err(e)) => Some(Err(e.into())),
            None => None,
        }
    }
    pub fn lookahead(&mut self, u: usize) -> Option<Result<char, Error>> {
        match self.source.lookahead(u) {
            Some(Ok(a)) => Some(Ok(*a)),
            Some(Err(e)) => Some(Err(e.into())),
            None => None,
        }
    }
}
impl Display for SourceReader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}:{}:{}", self.name, self.line, self.column))
    }
}
impl Debug for SourceReader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SourceReader")
            .field("source", &self.name)
            .field("position", &self.position)
            .field("line", &self.line)
            .field("column", &self.column)
            .finish()
    }
}
