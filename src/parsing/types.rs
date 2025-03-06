use std::{fs::File, io::Read};

pub struct Input {
    pub source: Box<dyn Read>,
    pub name: String,
    pub pos: usize,
    pub line: usize,
    pub col: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum InputResult<E> {
    Char(char),
    Eof,
    Error(E),
}

impl Input {
    pub fn new_from_source(name: &str, source: Box<dyn Read>) -> Self {
        Self {
            source,
            name: name.to_string(),
            pos: 0,
            line: 1,
            col: 1,
        }
    }
    pub fn new_from_file(name: &str, file: File) -> Self {
        Self {
            source: Box::new(file),
            name: name.to_string(),
            pos: 0,
            line: 1,
            col: 1,
        }
    }
    pub fn new_from_stdin() -> Self {
        Self {
            source: Box::new(std::io::stdin()),
            name: "<stdin>".to_string(),
            pos: 0,
            line: 1,
            col: 1,
        }
    }

    fn int_read_char(&mut self) -> InputResult<std::io::Error> {
        let mut buf = [0u8; 4];
        match self.source.read(&mut buf[..1]) {
            Err(e) => return InputResult::Error(e),
            Ok(0) => return InputResult::Eof,

            Ok(_) => {}
        }
        if buf[0] & 0b1000_0000 == 0 {
            return InputResult::Char(buf[0] as char);
        }
        let mut n = None;
        for i in 1..=4usize {
            if (buf[0] & (0b1000_0000u8 >> (i))) == 0 {
                n = Some(i);
                break;
            }
        }
        if n.is_none() {
            return InputResult::Error(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid UTF-8",
            ));
        }

        let n = n.unwrap();
        // dbg!(n);
        match self.source.read(&mut buf[1..n]) {
            Ok(_) => {
                return match std::str::from_utf8(&buf[..n]) {
                    Ok(s) => InputResult::Char(s.chars().next().unwrap()),
                    Err(e) => InputResult::Error(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        e.to_string(),
                    )),
                }
            }
            _ => {
                return InputResult::Error(std::io::Error::new(
                    std::io::ErrorKind::UnexpectedEof,
                    "",
                ))
            }
        }
    }

    pub fn read_char(&mut self) -> InputResult<std::io::Error> {
        let c = self.int_read_char();
        self.pos += 1;
        // let oc = (c, self.line, self.col);
        if let InputResult::Char(c) = c {
            if c == '\n' {
                self.line += 1;
                self.col = 1;
            } else {
                self.col += 1;
            }
        }
        c
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CharacterType {
    Escape = 0,
    BeginGroup,
    EndGroup,
    MathShift,
    AlignmentTab,
    EndOfLine,
    Parameter,
    Superscript,
    Subscript,
    Ignored,
    Space,
    Letter,
    Other,
    Active,
    Comment,
    Invalid,
}
pub struct TexCharacterMap {
    parent: Option<Box<TexCharacterMap>>,
    map: std::collections::HashMap<char, CharacterType>,
}

impl TexCharacterMap {
    pub fn new() -> Self {
        let mut m = Self {
            parent: None,
            map: std::collections::HashMap::new(),
        };
        m.map.insert('\\', CharacterType::Escape);
        m.map.insert('{', CharacterType::BeginGroup);
        m.map.insert('}', CharacterType::EndGroup);
        m.map.insert('$', CharacterType::MathShift);
        m.map.insert('&', CharacterType::AlignmentTab);
        m.map.insert('\n', CharacterType::EndOfLine);
        m.map.insert('#', CharacterType::Parameter);
        m.map.insert('^', CharacterType::Superscript);
        m.map.insert('_', CharacterType::Subscript);
        m.map.insert(' ', CharacterType::Space);
        m.map.insert('\t', CharacterType::Space);
        m.map.insert('\r', CharacterType::Space);
        for c in 'a'..='z' {
            m.map.insert(c, CharacterType::Letter);
            m.map.insert(c.to_ascii_uppercase(), CharacterType::Letter);
        }
        m.map.insert('%', CharacterType::Comment);
        m.map.insert(0 as char, CharacterType::Ignored);
        m.map.insert(0x7f as char, CharacterType::Invalid);

        m
    }

    pub fn new_with_parent(parent: TexCharacterMap) -> Self {
        Self {
            parent: Some(Box::new(parent)),
            map: std::collections::HashMap::new(),
        }
    }
    pub fn get(&self, c: char) -> Option<CharacterType> {
        match self.map.get(&c) {
            Some(t) => Some(*t),
            None => match &self.parent {
                Some(p) => p.get(c),
                None => None,
            },
        }
    }
    pub fn set(&mut self, c: char, t: CharacterType) {
        self.map.insert(c, t);
    }
    pub fn get_or(&self, c: char, default: CharacterType) -> CharacterType {
        match self.map.get(&c) {
            Some(t) => *t,
            None => match &self.parent {
                Some(p) => p.get_or(c, default),
                None => default,
            },
        }
    }
}
#[derive(Debug, Clone)]
pub enum Token {
    Word(String),
    SingleCharacter(char),
    ControlSequence(String),
    BeginGroup,
    EndGroup,
    MathShift,
    AlignmentTab,
    EndOfLine,
    Superscript,
    Subscript,
    EndOfFile,
}

pub struct Parser {
    pub input: Vec<Input>,
    pub map: TexCharacterMap,
}
impl Parser {
    pub fn new() -> Self {
        Self {
            input: vec![],
            map: TexCharacterMap::new(),
        }
    }
}

impl Parser {
    pub fn parse_token(&mut self) -> Token {
        let c = self.input[0].read_char();
        loop {
            match c {
                InputResult::Char(_) => {
                    break;
                }
                InputResult::Eof => return Token::EndOfFile,
                InputResult::Error(e) => {
                    panic!("Error: {:?}", e);
                }
            }
        }
        todo!()
    }
}
