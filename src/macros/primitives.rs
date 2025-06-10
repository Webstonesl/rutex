use crate::{
    error::{Error, ErrorKind},
    parser::{CharacterCategory, Token},
    state::State,
    writer::WriterTrait,
};

use super::{userdefined::UserDefinedMacro, MacroValue};
#[allow(unused_variables)]
pub fn def(state: &mut State) -> Result<(), Error> {
    state.set_expand(false);
    let command = state.get_token()?;
    let command = if let Token::ControlSequence(s) = command {
        s
    } else {
        return Err(Error::new(ErrorKind::InvalidCharacter));
    };
    let mut pattern = Vec::new();
    let mut last_parameter = 0;
    let replacement = loop {
        match state.get_token()? {
            g @ Token::Group(_) => break g,
            p @ Token::Parameter(nr) => {
                if nr != last_parameter + 1 {
                    return Err(Error::new_with_message(
                        ErrorKind::DuplicateParameters,
                        "Parameters must be numbered consequtively starting at 1",
                    ));
                } else {
                    last_parameter += 1
                }
                pattern.push(p);
            }
            t => pattern.push(t),
        }
    };

    state.define_macro(
        command.as_str(),
        MacroValue::UserdefinedMacro(UserDefinedMacro::new(
            pattern,
            vec![replacement],
            last_parameter,
        )),
    );
    state.set_expand(true);

    Ok(())
}
fn read_number(state: &mut State) -> Result<u8, Error> {
    let mut s = String::new();
    loop {
        match state.get_token()? {
            Token::Character(a @ '0'..='9', _) => s.push(a),
            a => {
                state.push_tokens(&[a]);
                break;
            }
        }
    }
    s.parse().map_err(Into::into)
}

pub fn write(state: &mut State) -> Result<(), Error> {
    let stream_number = dbg!(state.get_token()?);
    loop {
        if let Token::Character('0'..='9', _) = stream_number {}
    }

    let text = state.get_token()?;
    match state.output_streams.get_mut(&0) {
        Some(a) => eprintln!("{:?}", a.get_name()),
        None => todo!("Write Output Stream Error"),
    }
    // state.output_streams.get_mut(&nr).ok;
    todo!()
}
pub fn read(state: &mut State) -> Result<(), Error> {
    let nr = read_number(state)?;
    todo!()
}
pub fn openout(state: &mut State) -> Result<(), Error> {
    let nr = read_number(state)?;

    todo!()
}
pub fn openin(state: &mut State) -> Result<(), Error> {
    let nr = read_number(state)?;

    todo!()
}
pub fn global(state: &mut State) -> Result<(), Error> {
    state.set_global(true);
    state.read_element()?;
    state.set_global(false);
    Ok(())
}
