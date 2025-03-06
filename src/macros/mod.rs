use std::collections::HashMap;
use std::fmt::Debug;

use crate::errors::Error;
use crate::parser::parser::Token;
use crate::TexState;
use dyn_clone::DynClone;

mod pattern_matcher;
use pattern_matcher::*;
pub mod primitives;
pub trait Macro: DynClone + Debug {
    fn name<'a>(&self) -> String;
    fn run(&self, state: &mut TexState) -> Result<(), Error>;
    fn safe(&self, state: &TexState) -> bool;
}
dyn_clone::clone_trait_object!(Macro);

#[derive(Clone, Debug)]
pub struct UserDefinedMacro {
    name: String,
    parameters: Vec<Token>,
    replacements: Vec<Token>,
    parameter_count: u8,
}
impl UserDefinedMacro {
    fn new(
        name: String,
        parameters: Vec<Token>,
        replacements: Vec<Token>,
        parameter_count: u8,
    ) -> UserDefinedMacro {
        Self {
            name,
            parameters,
            replacements,
            parameter_count,
        }
    }
}
impl Macro for UserDefinedMacro {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn run(&self, state: &mut TexState) -> Result<(), Error> {
        // match_pattern(&self.parameters, state);
        todo!("{}", self.name());
    }

    fn safe(&self, _: &TexState) -> bool {
        todo!()
    }
}
#[derive(Clone, Debug)]
pub struct MacroMap(HashMap<String, Box<dyn Macro>>);

impl MacroMap {
    pub fn new() -> Self {
        MacroMap(HashMap::new())
    }
    pub fn init(&mut self) {
        self.0
            .insert(r"\def".to_string(), Box::new(primitives::Def));
    }
    pub fn new_and_init() -> Self {
        let mut map = Self::new();
        map.init();
        map
    }
    pub fn get(&self, s: &String) -> Option<&Box<dyn Macro>> {
        self.0.get(s)
    }
    pub fn contains(&self, s: String) -> bool {
        self.0.contains_key(&s)
    }
    pub fn set(&mut self, s: String, mcro: Box<dyn Macro>) {
        self.0.insert(s, mcro);
    }
}
