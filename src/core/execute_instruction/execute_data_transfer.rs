use crate::{Chip8, DataTransfer, DataTransfer::*};

impl Chip8 {
    pub(super) fn execute_data_transfer_instruction(&mut self, instruction: DataTransfer) {
        match instruction {
            LoadImmediateToRegister { x_register, value } => {
                self.general_registers[x_register as usize] = value;
            }
            LoadImmediateToIndexRegister { address } => {
                self.index_register = address;
            }
            LoadRegisterYToRegisterX {
                x_register,
                y_register,
            } => {
                self.general_registers[x_register as usize] =
                    self.general_registers[y_register as usize];
            }
            StoreBcdOfRegisterXAtIndex { x_register } => {
                let value = self.general_registers[x_register as usize];
                let hundreds = (value / 100) % 10;
                let tens = (value / 10) % 10;
                let ones = value % 10;

                self.memory[self.index_register as usize] = hundreds;
                self.memory[self.index_register as usize + 1] = tens;
                self.memory[self.index_register as usize + 2] = ones;
            }
            SaveNumRegistersToImediate { n_registers } => {
                for i in 0..n_registers + 1 {
                    let data = self.general_registers[i as usize];
                    self.memory[self.index_register as usize + i as usize] = data;
                }
            }
            SaveImmediateToNumRegisters { n_registers } => {
                for i in 0..n_registers + 1 {
                    let data = self.memory[self.index_register as usize + i as usize];
                    self.general_registers[i as usize] = data;
                }
            }
        }
    }
}
