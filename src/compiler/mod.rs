use std::{fs, io::Result};

use clap::Parser;

use crate::compiler::parse_program::parse_program;

#[derive(Parser, Debug)]
pub struct SourceFileArgs {
    path: String,
}

mod parse_program;

pub fn parse_source_file(args: SourceFileArgs) -> Result<()> {
    let source_code = fs::read_to_string(args.path)?;
    let (_, program) = parse_program(source_code.as_str()).expect("Failed to parse program");

    println!("Source code:\n{}\n---------", source_code);
    println!("{:#?}", program);
    Ok(())
}
