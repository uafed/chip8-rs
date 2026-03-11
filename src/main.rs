#![allow(dead_code)]

use clap::{Parser, Subcommand};

use std::{
    fs::{self},
    io::{BufWriter, Error, ErrorKind, Result, Write, stdout},
    time::{Duration, Instant},
};

use crossterm::{
    ExecutableCommand, QueueableCommand, cursor,
    event::{self, Event, KeyCode},
    style::{Color, ResetColor, SetBackgroundColor},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
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

const SET_I_REG_MASK: u16 = 0xa000;
const JUMP_TO_ROUTINE_MASK: u16 = 0x0;
const LOAD_TO_REG_MASK: u16 = 0x6000;
const JUMP_TO_ADDR_MASK: u16 = 0x1000;
const DISPLAY_N_BYTE_SPRITE_MASK: u16 = 0xd000;
const LOAD_REG_X_TO_REG_Y_MASK: u16 = 0x8000;
const SKIP_NEXT_IF_REG_NOT_EQUAL_MASK: u16 = 0x4000;
const CALL_SUBROUTINE_WITH_INCREMENT_MASK: u16 = 0x2000;

#[derive(Debug)]
struct LoadValueToRegister {
    register: u8,
    value: u16,
}

#[derive(Debug)]
struct LoadRegXToRegY {
    x: u8,
    y: u8,
}

#[derive(Debug)]
struct SkipNextIfRegisterNotEqual {
    register: u8,
    value: u16,
}

#[derive(Debug)]
struct DisplayNByteSprite {
    x: u8,
    y: u8,
    n: u8,
}

#[derive(Debug)]
enum Instruction {
    Clear,
    DisplayNByteSprite(DisplayNByteSprite),
    JumpToAddr(u16),
    CallSubroutine(u16),
    CallSubroutineWithIncrement(u16),
    LoadRegXToRegY(LoadRegXToRegY),
    LoadValueToRegister(LoadValueToRegister),
    Return,
    SetIRegister(u16),
    SkipNextIfRegisterNotEqual(SkipNextIfRegisterNotEqual),
}

fn matches_by_mask(value: u16, mask: u16) -> bool {
    return (value & mask) == mask;
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let Commands::FromRomFile(RomFileArgs { path }) = &cli.command;

    fs::read(path)?.chunks_exact(2).for_each(|pair| {
        let instruction = ((pair[0] as u16) << 8) | (pair[1] as u16);
        println!("{:#06x}", instruction);
    });

    let instructions: Result<Vec<Instruction>> = fs::read(path)?
        .chunks_exact(2)
        .map(|pair| {
            let instruction = ((pair[0] as u16) << 8) | (pair[1] as u16);
            match instruction {
                0x00e0 => Ok(Instruction::Clear),
                0x00ee => Ok(Instruction::Return),
                instr if (instr & 0xfff) == instr => {
                    Ok(Instruction::CallSubroutine(instr & 0x0fff))
                }
                instr if matches_by_mask(instr, SET_I_REG_MASK) => {
                    Ok(Instruction::SetIRegister(instr & 0x0fff))
                }
                instr if matches_by_mask(instr, JUMP_TO_ADDR_MASK) => {
                    Ok(Instruction::JumpToAddr(instr & 0x0fff))
                }
                instr if matches_by_mask(instr, CALL_SUBROUTINE_WITH_INCREMENT_MASK) => {
                    Ok(Instruction::CallSubroutineWithIncrement(instr & 0x0fff))
                }
                instr if matches_by_mask(instr, LOAD_TO_REG_MASK) => {
                    Ok(Instruction::LoadValueToRegister(LoadValueToRegister {
                        register: ((instr & 0x0f00) >> 8) as u8,
                        value: instr & 0xff,
                    }))
                }
                instr if matches_by_mask(instr, SKIP_NEXT_IF_REG_NOT_EQUAL_MASK) => Ok(
                    Instruction::SkipNextIfRegisterNotEqual(SkipNextIfRegisterNotEqual {
                        register: ((instr & 0x0f00) >> 8) as u8,
                        value: instr & 0xff,
                    }),
                ),
                instr if matches_by_mask(instr, LOAD_REG_X_TO_REG_Y_MASK) => {
                    Ok(Instruction::LoadRegXToRegY(LoadRegXToRegY {
                        x: ((instr & 0x0f00) >> 8) as u8,
                        y: ((instr & 0x00f0) >> 4) as u8,
                    }))
                }
                instr if matches_by_mask(instr, DISPLAY_N_BYTE_SPRITE_MASK) => {
                    Ok(Instruction::DisplayNByteSprite(DisplayNByteSprite {
                        x: ((instr & 0x0f00) >> 8) as u8,
                        y: ((instr & 0x00f0) >> 4) as u8,
                        n: (instr & 0x000f) as u8,
                    }))
                }
                instr => Err(Error::new(
                    ErrorKind::Other,
                    format!("unknown instruction: {:#06x}", instr),
                )),
            }
        })
        .collect();

    let instructions = instructions?;

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
