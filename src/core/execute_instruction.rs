use crate::{Chip8, core::Instruction};

impl Chip8 {
    pub fn get_flag_register(&self) -> u8 {
        self.general_registers[self.general_registers.len() - 1]
    }
    pub fn execute_instruction(&mut self, instruction: Instruction) {
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
                let height = self.frame_buffer.len();
                let width = self.frame_buffer[0].len();
                for row in 0..n_rows {
                    let sprite_row = self.memory[(self.index_register as usize) + (row as usize)];
                    for bit_index in 0..8 {
                        let bit = (sprite_row & (1 << (7 - bit_index))) >> (7 - bit_index);

                        let curr_pixel = self.frame_buffer
                            [(y_start as usize + row as usize) % height]
                            [(x_start as usize + bit_index) % width];

                        if curr_pixel ^ bit == 0 && curr_pixel != 0 {
                            self.general_registers[self.general_registers.len() - 1] = 1;
                        }

                        self.frame_buffer[(y_start as usize + row as usize) % height]
                            [(x_start as usize + bit_index) % width] ^= bit;
                    }
                }
            }
            Instruction::JumpToAddress { address } => {
                self.program_counter = address;
            }
            Instruction::LoadRegisterYToRegisterX {
                x_register,
                y_register,
            } => {
                self.general_registers[x_register as usize] =
                    self.general_registers[y_register as usize];
            }
            Instruction::AddRegisterXToImmediate { x_register } => {
                self.index_register += self.general_registers[x_register as usize] as u16;
            }
            Instruction::XorRegisterXWithRegisterY {
                x_register,
                y_register,
            } => {
                self.general_registers[x_register as usize] ^=
                    self.general_registers[y_register as usize];
            }
            Instruction::SkipNextIfRegisterXEqualsRegisterY {
                x_register,
                y_register,
            } => {
                if self.general_registers[x_register as usize]
                    == self.general_registers[y_register as usize]
                {
                    self.program_counter += 2;
                }
            }
            Instruction::SkipNextIfRegisterYNotEqualRegisterX {
                x_register,
                y_register,
            } => {
                if self.general_registers[x_register as usize]
                    != self.general_registers[y_register as usize]
                {
                    self.program_counter += 2;
                }
            }
            Instruction::AddRegisterYToRegisterX {
                x_register,
                y_register,
            } => {
                let y_value = self.general_registers[y_register as usize];
                let x_value = self.general_registers[x_register as usize];

                if y_value > 255 - x_value {
                    self.general_registers[x_register as usize] = 0;
                    self.general_registers[self.general_registers.len() - 1] = 1;
                    return;
                }

                self.general_registers[x_register as usize] += y_value;
                self.general_registers[self.general_registers.len() - 1] = 0;
            }
            Instruction::SubtractRegisterYFromRegisterX {
                x_register,
                y_register,
            } => {
                let y_value = self.general_registers[y_register as usize];
                let x_value = self.general_registers[x_register as usize];

                if x_value > y_value {
                    self.general_registers[self.general_registers.len() - 1] = 1;
                    self.general_registers[x_register as usize] -= y_value;
                    return;
                }

                self.general_registers[self.general_registers.len() - 1] = 0;
                self.general_registers[x_register as usize] = 0;
            }
            Instruction::SaveNumRegistersToImediate { n_registers } => {
                for i in 0..n_registers + 1 {
                    let data = self.general_registers[i as usize];
                    self.memory[self.index_register as usize + i as usize] = data;
                }
            }
            Instruction::SaveImmediateToNumRegisters { n_registers } => {
                for i in 0..n_registers + 1 {
                    let data = self.memory[self.index_register as usize + i as usize];
                    self.general_registers[i as usize] = data;
                }
            }
            Instruction::SkipNextIfRegisterXEqualsImmediate { x_register, value } => {
                if self.general_registers[x_register as usize] == value {
                    self.program_counter += 2;
                }
            }
        }
    }
}
