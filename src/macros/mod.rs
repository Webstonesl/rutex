use std::{collections::HashMap, hash::RandomState};

use crate::{error::Error, state::State};

mod patternmatcher;
mod userdefined;
use userdefined::*;

pub mod primitives;
// impl std::intrinsics::mir::Call for UserdefinedMacro {}

const PRE_DEF_ITEMS: &'static [(&'static str, MacroValue)] = &[
    (r"\def", MacroValue::PredefinedMacro(&primitives::def)),
    (r"\global", MacroValue::PredefinedMacro(&primitives::global)),
    (r"\write", MacroValue::PredefinedMacro(&primitives::write)),
    (r"\read", MacroValue::PredefinedMacro(&primitives::read)),
    (
        r"\openout",
        MacroValue::PredefinedMacro(&primitives::openout),
    ),
    (r"\openin", MacroValue::PredefinedMacro(&primitives::openin)),
];

pub struct MacroMap(HashMap<String, MacroValue>);

impl MacroMap {
    pub fn new() -> Self {
        MacroMap(HashMap::new())
    }

    pub fn new_and_init() -> Self {
        let map: HashMap<String, MacroValue, RandomState> = HashMap::from_iter(
            PRE_DEF_ITEMS
                .iter()
                .map(|(s, mv)| (s.to_string(), mv.clone())),
        );

        MacroMap(map)
    }

    pub fn get(&self, s: &str) -> Option<MacroValue> {
        match self.0.get(s) {
            None => None,
            Some(r) => Some(r.clone()),
        }
    }
    pub fn set(&mut self, s: &str, m: MacroValue) {
        self.0.insert(s.to_string(), m);
    }
}

#[derive(Clone)]
pub enum MacroValue {
    UserdefinedMacro(UserDefinedMacro),
    PredefinedMacro(&'static dyn Fn(&mut State) -> Result<(), Error>),
    Counter(usize),
}

impl MacroValue {
    pub fn run(&self, state: &mut State) -> Result<(), Error> {
        match self {
            MacroValue::UserdefinedMacro(user_defined_macro) => user_defined_macro.run(state),
            MacroValue::PredefinedMacro(r) => r(state),
            MacroValue::Counter(_) => todo!(),
        }
    }
}
