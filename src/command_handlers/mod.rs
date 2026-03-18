use clap::{Parser, Subcommand};

use crate::command_handlers::from_rom_file::RomFileArgs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    FromRomFile(RomFileArgs),
}

pub mod from_rom_file;
