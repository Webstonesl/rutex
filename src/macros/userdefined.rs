use std::{
    collections::HashMap,
    hash::RandomState,
};

use crate::{
    error::{Error, ErrorKind},
    parser::{CharacterCategory, Token},
    state::State,
};

use super::patternmatcher;

#[derive(Clone, Debug)]
pub struct UserDefinedMacro {
    pattern: Vec<Token>,
    replacement: Vec<Token>,
    parameter_count: u8,
}

impl UserDefinedMacro {
    pub fn new(pattern: Vec<Token>, replacement: Vec<Token>, parameter_count: u8) -> Self {
        UserDefinedMacro {
            pattern,
            replacement,
            parameter_count,
        }
    }
    pub fn pattern(&self) -> &Vec<Token> {
        &self.pattern
    }
    pub fn replacement(&self) -> &[Token] {
        &self.replacement
    }
    #[allow(unused_variables)]
    pub fn run(&self, state: &mut State) -> Result<(), Error> {
        // if self.pattern.len() == 0 {
        //     state.push_tokens(&self.replacement);
        //     return Ok(());
        // }
        let mut parameters = HashMap::new();
        for p in 1..=self.parameter_count {
            parameters.insert(p, 0..0);
        }

        let mut tokens = Vec::new();

        let mut v = vec![];
        for t in self.pattern.iter() {
            v.push(match t {
                Token::Parameter(nr) => patternmatcher::MatcherElement::Capture(*nr, 1),
                t => patternmatcher::MatcherElement::Literal(t),
            });
        }
        let pattern_matcher: patternmatcher::PatternMatcher<'_, u8, Token> =
            patternmatcher::PatternMatcher::new(&v);
        let parameters = 'main_loop: loop {
            // todo!();
            let token = state.get_token()?;
            tokens.push(token);
            match pattern_matcher.find_match(&tokens) {
                patternmatcher::PatternMatchResult::Found(hash_map) => {
                    break 'main_loop HashMap::<u8, Vec<Token>, RandomState>::from_iter(
                        hash_map
                            .iter()
                            .map(|(&a, d)| (a, Vec::from(&tokens[d.clone()]))),
                    );
                }
                patternmatcher::PatternMatchResult::Never => {
                    return Err(Error::new(ErrorKind::PatternMatchError))
                }
                patternmatcher::PatternMatchResult::Possible => continue 'main_loop,
            }
        };
        let mut result = Vec::new();
        for r in self.replacement.iter() {
            for t in match r {
                Token::Parameter(n) => parameters.get(n).unwrap(),
                r => {
                    let mut a = r.replace_parameters(&parameters)?.flatten();
                    if let Some(Token::Character(_, CharacterCategory::BeginGroup)) = a.first() {
                        if let Some(Token::Character(_, CharacterCategory::EndGroup)) = a.last() {
                            a.remove(0);
                            a.pop();
                        }
                    }

                    result.append(&mut a);

                    continue;
                }
            } {
                result.append(&mut t.flatten());
            }
        }
        state.push_tokens(&result);

        // dbg!(result);

        Ok(())
    }
}
