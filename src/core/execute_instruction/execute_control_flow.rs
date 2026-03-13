use crate::{Chip8, ControlFlow, ControlFlow::*};

impl Chip8 {
    pub(super) fn execute_control_flow_instruction(&mut self, instruction: ControlFlow) {
        match instruction {
            CallSubroutine { address } => {
                self.stack_pointer += 1;
                self.stack_memory[self.stack_pointer as usize - 1] = self.program_counter;
                self.program_counter = address;
            }
            ReturnFromSubroutine => {
                self.stack_pointer -= 1;
                self.program_counter = self.stack_memory[self.stack_pointer as usize];
            }
            JumpToAddress { address } => {
                self.program_counter = address;
            }
            SkipNextIfRegisterXEqualsRegisterY {
                x_register,
                y_register,
            } => {
                if self.general_registers[x_register as usize]
                    == self.general_registers[y_register as usize]
                {
                    self.program_counter += 2;
                }
            }
            SkipNextIfRegisterYNotEqualRegisterX {
                x_register,
                y_register,
            } => {
                if self.general_registers[x_register as usize]
                    != self.general_registers[y_register as usize]
                {
                    self.program_counter += 2;
                }
            }
            SkipNextIfRegisterXEqualsImmediate { x_register, value } => {
                if self.general_registers[x_register as usize] == value {
                    self.program_counter += 2;
                }
            }
            SkipNextIfRegisterXNotEqualsImmediate { x_register, value } => {
                if self.general_registers[x_register as usize] != value {
                    self.program_counter += 2;
                }
            }
        }
    }
}
