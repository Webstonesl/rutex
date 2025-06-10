#![allow(dead_code)]
use std::{error::Error, fs::OpenOptions, io::Read, path::PathBuf};

use lalrpop_util::lalrpop_mod;
lalrpop_mod!(grammar);

#[derive(Debug, Clone, Copy)]
pub(crate) enum Direction {
    LeftToRight,
    RightToLeft,
}
#[derive(Debug, Clone, Copy)]
pub(crate) struct NumberValue {
    value: u32,
    width: usize,
}
impl NumberValue {
    pub fn parse_value(s: &str) -> Self {
        NumberValue {
            width: s.len() / 2,
            value: u32::from_str_radix(s, 16).unwrap(),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub(crate) enum Value {
    Direction(Direction),
    Number(NumberValue),
}

#[derive(Debug)]
pub(crate) struct Mapping {
    in_map: Vec<Value>,
    out_map: Vec<Value>,
}
impl Mapping {
    fn get_string(&self) -> String {
        let mut out_map = String::new();

        for v in self.out_map.iter() {
            match v {
                Value::Number(NumberValue { value, .. }) => {
                    out_map.push(char::from_u32(*value).unwrap())
                }
                Value::Direction(Direction::LeftToRight) => out_map.push('\u{200E}'),
                Value::Direction(Direction::RightToLeft) => out_map.push('\u{200F}'),
            }
        }

        out_map
    }
    fn get_width(&self) -> Option<usize> {
        if self.in_map.len() == 1 {
            match self.in_map.first() {
                Some(Value::Number(number_value)) => Some(number_value.width),
                _ => None,
            }
        } else {
            None
        }
    }
}

pub struct Input(Vec<Value>);

pub struct Output {
    direction: Option<Direction>,
    outmap: String,
}

fn write_out(input_file: PathBuf) -> Result<(), Box<dyn Error>> {
    let mut buf = String::new();

    OpenOptions::new()
        .read(true)
        .open(&input_file)?
        .read_to_string(&mut buf)?;
    let mapping = grammar::MapsParser::new()
        .parse(&buf)
        .map_err(|a| a.to_string())?;

    let flattened = mapping
        .iter()
        .map(|a| a.get_width())
        .reduce(|a, b| if a == b { a } else { None })
        .flatten();
    if let Some(f) = flattened {
        eprintln!("pub &[(u{:?},Option<Direction>, &str)] = &[", f * 8);

        eprintln!("];");
    }
    Ok(())
}
pub mod file_iterator;
#[test]
fn test() {
    for fp in file_iterator::FileIterator::new(
        "/Users/webstones/Code/rutex/rutex_encodings/unicode_mappings_builder/MAPPINGS/".into(),
    )
    .unwrap()
    .filter(|a| a.is_file())
    {
        println!("{:?}", &fp);
        if let Err(e) = write_out(fp) {
            eprintln!("{}", e);
        }
        // match grammar::MapsParser::new().parse(&buf) {
        //     Ok(a) => println!(
        //         "{:?}",
        //     ),
        //     Err(e) => println!("{:?}", e),
        // }
    }
}
