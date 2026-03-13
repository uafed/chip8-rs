use crate::{Chip8, Logical, Logical::*};

impl Chip8 {
    pub(super) fn execute_logical_instruction(&mut self, instruction: Logical) {
        match instruction {
            OrRegisterXWithRegisterY {
                x_register,
                y_register,
            } => {
                self.general_registers[x_register as usize] |=
                    self.general_registers[y_register as usize];
            }
            AndRegisterXWithRegisterY {
                x_register,
                y_register,
            } => {
                self.general_registers[x_register as usize] &=
                    self.general_registers[y_register as usize];
            }
            XorRegisterXWithRegisterY {
                x_register,
                y_register,
            } => {
                self.general_registers[x_register as usize] ^=
                    self.general_registers[y_register as usize];
            }
            ShiftRegisterXRightWithRegisterY {
                x_register,
                y_register,
            } => {
                let y_value = self.general_registers[y_register as usize];
                let lowest_bit = y_value & 1;

                self.general_registers[x_register as usize] = y_value >> 1;
                self.set_flag_register(lowest_bit > 0);
            }
            ShiftRegisterXLeftWithRegisterY {
                x_register,
                y_register,
            } => {
                let y_value = self.general_registers[y_register as usize];
                let highest_bit = (y_value & 0x80) >> 7;

                self.general_registers[x_register as usize] = y_value << 1;
                self.set_flag_register(highest_bit > 0);
            }
        }
    }
}
