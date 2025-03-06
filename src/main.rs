const CARGO_VERSION: &'static str = env!("CARGO_PKG_VERSION");
use clap::Parser;
use rutex::{parser::lexer::TexFile, TexState};

#[derive(Parser, Debug)]
struct CommandLineArgs {
    /// Input File
    input_file: String,
}

fn main() {
    let banner = format!("This is RuTeX, Version {}", CARGO_VERSION);
    println!("{}", banner);
    let command_line_args = CommandLineArgs::parse();
    let mut state = TexState::new();
    state.files.push(TexFile::new(command_line_args.input_file));
    // while let Err(e) = state.parse_and_execute() {
    // println!("{}", e);
    //     }
}
#[test]
fn test_main() {
    let mut state = TexState::new();
    state.files.push(TexFile::new(
        "/Users/webstones/Code/rutex/tex_source/test.tex".to_string(),
    ));
    while let Err(_) = state.parse_and_execute() {
        // println!("{}", e);
    }
}
