use crate::{Arithmetic, Arithmetic::*, Chip8};

impl Chip8 {
    pub(super) fn execute_arithmetic_instruction(&mut self, instruction: Arithmetic) {
        match instruction {
            AddImmediateToRegister { x_register, value } => {
                let current_value = self.general_registers[x_register as usize];
                if value > 255 - current_value {
                    let wrapped = value - (255 - current_value + 1);
                    self.general_registers[x_register as usize] = wrapped;
                    return;
                }
                self.general_registers[x_register as usize] += value;
            }
            AddRegisterXToImmediate { x_register } => {
                self.index_register += self.general_registers[x_register as usize] as u16;
            }
            AddRegisterYToRegisterX {
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
            SubtractRegisterYFromRegisterX {
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
            SubtractNRegisterXFromRegisterY {
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
}
