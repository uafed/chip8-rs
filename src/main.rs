use std::io::Result;

use chip8_rs::command_handlers::{
    Cli, Commands, from_rom_file::execute_rom_file, to_rom_file::assemble_to_rom_file,
};
use clap::Parser;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::FromRomFile(args) => execute_rom_file(&args)?,
        Commands::ToRomFile(args) => assemble_to_rom_file(&args)?,
    }

    Ok(())
}
