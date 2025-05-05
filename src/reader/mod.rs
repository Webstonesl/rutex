use core::str;
use std::{
    collections::VecDeque,
    fmt::{Debug, Display},
    io::{self, stdin, BufRead as _, BufReader, Read},
    iter::Peekable,
    path::PathBuf,
};

use crate::error::{Error, ErrorKind};

pub struct ReaderLine {
    line_nr: usize,
    col_nr: usize,
    chars: VecDeque<char>,
}
impl ReaderLine {
    pub fn new(line_nr: usize, line: &str) -> ReaderLine {
        ReaderLine {
            line_nr,
            col_nr: 0,
            chars: line.chars().collect(),
        }
    }
}
impl Iterator for ReaderLine {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        match self.chars.pop_front() {
            a @ Some(c) if c != '\r' && c != '\n' => {
                self.col_nr += 1;
                a
            }
            a => a,
        }
    }
}
pub struct Reader {
    reader: BufReader<Box<dyn std::io::Read>>,
    name: String,
    line: Option<ReaderLine>,
    eof: bool,
}
impl Reader {
    pub fn new<T: ToString, R: std::io::Read + 'static>(name: T, reader: R) -> Self {
        Reader {
            reader: BufReader::new(Box::new(reader)),
            name: name.to_string(),
            line: None,
            eof: false,
        }
    }
    pub fn std_in() -> Self {
        Self::new("stdin", stdin())
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    fn read_line(&mut self) -> Result<(), Error> {
        let mut line = String::new();
        if self.reader.read_line(&mut line)? == 0 {
            return Err(Error::new_with_message(
                ErrorKind::EoFError,
                "End of File Reached",
            ));
        }

        let nr = if let Some(ReaderLine { line_nr, .. }) = self.line {
            line_nr + 1
        } else {
            1
        };
        self.line = Some(ReaderLine::new(nr, &line));

        Ok(())
    }
    fn line_is_empty(&self) -> bool {
        match self.line {
            Some(ReaderLine { ref chars, .. }) => chars.is_empty(),
            None => true,
        }
    }
    fn inner_iter(&mut self) -> Option<Result<char, Error>> {
        if self.line_is_empty() {
            match self.read_line() {
                Err(Error {
                    kind: ErrorKind::EoFError,
                    ..
                }) => {
                    self.eof = true;
                    return None;
                }
                Err(e) => {
                    return Some(Err(e));
                }
                Ok(()) => {}
            }
            if self.line_is_empty() {
                self.eof = true;
                return None;
            }
        }
        match self.line {
            Some(ref mut lr) => lr.next().map(Ok),
            None => None,
        }
    }
}
impl TryFrom<PathBuf> for Reader {
    type Error = std::io::Error;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        let name = value.to_str().unwrap().to_string();
        std::fs::OpenOptions::new()
            .read(true)
            .open(value)
            .map(move |reader| Reader::new(name, reader))
    }
}
impl Iterator for Reader {
    type Item = Result<char, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.eof {
            return None;
        }
        match self.inner_iter() {
            Some(Err(Error {
                kind: ErrorKind::EoFError,
                ..
            })) => {
                self.eof = true;
                None
            }
            a => a,
        }
    }
}

pub struct SourceReader {
    source: Peekable<Reader>,
    name: String,
    position: usize,
    line: usize,
    column: usize,
}
impl SourceReader {
    pub fn new(source: Reader) -> Self {
        SourceReader {
            name: source.name().to_string(),
            source: Reader::peekable(source),
            position: 0,
            line: 1,
            column: 0,
        }
    }

    pub fn consume(&mut self) -> Option<Result<char, Error>> {
        match self.source.next() {
            Some(Ok(a)) => {
                if a == '\n' {
                    self.line += 1;
                    self.column = 0
                } else {
                    self.column += 1;
                };
                Some(Ok(a))
            }
            Some(Err(e)) => Some(Err(e)),
            None => None,
        }
    }
    pub fn lookahead(&mut self) -> Option<&Result<char, Error>> {
        self.source.peek()
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
struct ByteReader(Vec<u8>, usize);
impl Read for ByteReader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let otherlen = buf.len();
        let selflen = self.0.len() - self.1;
        let len = selflen.min(otherlen);
        buf[..len].copy_from_slice(&self.0[self.1..(self.1 + len)]);
        self.1 += len;
        Ok(len)
    }
}
#[test]
#[allow(clippy::never_loop)]
fn test() -> Result<(), Error> {
    let a = vec![b'a', b'c', b'd', b'e', b'\n', b'a'];
    let v = Reader::new("byte-stream", ByteReader(a, 0));
    for a in v {
        dbg!(a)?;
    }
    Ok(())
}
