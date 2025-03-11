use std::path::PathBuf;

use clap::Parser;
use rutex::{
    error::{Error, ErrorKind},
    state::State,
    Config,
};
#[derive(Parser)]
struct Arguments {
    inputfile: PathBuf,
}
impl TryInto<Config> for Arguments {
    type Error = Error;

    fn try_into(self) -> Result<Config, Self::Error> {
        let inf = self.inputfile.canonicalize()?;

        if !inf.try_exists()? {
            return Err(ErrorKind::DoesNotExistError.into());
        }
        if !inf.is_file() {
            return Err(Into::<Error>::into(ErrorKind::IoError).with_message("File is not a file."));
        }
        let path: PathBuf = inf.parent().unwrap().into();

        Ok(Config {
            working_directory: path,
            source: inf,
        })
    }
}
fn run_with(arguments: Arguments) -> Result<(), Error> {
    let c: Config = arguments.try_into()?;

    #[allow(warnings)]
    let mut state = State::try_from(c)?;
    let r = state.read_file()?;
    println!("{:?}", &r);
    Ok(())
}
fn run() -> Result<(), Error> {
    let a = Arguments::try_parse()?;
    run_with(a)
}
pub fn main() -> Result<(), ()> {
    if let Err(e) = run() {
        println!("{}", e);
        Err(())
    } else {
        Ok(())
    }
}
#[test]
fn test() -> Result<(), Error> {
    let arguments = Arguments {
        inputfile: PathBuf::from("/Users/webstones/Code/rutex/test.tex"),
    };
    run_with(arguments)
}
