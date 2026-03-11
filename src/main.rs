#![allow(dead_code)]

use clap::{Parser, Subcommand};

use std::{
    io::{BufWriter, Result, Write, stdout},
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

fn main() -> Result<()> {
    let cli = Cli::parse();
    let Commands::FromRomFile(RomFileArgs { .. }) = &cli.command;

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
