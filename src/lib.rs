use clap::{Parser, Subcommand};

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

#[derive(Parser, Debug)]
pub struct RomFileArgs {
    pub path: String,
}

pub mod assembler;
pub mod core;
pub use core::*;
