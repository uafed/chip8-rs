#![allow(dead_code)]

use clap::{Parser, Subcommand};

use std::{
    fs::File,
    io::{BufWriter, Error, ErrorKind, Read, Result, Write, stdout},
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

const SET_INDEX_REGISTER: u16 = 0xa000;
const JUMP_TO_ROUTINE_MASK: u16 = 0x0;
const LOAD_TO_REGISTER: u16 = 0x6000;
const JUMP_TO_ADDRESS: u16 = 0x1000;
const DISPLAY_N_BYTE_SPRITE: u16 = 0xd000;
const ADD_VALUE_TO_REGISTER: u16 = 0x7000;
const LOAD_REG_X_TO_REG_Y_MASK: u16 = 0x8000;
const SKIP_NEXT_IF_REG_NOT_EQUAL_MASK: u16 = 0x4000;
const CALL_SUBROUTINE_WITH_INCREMENT_MASK: u16 = 0x2000;

fn matches_by_mask(value: u16, mask: u16) -> bool {
    return (value & mask) == mask;
}

#[derive(Debug)]
struct Chip8 {
    frame_buffer: [[u8; 64]; 32],
    memory: [u8; 4096],
    general_registers: [u8; 16],
    index_register: u16,

    delay_register: u8,
    timer_register: u8,

    program_counter: u16,
    stack_pointer: u8,
}

const SPRITES: [[u8; 5]; 16] = [
    [0xF0, 0x90, 0x90, 0x90, 0xF0], // '0'
    [0x20, 0x60, 0x20, 0x20, 0x70], // '1'
    [0xF0, 0x10, 0xf0, 0x80, 0xf0], // '2'
    [0xF0, 0x10, 0xf0, 0x10, 0xF0], // '3'
    [0x90, 0x90, 0xf0, 0x10, 0x10], // '4'
    [0xF0, 0x80, 0xf0, 0x10, 0xf0], // '5'
    [0xF0, 0x80, 0xf0, 0x90, 0xF0], // '6'
    [0x70, 0x10, 0x20, 0x40, 0x40], // '7'
    [0xF0, 0x90, 0xf0, 0x90, 0xF0], // '8'
    [0xF0, 0x90, 0xf0, 0x10, 0xF0], // '9'
    [0xF0, 0x90, 0x90, 0x90, 0xF0], // 'A'
    [0xF0, 0x90, 0x90, 0x90, 0xF0], // 'B'
    [0xF0, 0x90, 0x90, 0x90, 0xF0], // 'C'
    [0xF0, 0x90, 0x90, 0x90, 0xF0], // 'D'
    [0xF0, 0x90, 0x90, 0x90, 0xF0], // 'E'
    [0xF0, 0x90, 0x90, 0x90, 0xF0], // 'F'
];

const PROGRAM_START_OFFSET: usize = 0x200;

impl Chip8 {
    fn new(program_path: &str) -> Result<Self> {
        let mut instance = Self {
            memory: [0; 4096],
            general_registers: [0; 16],
            frame_buffer: [[0; 64]; 32],
            index_register: 0,
            delay_register: 0,
            timer_register: 0,
            program_counter: PROGRAM_START_OFFSET as u16,
            stack_pointer: 0,
        };

        SPRITES.iter().enumerate().for_each(|(index, item)| {
            let start = index * item.len();
            let end = start + item.len();
            instance.memory[start..end].copy_from_slice(item);
        });

        instance.load_program(program_path)?;
        Ok(instance)
    }

    fn load_program(&mut self, path: &str) -> Result<()> {
        let mut file = File::open(path)?;
        file.read(&mut self.memory[(PROGRAM_START_OFFSET as usize)..])?;
        Ok(())
    }

    fn read_instruction(&mut self) -> u16 {
        let start = self.program_counter as usize;
        let bytes = &self.memory[start..start + 2];

        self.program_counter += 2;
        return ((bytes[0] as u16) << 8) | bytes[1] as u16;
    }

    fn execute_instruction(&mut self, instruction: u16) -> Result<()> {
        match instruction {
            0x00e0 => {
                // Clear the screen
                for row in self.frame_buffer.iter_mut() {
                    for pixel in row {
                        *pixel = 0x00;
                    }
                }
            }
            // 0x00ee => {}
            // instr if (instr & 0xfff) == instr => {}
            instr if (instr & 0xF000) == SET_INDEX_REGISTER => {
                let value = (instr & 0xfff) as u16;
                self.index_register = value;
            }
            // instr if matches_by_mask(instr, CALL_SUBROUTINE_WITH_INCREMENT_MASK) => {}
            instr if (instr & 0xF000) == LOAD_TO_REGISTER => {
                let register = (instr & 0x0f00) >> 8;
                let value = (instr & 0xff) as u8;
                self.general_registers[register as usize] = value;
            }
            // instr if matches_by_mask(instr, SKIP_NEXT_IF_REG_NOT_EQUAL_MASK) => {}
            // instr if matches_by_mask(instr, LOAD_REG_X_TO_REG_Y_MASK) => {}
            instr if (instr & 0xF000) == DISPLAY_N_BYTE_SPRITE => {
                let start_x = self.general_registers[((instr & 0x0f00) >> 8) as usize] as usize;
                let start_y = self.general_registers[((instr & 0x00f0) >> 4) as usize] as usize;
                let n = (instr & 0x000f) as usize;

                for byte in 0..n {
                    let sprite = self.memory[self.index_register as usize + byte];
                    for x in 0..8 {
                        let bit = (sprite & (1 << (7 - x))) >> (7 - x);
                        self.frame_buffer[start_y + byte][start_x + x] ^= bit;
                    }
                }
            }
            instr if (instr & 0xF000) == ADD_VALUE_TO_REGISTER => {
                let register = (instr & 0x0f00) >> 8;
                let value = (instr & 0xff) as u8;
                self.general_registers[register as usize] += value;
            }
            instr if (instr & 0xF000) == JUMP_TO_ADDRESS => {
                let address = (instr & 0xfff) as u16;
                self.program_counter = address;
            }
            _ => {
                return Err(Error::new(
                    ErrorKind::Other,
                    format!(
                        "Unrecognized instruction: {instruction:#06x} {} ({:#06x})",
                        self.program_counter - 2,
                        self.program_counter - 2
                    )
                    .to_owned(),
                ));
            }
        }

        Ok(())
    }

    fn tick(&mut self) -> Result<()> {
        let instruction = self.read_instruction();
        // println!("instr: {:#06x}", instruction);
        self.execute_instruction(instruction)?;
        Ok(())
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let Commands::FromRomFile(RomFileArgs { path }) = &cli.command;

    let fps_target = 60;
    let ms_per_frame_target = Duration::from_millis(1000 / fps_target);
    let (terminal_w, terminal_h) = terminal::size()?;

    let mut chip8 = Chip8::new(path)?;
    let mut stdout = BufWriter::new(stdout());

    let frame_h = chip8.frame_buffer.len() as u16;
    let start_y = (terminal_h - frame_h) / 2;

    let frame_w = chip8.frame_buffer[0].len() as u16;
    let start_x = (terminal_w - frame_w) / 2;

    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(cursor::Hide)?;

    loop {
        let start = Instant::now();
        if event::poll(Duration::ZERO)? {
            match event::read()? {
                Event::Key(k) if k.code == KeyCode::Char('q') => break,
                _ => {}
            }
        }

        if let Err(e) = chip8.tick() {
            panic!("{:?}", e);
        }

        // draw the screen
        for (y, row) in chip8.frame_buffer.iter().enumerate() {
            stdout.queue(cursor::MoveTo(start_x + 0, start_y + y as u16))?;
            for &pixel in row {
                let color = if pixel > 0 {
                    Color::White
                } else {
                    Color::Black
                };
                stdout.queue(SetBackgroundColor(color))?;
                stdout.write_all(b" ")?;
            }
        }

        // draw some stats
        for (index, value) in chip8.general_registers.iter().enumerate() {
            stdout.queue(cursor::MoveTo(start_x + frame_w, start_y + index as u16))?;
            let output = format!("V{:<2} = {1:#06x} ({1})", index, value);
            stdout.write_all(output.as_bytes())?;
        }

        let other_registers = [
            String::from("-------"),
            format!("PC  = {0:#06x} ({0})", chip8.program_counter),
            format!("IR  = {0:#06x} ({0})", chip8.index_register),
            format!("DR  = {0:#06x} ({0})", chip8.delay_register),
            format!("TR  = {0:#06x} ({0})", chip8.timer_register),
        ];
        for (index, value) in other_registers.iter().enumerate() {
            stdout.queue(cursor::MoveTo(
                start_x + frame_w,
                start_y + chip8.general_registers.len() as u16 + index as u16,
            ))?;
            stdout.write_all(value.as_bytes())?;
        }

        stdout.queue(ResetColor)?;
        stdout.flush()?;

        let elapsed = start.elapsed();

        if elapsed < ms_per_frame_target {
            std::thread::sleep(ms_per_frame_target - elapsed);
        }
    }

    terminal::disable_raw_mode()?;
    stdout.execute(LeaveAlternateScreen)?;
    stdout.execute(cursor::Show)?;

    Ok(())
}
