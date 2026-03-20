use byteorder::{BigEndian, WriteBytesExt};
use std::{
    fs::{File, read_to_string},
    io::{Error, ErrorKind, Result},
};

use clap::Parser;

use crate::assembler::{encode_instructions, parse_instructions};

#[derive(Parser, Debug)]
pub struct ToRomFileArgs {
    pub source: String,
    pub output_path: String,
}

pub fn assemble_to_rom_file(args: &ToRomFileArgs) -> Result<()> {
    let data = read_to_string(&args.source)?; 
    let (_, instructions) = parse_instructions(data.as_str())
        .map_err(|error| Error::new(ErrorKind::InvalidData, error.to_string()))?;
    let mut output_file = File::create(&args.output_path)?;

    for encoded in encode_instructions(instructions.as_slice()) {
        output_file.write_u16::<BigEndian>(encoded)?;
    }
    Ok(())
}
