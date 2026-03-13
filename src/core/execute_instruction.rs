use crate::{Arithmetic, Chip8, ControlFlow, DataTransfer, Drawing, Logical, core::Instruction};

impl Chip8 {
    pub fn get_flag_register(&self) -> u8 {
        self.general_registers[self.general_registers.len() - 1]
    }

    pub fn execute_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Arithmetic(instruction) => {
                self.execute_arithmetic_instruction(instruction);
            }
            Instruction::ControlFlow(instruction) => {
                self.execute_control_flow_instruction(instruction);
            }
            Instruction::DataTransfer(instruction) => {
                self.execute_data_transfer_instruction(instruction);
            }
            Instruction::Drawing(instruction) => {
                self.execute_drawing_instruction(instruction);
            }
            Instruction::Logical(instruction) => {
                self.execute_logical_instruction(instruction);
            }
        }
    }

    fn execute_arithmetic_instruction(&mut self, instruction: Arithmetic) {
        match instruction {
            Arithmetic::AddImmediateToRegister { x_register, value } => {
                let current_value = self.general_registers[x_register as usize];
                if value > 255 - current_value {
                    let wrapped = value - (255 - current_value + 1);
                    self.general_registers[x_register as usize] = wrapped;
                    return;
                }
                self.general_registers[x_register as usize] += value;
            }
            Arithmetic::AddRegisterXToImmediate { x_register } => {
                self.index_register += self.general_registers[x_register as usize] as u16;
            }
            Arithmetic::AddRegisterYToRegisterX {
                x_register,
                y_register,
            } => {
                let y_value = self.general_registers[y_register as usize];
                let x_value = self.general_registers[x_register as usize];

                if y_value > 255 - x_value {
                    let wrapped = y_value - (255 - x_value + 1);
                    self.general_registers[x_register as usize] = wrapped;
                    self.general_registers[self.general_registers.len() - 1] = 1;
                    return;
                }

                self.general_registers[x_register as usize] += y_value;
                self.general_registers[self.general_registers.len() - 1] = 0;
            }
            Arithmetic::SubtractRegisterYFromRegisterX {
                x_register,
                y_register,
            } => {
                let y_value = self.general_registers[y_register as usize];
                let x_value = self.general_registers[x_register as usize];

                if y_value > x_value {
                    // underflow
                    let wrapped = 255 - (y_value - x_value - 1);
                    self.general_registers[x_register as usize] = wrapped;
                    return;
                }
                self.general_registers[x_register as usize] = x_value - y_value;

                self.general_registers[self.general_registers.len() - 1] =
                    if x_value > y_value { 1 } else { 0 };
            }
            Arithmetic::SubtractNRegisterXFromRegisterY {
                x_register,
                y_register,
            } => {
                let y_value = self.general_registers[y_register as usize];
                let x_value = self.general_registers[x_register as usize];

                self.general_registers[self.general_registers.len() - 1] =
                    if y_value > x_value { 1 } else { 0 };

                if y_value < x_value {
                    let wrapped = 255 - (x_value - y_value - 1);
                    self.general_registers[x_register as usize] = wrapped;
                    return;
                }

                self.general_registers[x_register as usize] = y_value - x_value;
            }
        }
    }
    fn execute_control_flow_instruction(&mut self, instruction: ControlFlow) {
        match instruction {
            ControlFlow::CallSubroutine { address } => {
                self.stack_pointer += 1;
                self.stack_memory[self.stack_pointer as usize - 1] = self.program_counter;
                self.program_counter = address;
            }
            ControlFlow::ReturnFromSubroutine => {
                self.stack_pointer -= 1;
                self.program_counter = self.stack_memory[self.stack_pointer as usize];
            }
            ControlFlow::JumpToAddress { address } => {
                self.program_counter = address;
            }
            ControlFlow::SkipNextIfRegisterXEqualsRegisterY {
                x_register,
                y_register,
            } => {
                if self.general_registers[x_register as usize]
                    == self.general_registers[y_register as usize]
                {
                    self.program_counter += 2;
                }
            }
            ControlFlow::SkipNextIfRegisterYNotEqualRegisterX {
                x_register,
                y_register,
            } => {
                if self.general_registers[x_register as usize]
                    != self.general_registers[y_register as usize]
                {
                    self.program_counter += 2;
                }
            }
            ControlFlow::SkipNextIfRegisterXEqualsImmediate { x_register, value } => {
                if self.general_registers[x_register as usize] == value {
                    self.program_counter += 2;
                }
            }
            ControlFlow::SkipNextIfRegisterXNotEqualsImmediate { x_register, value } => {
                if self.general_registers[x_register as usize] != value {
                    self.program_counter += 2;
                }
            }
        }
    }
    fn execute_data_transfer_instruction(&mut self, instruction: DataTransfer) {
        match instruction {
            DataTransfer::LoadImmediateToRegister { x_register, value } => {
                self.general_registers[x_register as usize] = value;
            }
            DataTransfer::LoadImmediateToIndexRegister { address } => {
                self.index_register = address;
            }
            DataTransfer::LoadRegisterYToRegisterX {
                x_register,
                y_register,
            } => {
                self.general_registers[x_register as usize] =
                    self.general_registers[y_register as usize];
            }
            DataTransfer::StoreBcdOfRegisterXAtIndex { x_register } => {
                let value = self.general_registers[x_register as usize];
                let hundreds = (value / 100) % 10;
                let tens = (value / 10) % 10;
                let ones = value % 10;

                self.memory[self.index_register as usize] = hundreds;
                self.memory[self.index_register as usize + 1] = tens;
                self.memory[self.index_register as usize + 2] = ones;
            }
            DataTransfer::SaveNumRegistersToImediate { n_registers } => {
                for i in 0..n_registers + 1 {
                    let data = self.general_registers[i as usize];
                    self.memory[self.index_register as usize + i as usize] = data;
                }
            }
            DataTransfer::SaveImmediateToNumRegisters { n_registers } => {
                for i in 0..n_registers + 1 {
                    let data = self.memory[self.index_register as usize + i as usize];
                    self.general_registers[i as usize] = data;
                }
            }
        }
    }

    fn execute_drawing_instruction(&mut self, instruction: Drawing) {
        match instruction {
            Drawing::ClearScreen => {
                for row in self.frame_buffer.iter_mut() {
                    for pixel in row {
                        *pixel = 0x00;
                    }
                }
            }
            Drawing::DrawSpriteToScreen {
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
        }
    }
    fn execute_logical_instruction(&mut self, instruction: Logical) {
        match instruction {
            Logical::OrRegisterXWithRegisterY {
                x_register,
                y_register,
            } => {
                self.general_registers[x_register as usize] |=
                    self.general_registers[y_register as usize];
            }
            Logical::AndRegisterXWithRegisterY {
                x_register,
                y_register,
            } => {
                self.general_registers[x_register as usize] &=
                    self.general_registers[y_register as usize];
            }
            Logical::XorRegisterXWithRegisterY {
                x_register,
                y_register,
            } => {
                self.general_registers[x_register as usize] ^=
                    self.general_registers[y_register as usize];
            }
            Logical::ShiftRegisterXRightWithRegisterY {
                x_register,
                y_register,
            } => {
                let y_value = self.general_registers[y_register as usize];
                let lowest_bit = y_value & 1;

                self.general_registers[self.general_registers.len() - 1] = lowest_bit;
                self.general_registers[x_register as usize] = y_value >> 1;
            }
            Logical::ShiftRegisterXLeftWithRegisterY {
                x_register,
                y_register,
            } => {
                let y_value = self.general_registers[y_register as usize];
                let lowest_bit = (y_value & 80) >> 7;

                self.general_registers[self.general_registers.len() - 1] = lowest_bit;
                self.general_registers[x_register as usize] = y_value << 1;
            }
        }
    }
}
