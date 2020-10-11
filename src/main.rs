extern crate regex;

use std::io::prelude::*;
use std::io::{BufReader, Result, Error};

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        println!("Usage: rust-grep [OPTION]... PATTERNS [FILE]...");
        return Ok(())
    }

    let search = match regex::Regex::new(&args[1]) {
        Ok(r) => r,
        Err(e) => return Err(Error::new(std::io::ErrorKind::InvalidData, e))
    };

    if args.len() == 2 {
        let stdin = std::io::stdin();
        return match_input(stdin.lock(), &search)
    }

    if args.len() == 3 {
        let file = std::fs::File::open(&args[2])?;
        let reader = BufReader::new(file);
        return match_input(reader, &search)
    }

    return Err(invalid_arguments(&args))
}

fn match_input<T: BufRead>(reader: T, search: &regex::Regex) -> Result<()> {
    for (i, line) in reader.lines().enumerate() {
        match line {
            Ok(l) => if search.is_match(l.as_str()) {
                println!("{:?}: {:?}", i, l)
            },
            Err(e) => println!("Error: {:?}", e)
        }
    }
    Ok(())
}

fn invalid_arguments(args: &Vec<String>) -> Error {
    return Error::new(std::io::ErrorKind::Other,  format!("Receiving an invalid parameter count: {:?}", args.len()).as_str())
}