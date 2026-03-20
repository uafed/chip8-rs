use clap::{Parser, Subcommand};

use crate::command_handlers::from_rom_file::RomFileArgs;
use crate::command_handlers::to_rom_file::ToRomFileArgs;
use crate::compiler::SourceFileArgs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    FromRomFile(RomFileArgs),
    ToRomFile(ToRomFileArgs),
    CompileSourceFile(SourceFileArgs),
}

pub mod from_rom_file;
pub mod to_rom_file;
