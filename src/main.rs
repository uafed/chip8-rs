#![allow(dead_code)]
use chip8_rs::*;
use clap::Parser;

use std::{
    io::{BufWriter, Result, Write, stdout},
    time::Duration,
};

use crossterm::{
    ExecutableCommand, QueueableCommand, cursor,
    event::{
        self, Event, KeyCode, KeyEvent, KeyEventKind, KeyboardEnhancementFlags,
        PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
    },
    execute,
    style::{Color, ResetColor, SetBackgroundColor},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};

fn main() -> Result<()> {
    let cli = Cli::parse();
    let Commands::FromRomFile(RomFileArgs { path }) = &cli.command;

    let mut stdout = BufWriter::new(stdout());

    let start_x = 0;
    let start_y = 0;

    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(cursor::Hide)?;
    execute!(
        stdout,
        PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::REPORT_EVENT_TYPES)
    )?;

    let sidebar_width = 40;

    let mut chip8 = Chip8::new_from_program_file(path)?;
    let frame_w = chip8.frame_buffer[0].len() as u16;

    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let _ = execute!(std::io::stdout(), PopKeyboardEnhancementFlags);
        let _ = terminal::disable_raw_mode();
        let _ = std::io::stdout().execute(LeaveAlternateScreen);
        let _ = std::io::stdout().execute(cursor::Show);
        default_hook(info);
    }));

    loop {
        while chip8.is_waiting_for_key_press() {
            match event::read()? {
                Event::Key(k) if k.code == KeyCode::Char('q') => {}
                Event::Key(k) if k.is_press() => {
                    if let Some(index) = is_hex_key_event(k) {
                        chip8.respond_to_key_press(index);
                    }
                }
                _ => {}
            }
        }
        if event::poll(Duration::ZERO)? {
            match event::read()? {
                Event::Key(k) if k.code == KeyCode::Char('q') => break,
                Event::Key(k) => {
                    if let Some(index) = is_hex_key_event(k) {
                        chip8.set_key_state(index, k.kind == KeyEventKind::Press);
                    }
                }
                _ => {}
            }
        }

        chip8.tick()?;

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

        let sound_timer_elapsed = if let Some(start) = chip8.sound_timer_reference {
            format!("{:.2}", start.elapsed().as_secs_f32())
        } else {
            "<None>".to_string()
        };

        let delay_timer_elapsed = if let Some(start) = chip8.delay_timer_reference {
            format!("{:.2}", start.elapsed().as_secs_f32())
        } else {
            "<None>".to_string()
        };

        let other_registers = [
            String::from("-------"),
            format!("PC  = {0:#06X} ({0})", chip8.program_counter),
            format!("{}{}", instruction_label, instruction_text),
            format!("IR  = {0:#06X} ({0})", chip8.index_register),
            format!(
                "DT  = {0:#06X} ({0}) ({1} elapsed)",
                chip8.delay_register, delay_timer_elapsed
            ),
            format!(
                "ST  = {0:#06X} ({0}) ({1} elapsed)",
                chip8.sound_timer, sound_timer_elapsed
            ),
            String::from("-------"),
            String::from("Key states"),
        ];
        for (index, value) in other_registers.iter().enumerate() {
            stdout.queue(cursor::MoveTo(
                start_x + frame_w * 2,
                start_y + chip8.general_registers.len() as u16 + index as u16,
            ))?;
            write!(stdout, "{:<width$}", value, width = sidebar_width)?;
        }
        for key_index in 0..16 {
            stdout.queue(cursor::MoveTo(
                start_x + frame_w * 2,
                start_y
                    + chip8.general_registers.len() as u16
                    + other_registers.len() as u16
                    + key_index as u16,
            ))?;
            write!(
                stdout,
                "{0:X} = {1:<width$}",
                key_index,
                chip8.get_key_state(key_index),
                width = sidebar_width
            )?;
        }

        stdout.queue(ResetColor)?;
        stdout.flush()?;
    }

    terminal::disable_raw_mode()?;
    execute!(stdout, PopKeyboardEnhancementFlags)?;
    stdout.execute(LeaveAlternateScreen)?;
    stdout.execute(cursor::Show)?;

    Ok(())
}

fn is_hex_key_event(key: KeyEvent) -> Option<u8> {
    let char = key.code.as_char()?.to_ascii_lowercase();
    return if char.is_digit(16) {
        Some(char.to_digit(16)? as u8)
    } else {
        None
    };
}
