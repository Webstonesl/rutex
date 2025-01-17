use clap::{arg, Parser, ValueEnum};
use rutex;
fn print_greeting_line() {
    print!("rutex {} (", rutex::build_info::VERSION);
    if let Some(tag) = rutex::build_info::GIT_TAG {
        print!("tag: {}", tag);
    } else {
        print!("git hash {}", rutex::build_info::GIT_HASH);
    }
    if rutex::build_info::HAS_CHANGES {
        print!(", with uncommitted changes");
    }
    println!(")");
}

#[derive(ValueEnum, Debug, Clone)]
enum InteractionMode {
    Batch,
    NonStop,
    Scroll,
    ErrorStopMode,
}

#[derive(Parser, Debug)]
struct Options {
    /// Activate debug mode
    #[arg(short)]
    verbose: bool,
    #[arg(short, long)]
    interaction_mode: Option<InteractionMode>,

    /// The input file to process
    file: Option<String>,
}

fn main() {
    print_greeting_line();
    let opts = Options::parse();
    let mut parser = rutex::parsing::Parser::new();
    println!("{:?}", &opts);
    parser.input.push(if let Some(file) = opts.file {
        rutex::parsing::Input::new_from_file(&file, std::fs::File::open(file.clone()).unwrap())
    } else {
        rutex::parsing::Input::new_from_stdin()
    });
    parser.parse_token();
}
