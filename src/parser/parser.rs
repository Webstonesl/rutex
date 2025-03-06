
use super::lexer::CharacterCategory;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Character(char, CharacterCategory),
    ControlSequence(String),
    Parameter(char, u8),
}

impl ToString for Token {
    fn to_string(&self) -> String {
        match self {
            Token::Character(a, _) => a.to_string(),
            Token::ControlSequence(s) => s.clone(),
            Token::Parameter(c, u) => format!("{c}{u}"),
        }
    }
}
