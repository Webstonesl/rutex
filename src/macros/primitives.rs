use crate::parser::{lexer::CharacterCategory, parser::Token};

use super::*;
#[derive(Clone, Debug)]
pub struct Def;

impl Macro for Def {
    fn name(&self) -> String {
        return r"\def".to_string();
    }

    fn run(&self, state: &mut TexState) -> Result<(), Error> {
        state.push_group();
        let command = state.get_element()?;
        let mut parameters: Vec<Token> = Vec::new();
        let mut parameter_count = 0;
        let mut replacements = Vec::new();
        loop {
            match state.get_element()? {
                t @ Token::Character(_, CharacterCategory::BeginGroup) => {
                    replacements.push(t);
                    break;
                }
                a @ Token::Parameter(_, _) => {
                    parameter_count += 1;
                    parameters.push(a);
                }
                a => {
                    parameters.push(a);
                }
            }
        }
        state.push_group();

        loop {
            match state.get_element()? {
                t @ Token::Character(_, CharacterCategory::EndGroup) => {
                    replacements.push(t);
                    break;
                }
                a => {
                    replacements.push(a);
                }
            }
        }

        state.pop_group()?;
        state.pop_group()?;
        state.define(Box::new(UserDefinedMacro {
            name: command.to_string(),
            parameters,
            replacements,
            parameter_count,
        }));

        Ok(())
    }

    fn safe(&self, _: &TexState) -> bool {
        todo!()
    }
}
