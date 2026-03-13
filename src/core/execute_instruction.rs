use crate::{Chip8, core::Instruction};

mod execute_arithmetic;
mod execute_control_flow;
mod execute_data_transfer;
mod execute_drawing;
mod execute_logical;

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
}
