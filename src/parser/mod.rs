use std::{collections::HashMap, fmt::Debug};

use crate::error::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CharacterCategory {
    Escape,
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

#[derive(Debug, Clone)]
pub struct CharacterMap(HashMap<char, CharacterCategory>);

impl CharacterMap {
    pub fn new() -> Self {
        let map = HashMap::new();
        Self(map)
    }
    pub fn new_and_init() -> Self {
        let mut s = Self::new();
        s.init();
        s
    }
    pub fn init(&mut self) {
        for i in 0o000..0o200 {
            let chr = unsafe { char::from_u32_unchecked(i) };
            self.0.insert(
                chr,
                match chr {
                    '\\' => CharacterCategory::Escape,
                    '{' => CharacterCategory::BeginGroup,
                    '}' => CharacterCategory::EndGroup,
                    '$' => CharacterCategory::MathShift,
                    '&' => CharacterCategory::AlignmentTab,
                    '\n' => CharacterCategory::EndOfLine,
                    '#' => CharacterCategory::Parameter,
                    '^' => CharacterCategory::Superscript,
                    '_' => CharacterCategory::Subscript,
                    '\0' => CharacterCategory::Ignored,
                    ' ' | '\t' => CharacterCategory::Space,
                    'A'..='Z' | 'a'..='z' => CharacterCategory::Letter,
                    '~' => CharacterCategory::Active,
                    '%' => CharacterCategory::Comment,
                    '\u{7f}' => CharacterCategory::Invalid,
                    _ => CharacterCategory::Other,
                },
            );
        }
    }
    pub fn set(&mut self, chr: char, cat: CharacterCategory) {
        self.0.insert(chr, cat);
    }
    pub fn get(&self, chr: char) -> Option<CharacterCategory> {
        match self.0.get(&chr) {
            Some(a) => Some(a.clone()),
            None => None,
        }
    }
    pub fn copy(&self) -> Self {
        let mut map = HashMap::new();
        for (key, value) in self.0.iter() {
            map.insert(key.clone(), value.clone());
        }
        return Self(map);
    }
}

type ParameterType = u8;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Token {
    ControlSequence(String),
    Character(char, CharacterCategory),
    Parameter(ParameterType),
    Group(Vec<Token>),
}
impl Token {
    pub fn replace_parameters(&self, map: &HashMap<u8, Vec<Token>>) -> Result<Self, Error> {
        match self {
            Token::Parameter(_) => unreachable!(),
            Token::Group(tokens) => {
                let mut v = Vec::new();
                for token in tokens {
                    if let Token::Parameter(nr) = token {
                        let p = map.get(nr).unwrap();
                        for (i, t) in p.iter().enumerate() {
                            match (i, t) {
                                (0, Token::Character(_, CharacterCategory::BeginGroup)) => continue,
                                (n, Token::Character(_, CharacterCategory::EndGroup))
                                    if n + 1 == p.len() =>
                                {
                                    continue
                                }
                                (_, t) => v.push(t.clone()),
                            }
                        }
                    } else {
                        v.push(token.replace_parameters(map)?);
                    }
                }
                Ok(Token::Group(v))
            }
            a => Ok(a.clone()),
        }
    }
    pub fn flatten(&self) -> Vec<Token> {
        let mut v = Vec::new();
        if let Self::Group(group) = self {
            for g in group {
                v.append(&mut g.flatten());
            }
        } else {
            v.push(self.clone());
        }
        v
    }
}
