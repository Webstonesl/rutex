use crate::{
    error::{Error, ErrorKind},
    macros::{MacroMap, MacroValue},
    parser::{CharacterCategory, CharacterMap},
};

pub struct GroupState {
    character_map: CharacterMap,
    macro_map: MacroMap,
    global: bool,
}
impl GroupState {
    pub fn new() -> Self {
        Self {
            character_map: CharacterMap::new_and_init(),
            macro_map: MacroMap::new_and_init(),
            global: false,
        }
    }
    pub fn child(&self) -> Self {
        return Self {
            character_map: CharacterMap::new(),
            macro_map: MacroMap::new(),
            global: self.global,
        };
    }
}

pub struct GroupStates(Vec<GroupState>);
impl GroupStates {
    pub fn new() -> Self {
        Self(vec![GroupState::new()])
    }

    pub fn push(&mut self) {
        self.0.push(self.0.last().unwrap().child())
    }

    pub fn pop(&mut self) -> Result<(), Error> {
        self.0.pop();
        if self.0.len() == 0 {
            Err(Error::new_with_message(
                ErrorKind::ArgumentError,
                "Last Group Popped",
            ))
        } else {
            Ok(())
        }
    }
    pub fn set_global(&mut self, global: bool) {
        self.0.last_mut().unwrap().global = global;
    }
    pub fn is_global(&self) -> bool {
        self.0.last().unwrap().global
    }
    pub fn get_macro(&self, command: &str) -> Result<MacroValue, Error> {
        for mv in self.0.iter().rev() {
            if let Some(v) = mv.macro_map.get(command) {
                return Ok(v);
            }
        }
        Err(Error::new_with_message(
            crate::error::ErrorKind::DoesNotExistError,
            format!("Macro '{}' does not exist", command),
        ))
    }
    pub fn set_macro_global(&mut self, command: &str, m: MacroValue, global: bool) {
        let map = if global {
            self.0.first_mut()
        } else {
            self.0.last_mut()
        }
        .unwrap();
        map.macro_map.set(command, m);
    }
    pub fn set_macro(&mut self, command: &str, m: MacroValue) {
        return self.set_macro_global(command, m, self.0.last().unwrap().global);
    }
    pub fn get_category(&self, chr: char) -> CharacterCategory {
        for mv in self.0.iter().rev() {
            if let Some(v) = mv.character_map.get(chr) {
                return v;
            }
        }
        CharacterCategory::Other
    }
    pub fn set_category_global(&mut self, chr: char, cat: CharacterCategory, global: bool) {
        let map = if global {
            self.0.first_mut()
        } else {
            self.0.last_mut()
        }
        .unwrap();
        map.character_map.set(chr, cat);
    }
    pub fn set_category(&mut self, chr: char, cat: CharacterCategory) {
        self.set_category_global(chr, cat, self.0.last().unwrap().global);
    }
}
