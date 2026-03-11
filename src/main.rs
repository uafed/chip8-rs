#![allow(dead_code)]

use clap::{Parser, Subcommand};

use std::{
    fs::{self},
    io::{BufWriter, Result, Write, stdout},
    time::{Duration, Instant},
};

use crossterm::{
    ExecutableCommand, QueueableCommand, cursor,
    event::{self, Event, KeyCode},
    style::{Color, ResetColor, SetBackgroundColor},
    terminal::{self, Clear, EnterAlternateScreen, LeaveAlternateScreen},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    FromRomFile(RomFileArgs),
}

#[derive(Parser, Debug)]
struct RomFileArgs {
    path: String,
}

const SET_REGISTER_MASK: u16 = 0x0600;
const SET_I_REG_MASK: u16 = 0xa000;
const LOAD_TO_REG_MASK: u16 = 0x6000;

#[derive(Debug)]
struct LoadValueToRegister {
    register: u8,
    value: u16,
}

#[derive(Debug)]
enum Instruction {
    Clear,
    Return,
    SetIRegister(u16),
    LoadValueToRegister(LoadValueToRegister),
}

fn matches_by_mask(value: u16, mask: u16) -> bool {
    return (value & mask) == mask;
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let Commands::FromRomFile(RomFileArgs { path }) = &cli.command;

    let instructions: Vec<Option<Instruction>> = fs::read(path)?
        .chunks_exact(2)
        .map(|pair| {
            let instruction = ((pair[0] as u16) << 8) | (pair[1] as u16);
            match instruction {
                0x00e0 => Some(Instruction::Clear),
                0x00ee => Some(Instruction::Return),
                instr if matches_by_mask(instr, SET_I_REG_MASK) => {
                    let value = instr & 0x0fff;
                    Some(Instruction::SetIRegister(value))
                }
                instr if matches_by_mask(instr, SET_REGISTER_MASK) => {
                    let register = ((instr & 0x0f00) >> 8) as u8;
                    let value = instr & 0xff;
                    Some(Instruction::LoadValueToRegister(LoadValueToRegister {
                        register,
                        value,
                    }))
                }
                _ => None,
            }
        })
        .collect();

    for instr in instructions {
        println!("{:?}", instr);
    }

    let mut stdout = BufWriter::new(stdout());

    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(cursor::Hide)?;

    let time = Instant::now();
    loop {
        if event::poll(Duration::ZERO)? {
            match event::read()? {
                Event::Key(k) if k.code == KeyCode::Char('q') => break,
                _ => {}
            }
        }

        for y in 0..32 {
            stdout.queue(cursor::MoveTo(0, y))?;
            for _ in 0..64 {
                let t = time.elapsed().as_secs_f32();
                let r = ((t - t.floor()) * 255.0) as u8;
                let g = ((((y as f32) / 64.0f32) % 64.0) * 255.0) as u8;
                let b = (0.2 * 255.0) as u8;
                stdout.queue(SetBackgroundColor(Color::Rgb { r, g, b }))?;
                stdout.write_all(b" ")?;
            }
        }

        stdout.queue(ResetColor)?;
        stdout.flush()?;
    }

    terminal::disable_raw_mode()?;
    stdout.execute(LeaveAlternateScreen)?;
    stdout.execute(cursor::Show)?;

    Ok(())
}
