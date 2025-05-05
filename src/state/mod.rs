use std::{collections::HashMap, fs::File, mem};

use crate::{
    error::{Error, ErrorKind},
    macros::MacroValue,
    parser::{CharacterCategory, Token},
    reader::{Reader, SourceReader},
    writer::Writer,
    Config,
};
pub mod groupstate;
use groupstate::GroupStates;
/// Main Object of the Tex Implementation. Created from a `config`, and run by
/// running `get_group`. The author is aware of non
#[allow(warnings)]
pub struct State {
    pub(crate) sources: Vec<SourceReader>,
    pub(crate) result: Writer,
    pub(crate) input_streams: HashMap<u8, Reader>,
    pub(crate) input_text_streams: HashMap<u8, SourceReader>,
    pub(crate) output_streams: HashMap<u8, Writer>,
    pub(crate) group_states: GroupStates,
    pub(crate) group_tokens: Vec<Token>,
    pub(crate) back_log: Vec<Token>,
    pub(crate) expand: bool,
}

unsafe impl Sync for State {}

impl TryFrom<Config> for State {
    type Error = Error;

    fn try_from(value: Config) -> Result<Self, Self::Error> {
        let name = value
            .source
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        let log_path = value
            .working_directory
            .with_file_name(name.replace(".tex", ".log"));

        let result_name = name.replace(".tex", ".pdf");
        let result_path = value.working_directory.with_file_name(&result_name);
        let mut output_streams = HashMap::new();
        output_streams.insert(
            0,
            Writer::Dual {
                name: "log".to_string(),
                writer1: Box::new(Writer::new_file(&log_path, false)?),
                writer2: Box::new(Writer::StdErr),
            },
        );
        let mut input_text_streams = HashMap::new();
        input_text_streams.insert(0, SourceReader::new(Reader::std_in()));

        Ok(State {
            sources: vec![SourceReader::new(Reader::try_from(value.source)?)],
            result: Writer::File {
                name: result_path.clone(),
                writer: File::create(result_path)?,
            },
            input_streams: HashMap::new(),
            input_text_streams,
            output_streams,
            group_states: GroupStates::new(),
            group_tokens: Vec::new(),
            back_log: Vec::new(),
            expand: true,
        })
    }
}

impl State {
    fn character_consume(&mut self) -> Result<(char, CharacterCategory), Error> {
        if let Some(source) = self.sources.last_mut() {
            match source.consume() {
                Some(Ok(c)) => return Ok((c, self.group_states.get_category(c))),
                Some(Err(e)) => return Err(e),
                None => {
                    self.sources.pop();
                    return self.character_consume();
                }
            }
        }
        Err(Error::new(crate::error::ErrorKind::EoFError))
    }
    fn character_lookahead(&mut self) -> Option<Result<(char, CharacterCategory), Error>> {
        if let Some(source) = self.sources.last_mut() {
            return match source.lookahead() {
                Some(Ok(c)) => Some(Ok((*c, self.group_states.get_category(*c)))),
                Some(Err(e)) => Some(Err(e.clone())),
                None => None,
            };
        } else {
            None
        }
    }
    /// Adds a group state so that local definitions can happen.
    pub fn push_group(&mut self) {
        self.group_states.push();
    }
    /// Removes a group from the stack and pops the group tokens.
    pub fn pop_group(&mut self) -> Result<(), Error> {
        self.group_states.pop()?;
        let mut gt = Vec::new();
        mem::swap(&mut gt, &mut self.group_tokens);
        Err(Error::new(ErrorKind::PopGroup(Token::Group(gt))))
    }
    /// Reads a control sequence from the input stream.
    pub fn read_control_sequence(&mut self, c: char) -> Result<Token, Error> {
        let mut s = String::new();
        s.push(c);
        let (c, cat) = self.character_consume()?;
        s.push(c);
        if cat == CharacterCategory::Letter {
            while let Some(Ok((c, CharacterCategory::Letter))) = self.character_lookahead() {
                s.push(c);
                self.character_consume()?;
            }
        }

        while let Some(Ok((_, CharacterCategory::Space))) = self.character_lookahead() {
            let _ = self.character_consume();
        }

        return Ok(Token::ControlSequence(s));
    }
    /// parses a single token and returns it
    pub fn get_token(&mut self) -> Result<Token, Error> {
        let (chr, cat) = match self.back_log.pop() {
            Some(Token::Character(chr, cat)) => (chr, cat),
            Some(t) => return Ok(t),
            None => self.character_consume()?,
        };
        match cat {
            CharacterCategory::Escape => return self.read_control_sequence(chr),
            CharacterCategory::BeginGroup => {
                return self.get_group(Token::Character(chr, cat));
            }
            CharacterCategory::EndGroup => {
                self.group_tokens.push(Token::Character(chr, cat));
                self.pop_group()?;
                unreachable!();
            }
            CharacterCategory::Parameter => return self.get_parameter(),
            CharacterCategory::Ignored => return self.get_token(),
            CharacterCategory::Space => {
                while let Some(Ok((_, CharacterCategory::Space))) = self.character_lookahead() {
                    let _ = self.character_consume();
                }
                return Ok(Token::Character(chr, CharacterCategory::Space));
            }
            CharacterCategory::Active => return Ok(Token::ControlSequence(chr.to_string())),
            CharacterCategory::Comment => {
                self.skip_comment()?;
                return self.get_token();
            }
            CharacterCategory::Invalid => {
                return Err(Error::new_with_message(
                    ErrorKind::InvalidCharacter,
                    format!("{:?} is invalid", chr),
                ))
            }
            a => return Ok(Token::Character(chr, a)),
        }
    }
    /// gets a group of tokens from the sources.
    pub fn get_group(&mut self, token: Token) -> Result<Token, Error> {
        self.push_group();
        self.group_tokens.push(token);
        loop {
            match self.get_token() {
                Ok(Token::ControlSequence(s)) if self.expand => {
                    match self.group_states.get_macro(&s) {
                        Ok(v) => v.run(self)?,
                        Err(e) => return Err(e),
                    }
                }
                Ok(t) => self.group_tokens.push(t),
                Err(Error {
                    kind: ErrorKind::PopGroup(t),
                    ..
                }) => {
                    return Ok(t);
                }
                Err(Error {
                    kind: ErrorKind::NoToken,
                    ..
                }) => continue,
                Err(e) => return Err(e),
            }
        }
    }

    fn skip_comment(&mut self) -> Result<(), Error> {
        loop {
            match self.character_consume()?.1 {
                CharacterCategory::EndOfLine => break,
                _ => continue,
            }
        }
        Ok(())
    }
    fn get_parameter(&mut self) -> Result<Token, Error> {
        let mut s = String::new();
        while let Some(Ok((c @ '0'..='9', _))) = self.character_lookahead() {
            s.push(c);
            self.character_consume()?;
        }
        let u = s.parse()?;
        Ok(Token::Parameter(u))
    }
    /// push tokens onto a stack which will be read before new characters gets
    /// read.
    pub fn push_tokens(&mut self, tokens: &[Token]) {
        for token in tokens.iter().rev() {
            self.back_log.push(token.clone())
        }
    }
    pub fn define_macro(&mut self, s: &str, v: MacroValue) {
        self.group_states.set_macro(s, v);
    }
    pub fn set_expand(&mut self, expand: bool) {
        self.expand = expand;
    }
    pub fn read_element(&mut self) -> Result<(), Error> {
        match self.get_token() {
            Ok(Token::ControlSequence(s)) if self.expand => match self.group_states.get_macro(&s) {
                Ok(v) => v.run(self)?,
                Err(e) => return Err(e),
            },
            Ok(t) => self.group_tokens.push(t),
            Err(e) => return Err(e),
        };
        Ok(())
    }
    pub fn read_file(&mut self) -> Result<Vec<Token>, Error> {
        self.push_group();

        let r = loop {
            match self.read_element() {
                Ok(()) => continue,
                Err(Error {
                    kind: ErrorKind::EoFError,
                    ..
                }) => {
                    let gt = self.group_tokens.clone();
                    break Ok(gt);
                }
                Err(Error {
                    kind: ErrorKind::NoToken,
                    ..
                }) => continue,
                Err(e) => break Err(e),
            }
        }?;
        match self.pop_group() {
            Ok(_) => {}
            Err(Error {
                kind: ErrorKind::PopGroup(_),
                ..
            }) => (),
            Err(e) => return Err(e),
        }
        Ok(r)
    }

    pub(crate) fn set_global(&mut self, arg: bool) {
        self.group_states.set_global(arg);
    }
}
