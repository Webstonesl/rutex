use std::{collections::HashMap, fmt::Debug};

#[derive(Debug, Clone, Copy, PartialEq)]
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

pub struct TexFile {
    file_name: String,
    path: String,
    contents: String,
    position: usize,
}

impl TexFile {
    pub fn new(path: String) -> Self {
        let file_name = path.split("/").last().unwrap().to_string();

        let contents = std::fs::read_to_string(&path).unwrap();
        Self {
            file_name,
            path,
            contents,
            position: 0,
        }
    }
    pub fn new_from_contents(name: String, contents: String) -> Self {
        Self {
            file_name: name,
            path: "custom".to_string(),
            contents,
            position: 0,
        }
    }
    pub fn get_text_position(&self) -> (usize, usize) {
        let (care, _) = self.contents.split_at(self.position);
        let mut line_nr = 1;
        let mut last_new_line = 0;
        for (i, c) in care.chars().enumerate() {
            if c == '\n' {
                line_nr += 1;
                last_new_line = i;
            }
        }

        (line_nr, care.len() - last_new_line)
    }
    pub fn get_current_char(&self, offset: usize) -> Option<char> {
        self.contents.chars().nth(self.position + offset)
    }
    pub fn advance(&mut self, offset: usize) -> Result<(), ()> {
        if self.contents.len() >= self.position + offset {
            self.position += offset;
            Ok(())
        } else {
            Err(())
        }
    }
}

impl Debug for TexFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("File")
            .field("file_name", &self.file_name)
            .field("path", &self.path)
            .field("position", &self.position)
            .finish()
    }
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
                    ' ' => CharacterCategory::Space,
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
