use crate::{Arithmetic, Arithmetic::*, Chip8};

impl Chip8 {
    pub(super) fn execute_arithmetic_instruction(&mut self, instruction: Arithmetic) {
        match instruction {
            AddImmediateToRegister { x_register, value } => {
                let current_value = self.general_registers[x_register as usize];
                self.general_registers[x_register as usize] = current_value.wrapping_add(value);
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

                self.set_flag_register(y_value > 255 - x_value);
                self.general_registers[x_register as usize] = x_value.wrapping_add(y_value);
            }
            SubtractRegisterYFromRegisterX {
                x_register,
                y_register,
            } => {
                let y_value = self.general_registers[y_register as usize];
                let x_value = self.general_registers[x_register as usize];

                self.set_flag_register(x_value > y_value);
                self.general_registers[x_register as usize] = x_value.wrapping_sub(y_value);
            }
            SubtractNRegisterXFromRegisterY {
                x_register,
                y_register,
            } => {
                let y_value = self.general_registers[y_register as usize];
                let x_value = self.general_registers[x_register as usize];

                self.set_flag_register(y_value > x_value);
                self.general_registers[x_register as usize] = y_value.wrapping_sub(x_value);
            }
        }
    }
}
