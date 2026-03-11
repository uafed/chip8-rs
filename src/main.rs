#![allow(dead_code)]

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
    let mut stdout = BufWriter::new(stdout());

    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(cursor::Hide)?;

    loop {
        if event::poll(Duration::ZERO)? {
            match event::read()? {
                Event::Key(k) if k.code == KeyCode::Char('q') => break,
                _ => {}
            }
        }

        for y in 0..32 {
            stdout.queue(cursor::MoveTo(0, y))?;
            for x in 0..64 {
                let r = ((x as f32 / 64.0f32) * 255.0) as u8;
                let g = ((y as f32 / 64.0f32) * 255.0) as u8;
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
