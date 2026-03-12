use crate::{Chip8, core::Instruction};
use std::io::Result;

impl Chip8 {
    pub fn execute_instruction(&mut self, instruction: Instruction) -> Result<()> {
        match instruction {
            Instruction::ClearScreen => {
                for row in self.frame_buffer.iter_mut() {
                    for pixel in row {
                        *pixel = 0x00;
                    }
                }
            }
            Instruction::LoadImmediateToRegister { register, value } => {
                self.general_registers[register as usize] = value;
            }
            Instruction::AddImmediateToRegister { register, value } => {
                self.general_registers[register as usize] += value;
            }
            Instruction::LoadImmediateToIndexRegister { value } => {
                self.index_register = value;
            }
            Instruction::DrawSpriteToScreen {
                x_register,
                y_register,
                n_rows,
            } => {
                let x_start = self.general_registers[x_register as usize];
                let y_start = self.general_registers[y_register as usize];
                for row in 0..n_rows {
                    let sprite_row = self.memory[(self.index_register as usize) + (row as usize)];
                    for bit_index in 0..8 {
                        let bit = (sprite_row & (1 << (7 - bit_index))) >> (7 - bit_index);
                        self.frame_buffer[y_start as usize + row as usize]
                            [x_start as usize + bit_index] ^= bit;
                    }
                }
            }
            Instruction::JumpToAddress { address } => {
                self.program_counter = address;
            }
        }
        Ok(())
    }
}
