use clap::{Parser, Subcommand};

use std::{
    fs::File,
    io::{Error, ErrorKind, Read, Result},
};

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

pub const SET_INDEX_REGISTER: u16 = 0xa000;
pub const JUMP_TO_ROUTINE_MASK: u16 = 0x0;
pub const LOAD_TO_REGISTER: u16 = 0x6000;
pub const JUMP_TO_ADDRESS: u16 = 0x1000;
pub const DISPLAY_N_BYTE_SPRITE: u16 = 0xd000;
pub const ADD_VALUE_TO_REGISTER: u16 = 0x7000;
pub const ADD_VALUE_TO_INDEX_REGISTER: u16 = 0xf01e;
pub const LOAD_REG_X_TO_REG_Y_MASK: u16 = 0x8000;
pub const XOR_REGISTERS_X_AND_Y: u16 = 0x8003;
pub const ADD_WITH_CARRY: u16 = 0x8004;
pub const SUBTRACT_WITH_BORROW: u16 = 0x8005;
pub const SKIP_NEXT_IF_REGISTERS_ARE_EQUAL: u16 = 0x5000;
pub const SKIP_NEXT_IF_REGISTERS_NOT_EQUAL: u16 = 0x9000;
pub const SKIP_NEXT_IF_REGISTER_IS_EQUAL: u16 = 0x3000;
pub const STORE_REGISTERS_TO_INDEX_ADDRESS: u16 = 0xf055;
pub const LOAD_INDEX_ADDRESS_TO_REGISTERS: u16 = 0xf065;
pub const SKIP_NEXT_IF_REG_NOT_EQUAL_MASK: u16 = 0x4000;
pub const CALL_SUBROUTINE_WITH_INCREMENT_MASK: u16 = 0x2000;

#[derive(Debug)]
pub struct Chip8 {
    pub current_instruction: u16,
    pub delay_register: u8,
    pub frame_buffer: [[u8; 64]; 32],
    pub general_registers: [u8; 16],
    pub index_register: u16,
    pub memory: [u8; 4096],
    pub program_counter: u16,
    pub stack_pointer: u8,
    pub timer_register: u8,
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

pub const PROGRAM_START_OFFSET: usize = 0x200;

impl Chip8 {
    pub fn new() -> Self {
        let mut instance = Self {
            current_instruction: 0,
            delay_register: 0,
            frame_buffer: [[0; 64]; 32],
            general_registers: [0; 16],
            index_register: 0,
            memory: [0; 4096],
            program_counter: PROGRAM_START_OFFSET as u16,
            stack_pointer: 0,
            timer_register: 0,
        };

        SPRITES.iter().enumerate().for_each(|(index, item)| {
            let start = index * item.len();
            let end = start + item.len();
            instance.memory[start..end].copy_from_slice(item);
        });

        instance
    }

    pub fn new_from_program(insructions: &[u16]) -> Self {
        let mut instance = Self::new();

        for (index, &instr) in insructions.iter().enumerate() {
            let bytes = instr.to_be_bytes();
            let start = index * 2;

            let destinations = &mut instance.memory[PROGRAM_START_OFFSET as usize + start..];
            destinations[0..2].copy_from_slice(&bytes);
        }

        instance
    }

    pub fn new_from_program_file(program_path: &str) -> Result<Self> {
        let mut instance = Self::new();
        instance.load_program(program_path)?;
        Ok(instance)
    }

    pub fn load_program(&mut self, path: &str) -> Result<()> {
        let mut file = File::open(path)?;
        file.read(&mut self.memory[(PROGRAM_START_OFFSET as usize)..])?;
        Ok(())
    }

    pub fn read_instruction(&mut self) -> u16 {
        let start = self.program_counter as usize;
        let bytes = &self.memory[start..start + 2];

        let instruction = ((bytes[0] as u16) << 8) | bytes[1] as u16;

        self.current_instruction = instruction;
        self.program_counter += 2;
        return instruction;
    }

    pub fn execute_instruction(&mut self, instruction: u16) -> Result<()> {
        match instruction {
            0x00e0 => {
                for row in self.frame_buffer.iter_mut() {
                    for pixel in row {
                        *pixel = 0x00;
                    }
                }
            }
            instr if (instr & 0xF000) == SET_INDEX_REGISTER => {
                let value = (instr & 0xfff) as u16;
                self.index_register = value;
            }
            instr if (instr & 0xF000) == LOAD_TO_REGISTER => {
                let register = (instr & 0x0f00) >> 8;
                let value = (instr & 0xff) as u8;
                self.general_registers[register as usize] = value;
            }
            instr if (instr & 0xf00f) == LOAD_REG_X_TO_REG_Y_MASK => {
                let register_x = ((instr & 0x0f00) >> 8) as usize;
                let register_y = ((instr & 0x00f0) >> 4) as usize;
                self.general_registers[register_x] = self.general_registers[register_y];
            }
            instr if (instr & 0xf0ff) == ADD_VALUE_TO_INDEX_REGISTER => {
                let register_x = ((instr & 0x0f00) >> 8) as usize;
                self.index_register += self.general_registers[register_x] as u16;
            }
            instr if (instr & 0xf0ff) == STORE_REGISTERS_TO_INDEX_ADDRESS => {
                let register_num = ((instr & 0x0f00) >> 8) as usize;
                for (index, &value) in self.general_registers[0..register_num].iter().enumerate() {
                    self.memory[self.index_register as usize + index] = value;
                }
            }
            instr if (instr & 0xf0ff) == LOAD_INDEX_ADDRESS_TO_REGISTERS => {
                let register_num = ((instr & 0x0f00) >> 8) as usize;
                for index in 0..register_num {
                    self.general_registers[index] =
                        self.memory[self.index_register as usize + index];
                }
            }
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
            instr if (instr & 0xf00f) == ADD_WITH_CARRY => {
                let register_x = ((instr & 0x0f00) >> 8) as usize;
                let register_y = ((instr & 0x00f0) >> 4) as usize;

                let x_value = self.general_registers[register_x];
                let y_value = self.general_registers[register_y];

                self.general_registers[register_x] = x_value + y_value;
                self.general_registers[self.general_registers.len() - 1] =
                    if x_value >= 255 - y_value { 1 } else { 0 };
            }
            instr if (instr & 0xf00f) == XOR_REGISTERS_X_AND_Y => {
                let register_x = ((instr & 0x0f00) >> 8) as usize;
                let register_y = ((instr & 0x00f0) >> 4) as usize;

                let y_value = self.general_registers[register_y];

                self.general_registers[register_x] ^= y_value;
            }
            instr if (instr & 0xf00f) == SKIP_NEXT_IF_REGISTERS_ARE_EQUAL => {
                let register_x = ((instr & 0x0f00) >> 8) as usize;
                let register_y = ((instr & 0x00f0) >> 4) as usize;

                let x_value = self.general_registers[register_x];
                let y_value = self.general_registers[register_y];

                if x_value == y_value {
                    self.program_counter += 2;
                }
            }
            instr if (instr & 0xf00f) == SKIP_NEXT_IF_REGISTERS_NOT_EQUAL => {
                let register_x = ((instr & 0x0f00) >> 8) as usize;
                let register_y = ((instr & 0x00f0) >> 4) as usize;

                let x_value = self.general_registers[register_x];
                let y_value = self.general_registers[register_y];

                if x_value != y_value {
                    self.program_counter += 2;
                }
            }
            instr if (instr & 0xf000) == SKIP_NEXT_IF_REGISTER_IS_EQUAL => {
                let register_x = ((instr & 0x0f00) >> 8) as usize;
                let value = (instr & 0x00ff) as u8;

                if self.general_registers[register_x] == value {
                    self.program_counter += 2;
                }
            }
            instr if (instr & 0xf00f) == SUBTRACT_WITH_BORROW => {
                let register_x = ((instr & 0x0f00) >> 8) as usize;
                let register_y = ((instr & 0x00f0) >> 4) as usize;

                let x_value = self.general_registers[register_x] as u16;
                let y_value = self.general_registers[register_y] as u16;

                let difference = x_value - y_value;

                self.general_registers[register_x] = (difference & 0xff) as u8;
                self.general_registers[self.general_registers.len() - 1] =
                    if x_value > y_value { 1 } else { 0 };
            }
            instr if (instr & 0xF000) == JUMP_TO_ADDRESS => {
                let address = (instr & 0xfff) as u16;
                self.program_counter = address;
            }
            _ => {
                return Err(Error::new(
                    ErrorKind::Other,
                    format!(
                        "Unrecognized instruction: {instruction:#06x} {0} ({0:#06x})",
                        self.program_counter - 2,
                    )
                    .to_owned(),
                ));
            }
        }

        Ok(())
    }

    pub fn tick(&mut self) -> Result<()> {
        let instruction = self.read_instruction();
        self.execute_instruction(instruction)?;
        Ok(())
    }
}
