use errors::Error;
use macros::{Macro, MacroMap};
use parser::{
    lexer::{CharacterCategory, CharacterMap, TexFile},
    parser::Token,
};

pub mod document_generation;
pub mod errors;
pub mod macros;
pub mod parser;
pub(crate) mod constants {
    #![allow(warnings)]
    pub const MEM_MAX: usize = 30000;
    pub const MEM_MIN: usize = 0;
    pub const BUF_SIZE: usize = 500;
    pub const ERROR_LINE: usize = 72;
    pub const HALF_ERROR_LINE: usize = 42;
    pub const MAX_PRINT_LINE: usize = 79;
    pub const STACK_SIZE: usize = 200;
    pub const MAX_IN_OPEN: usize = 6;
    pub const FONT_MAX: usize = 75;
    pub const FONT_MEM_SIZE: usize = 20000;
    pub const PARAM_SIZE: usize = 60;
    pub const NEST_SIZE: usize = 40;
    pub const MAX_STRINGS: usize = 3000;
    pub const STRING_VACANCIES: usize = 8000;
    pub const POOL_SIZE: usize = 32000;
    pub const SAVE_SIZE: usize = 600;
    pub const TRIE_SIZE: usize = 8000;
    pub const TRIE_OP_SIZE: usize = 500;
    pub const DVI_BUF_SIZE: usize = 800;
    pub const FILE_NAME_SIZE: usize = 40;
    pub const POOL_NAME: &'static str = "TeXformats:TEX.POOL                     ";
}
#[derive(Clone, Debug)]
pub struct TexGroupState {
    parent: Option<Box<TexGroupState>>,
    character_map: CharacterMap,
    macro_map: MacroMap,
    global_defs: bool,
}

impl TexGroupState {
    pub fn initial() -> Self {
        TexGroupState {
            parent: None,
            character_map: CharacterMap::new_and_init(),
            macro_map: MacroMap::new_and_init(),
            global_defs: false,
        }
    }

    pub fn child(self) -> Self {
        TexGroupState {
            character_map: CharacterMap::new(),
            macro_map: MacroMap::new(),
            global_defs: self.global_defs.clone(),
            parent: Some(Box::new(self)),
        }
    }

    pub fn get_category(&self, c: char) -> CharacterCategory {
        match self.character_map.clone().get(c) {
            Some(c) => c,
            None => {
                if let Some(p) = &self.parent {
                    p.get_category(c)
                } else {
                    CharacterCategory::Other
                }
            }
        }
    }

    pub fn get_global_defs(&self) -> bool {
        self.global_defs
    }
    pub fn set_global_defs(&mut self, b: bool) {
        self.global_defs = b;
    }
    pub fn get_macro(&self, s: &String) -> Option<&Box<dyn Macro>> {
        if let Some(s) = self.macro_map.get(s) {
            Some(s)
        } else if let Some(ref p) = self.parent {
            p.get_macro(s)
        } else {
            None
        }
    }

    pub fn set_category_with_global(&mut self, chr: char, cat: CharacterCategory, global: bool) {
        if global {
            if let Some(ref mut p) = self.parent {
                return p.set_category_with_global(chr, cat, global);
            }
        }
        self.character_map.set(chr, cat);
    }

    pub fn set_macro_with_global(&mut self, r#macro: Box<dyn Macro>, global: bool) {
        if global {
            if let Some(ref mut p) = self.parent {
                return p.set_macro_with_global(r#macro, global);
            }
        }
        let name = r#macro.name();
        self.macro_map.set(name, r#macro);
    }
    pub fn set_category(&mut self, chr: char, cat: CharacterCategory) {
        self.set_category_with_global(chr, cat, self.get_global_defs());
    }
    pub fn set_macro(&mut self, r#macro: Box<dyn Macro>) {
        self.set_macro_with_global(r#macro, self.global_defs);
    }

    pub fn pop(self) -> Option<Self> {
        if let Some(x) = self.parent {
            Some(*x)
        } else {
            None
        }
    }
    pub fn run_macro(&mut self, s: &String, state: &mut TexState) -> Result<(), Error> {
        if let Some(m) = self.get_macro(&s.clone()) {
            m.run(state)
        } else {
            Err(Error::new(
                errors::ErrorKind::UnknownMacroError,
                format!("Command {:?} not found", s),
            ))
        }
    }
}

pub struct TexState {
    pub files: Vec<TexFile>,
    pub state: TexGroupState,
}
unsafe impl Sync for TexState {}
unsafe impl Send for TexState {}

impl TexState {
    pub fn new() -> Self {
        TexState {
            files: vec![],
            state: TexGroupState::initial(),
        }
    }
    pub fn add_file(&mut self, file: TexFile) {
        self.files.push(file);
    }
    pub fn advance(&mut self) {
        if let Some(file) = self.files.last_mut() {
            if let Err(_) = file.advance(1) {
                self.files.pop();
            }
        }
    }
    #[inline]
    pub fn advance_by(&mut self, n: usize) {
        for _ in 0..n {
            self.advance();
        }
    }
    pub fn read_character(&mut self) -> Option<char> {
        if let Some(file) = self.files.last_mut() {
            let m = file.get_current_char(0);
            if let Err(_) = file.advance(1) {
                self.files.pop();
            }
            m
        } else {
            None
        }
    }
    pub fn read_ahead_character(&mut self, n: usize) -> Option<char> {
        if let Some(file) = self.files.last_mut() {
            file.get_current_char(n)
        } else {
            None
        }
    }
    #[inline]
    fn get_category(&self, c: char) -> CharacterCategory {
        self.state.get_category(c)
    }
    fn get_char_and_category(&mut self, n: usize) -> Option<(char, CharacterCategory)> {
        let c = match self.read_ahead_character(n) {
            Some(c) => c,
            None => return None,
        };
        Some((c, self.get_category(c)))
    }
    fn get_control(&mut self) -> Result<Token, Error> {
        if let Some(c) = self.read_character() {
            let cat = self.get_category(c);
            let mut command = String::new();
            command.push('\\');
            command.push(c);
            if cat == CharacterCategory::Letter {
                let mut n = 0;
                let mut space = false;
                loop {
                    match self.get_char_and_category(n) {
                        Some((c, CharacterCategory::Letter)) if !space => command.push(c),
                        Some((_, CharacterCategory::Space)) => {
                            if space {
                            } else {
                                space = true;
                            }
                        }
                        _ => break,
                    }
                    n += 1;
                }
                for _ in 0..n {
                    self.advance();
                }
            }
            Ok(Token::ControlSequence(command))
        } else {
            Err(Error::eof())
        }
    }
    pub fn get_element(&mut self) -> Result<Token, Error> {
        let c = match self.read_character() {
            Some(c) => c,
            None => return Err(Error::eof()),
        };

        match self.get_category(c) {
            CharacterCategory::Escape => self.get_control(),
            CharacterCategory::Active => Ok(Token::ControlSequence(c.to_string())),
            CharacterCategory::Parameter => {
                let mut s = String::new();
                let mut n = 0;
                while let Some(c) = self.read_ahead_character(n) {
                    if c.is_numeric() {
                        s.push(c);

                        n += 1;
                    } else {
                        break;
                    }
                }
                self.advance_by(n);
                Ok(Token::Parameter(c, s.parse()?))
            }
            CharacterCategory::Comment => {
                self.skip_comment()?;
                self.get_element()
            }
            a => Ok(Token::Character(c, a)),
        }
    }
    pub fn push_group(&mut self) {
        self.state = self.state.clone().child();
    }
    pub fn pop_group(&mut self) -> Result<(), Error> {
        let state = self.state.clone().pop();
        self.state = state.ok_or(Error::new(
            errors::ErrorKind::UnknownError,
            "No states left to pop".to_string(),
        ))?;
        Ok(())
    }
    pub fn parse_and_execute(&mut self) -> Result<(), Error> {
        loop {
            self.parse_and_execute_one()?;
        }
        // Ok(())
    }
    pub fn parse_and_execute_one(&mut self) -> Result<(), Error> {
        let token = self.get_element()?;
        self.execute_token(token.clone())?;
        Ok(())
    }
    pub fn execute_token(&mut self, token: Token) -> Result<(), Error> {
        match token.clone() {
            Token::ControlSequence(s) => self.state.clone().run_macro(&s, self)?,
            Token::Character(_, CharacterCategory::BeginGroup) => self.push_group(),
            Token::Character(_, CharacterCategory::EndGroup) => self.pop_group()?,
            Token::Character(chr, cat) => println!("{chr:?} {cat:?}"),
            Token::Parameter(_, _) => {
                return Err(Error::new(
                    errors::ErrorKind::UnknownError,
                    "Parameter in text".to_string(),
                ))
            }
        };
        Ok(())
    }
    pub fn define(&mut self, d: Box<dyn Macro>) {
        self.state.set_macro(d);
    }

    fn skip_comment(&mut self) -> Result<(), Error> {
        while let Some(e) = self.read_character() {
            if self.get_category(e) == CharacterCategory::EndOfLine {
                return Ok(());
            }
        }
        Err(Error::eof())
    }
}
