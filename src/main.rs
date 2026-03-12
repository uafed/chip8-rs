#![allow(dead_code)]
use chip8_rs::*;
use clap::Parser;

use std::{
    io::{BufWriter, Result, Write, stdout},
    time::Duration,
};

use crossterm::{
    ExecutableCommand, QueueableCommand, cursor,
    event::{self, Event, KeyCode},
    style::{Color, ResetColor, SetBackgroundColor},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};

fn main() -> Result<()> {
    let cli = Cli::parse();
    let Commands::FromRomFile(RomFileArgs { path }) = &cli.command;

    let mut chip8 = Chip8::new_from_program_file(path)?;
    let mut stdout = BufWriter::new(stdout());

    let start_x = 0;
    let start_y = 0;

    let frame_w = chip8.frame_buffer[0].len() as u16;

    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(cursor::Hide)?;

    let sidebar_width = 32;

    loop {
        if event::poll(Duration::ZERO)? {
            match event::read()? {
                Event::Key(k) if k.code == KeyCode::Char('q') => break,
                _ => {}
            }
        }

        if let Err(e) = chip8.tick() {
            terminal::disable_raw_mode()?;
            stdout.execute(LeaveAlternateScreen)?;
            stdout.execute(cursor::Show)?;
            panic!("{:?}", e);
        }

        for (y, row) in chip8.frame_buffer.iter().enumerate() {
            stdout.queue(cursor::MoveTo(start_x, start_y + y as u16))?;
            for &pixel in row {
                let color = if pixel > 0 {
                    Color::White
                } else {
                    Color::Black
                };
                stdout.queue(SetBackgroundColor(color))?;
                stdout.write_all(b" ")?;
                stdout.write_all(b" ")?;
            }
        }

        for (index, value) in chip8.general_registers.iter().enumerate() {
            stdout.queue(cursor::MoveTo(
                start_x + frame_w * 2,
                start_y + index as u16,
            ))?;
            let output = format!("V{:<2} = {1:#06X} ({1})", index, value);
            write!(stdout, "{:<width$}", output, width = sidebar_width)?;
        }

        let instruction_label = "INS = ";
        let instruction_text = if let Some(instruction) = chip8.current_instruction {
            format!("{}", instruction)
                .chars()
                .take(sidebar_width - instruction_label.len())
                .collect::<String>()
        } else {
            "<None>".to_string()
        };
        let other_registers = [
            String::from("-------"),
            format!("PC  = {0:#06X} ({0})", chip8.program_counter),
            format!("{}{}", instruction_label, instruction_text),
            format!("IR  = {0:#06X} ({0})", chip8.index_register),
            format!("DR  = {0:#06X} ({0})", chip8.delay_register),
            format!("TR  = {0:#06X} ({0})", chip8.timer_register),
        ];
        for (index, value) in other_registers.iter().enumerate() {
            stdout.queue(cursor::MoveTo(
                start_x + frame_w * 2,
                start_y + chip8.general_registers.len() as u16 + index as u16,
            ))?;
            write!(stdout, "{:<width$}", value, width = sidebar_width)?;
        }

        stdout.queue(ResetColor)?;
        stdout.flush()?;
    }

    terminal::disable_raw_mode()?;
    stdout.execute(LeaveAlternateScreen)?;
    stdout.execute(cursor::Show)?;

    Ok(())
}
