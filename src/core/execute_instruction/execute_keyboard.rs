use crate::{Chip8, Keyboard, Keyboard::*};

impl Chip8 {
    pub(super) fn execute_keyboard_instruction(&mut self, instruction: Keyboard) {
        match instruction {
            SkipIfKeyInRegisterXIsPressed { x_register } => {
                let value = self.get_register_value(x_register);
                if self.key_states[value as usize] {
                    self.program_counter += 2;
                }
            }
            SkipIfKeyInRegisterXIsNotPressed { x_register } => {
                let value = self.get_register_value(x_register);
                if !self.key_states[value as usize] {
                    self.program_counter += 2;
                }
            }
        }
    }
}
