use std::fmt;

use crate::Instruction;

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Instruction::ClearScreen => write!(f, "CLEAR")?,
            Instruction::CallSubroutine { address } => write!(f, "CALL {0:#06X}", address)?,
            Instruction::ReturnFromSubroutine => write!(f, "RET")?,
            Instruction::LoadImmediateToRegister { x_register: register, value } => {
                write!(f, "LD V{0}, {1:#06X}", register, value)?
            }
            Instruction::AddImmediateToRegister { x_register: register, value } => {
                write!(f, "ADD V{0}, {1:#06X}", register, value)?
            }
            Instruction::LoadImmediateToIndexRegister { value } => {
                write!(f, "LD I, {0:#06X}", value)?
            }
            Instruction::DrawSpriteToScreen {
                x_register,
                y_register,
                n_rows,
            } => write!(f, "DRAW V{}, V{}, {}", x_register, y_register, n_rows)?,
            Instruction::JumpToAddress { address } => write!(f, "JMP {:#06X}", address)?,
            Instruction::LoadRegisterYToRegisterX {
                x_register,
                y_register,
            } => write!(f, "ADD V{}, V{}", x_register, y_register)?,
            Instruction::AddRegisterXToImmediate { x_register } => {
                write!(f, "ADD I, V{x_register}")?
            }
            Instruction::SkipNextIfRegisterXEqualsImmediate { x_register, value } => {
                write!(f, "SE V{x_register}, {value:#04X}")?
            }
            Instruction::SkipNextIfRegisterXNotEqualsImmediate { x_register, value } => {
                write!(f, "SNE V{x_register}, {value:#04X}")?
            }
            Instruction::SkipNextIfRegisterXEqualsRegisterY {
                x_register,
                y_register,
            } => write!(f, "SE V{x_register}, V{y_register}")?,
            Instruction::SkipNextIfRegisterYNotEqualRegisterX {
                x_register,
                y_register,
            } => write!(f, "SNE V{x_register}, V{y_register}")?,
            Instruction::OrRegisterXWithRegisterY {
                x_register,
                y_register,
            } => write!(f, "OR V{x_register}, V{y_register}")?,
            Instruction::AndRegisterXWithRegisterY {
                x_register,
                y_register,
            } => write!(f, "AND V{x_register}, V{y_register}")?,
            Instruction::XorRegisterXWithRegisterY {
                x_register,
                y_register,
            } => write!(f, "XOR V{x_register}, V{y_register}")?,
            Instruction::AddRegisterYToRegisterX {
                x_register,
                y_register,
            } => write!(f, "ADD V{x_register}, V{y_register}")?,
            Instruction::SubtractRegisterYFromRegisterX {
                x_register,
                y_register,
            } => write!(f, "SUB V{x_register}, V{y_register}")?,
            Instruction::ShiftRegisterXRightWithRegisterY {
                x_register,
                y_register,
            } => write!(f, "SHR V{x_register}, V{y_register}")?,
            Instruction::ShiftRegisterXLeftWithRegisterY {
                x_register,
                y_register,
            } => write!(f, "SHL V{x_register}, V{y_register}")?,
            Instruction::SubtractNRegisterXFromRegisterY {
                x_register,
                y_register,
            } => write!(f, "SUBN V{x_register}, V{y_register}")?,
            Instruction::StoreBcdOfRegisterXAtIndex { x_register } => {
                write!(f, "BCD [I], V{x_register}")?
            }
            Instruction::SaveNumRegistersToImediate { n_registers } => {
                write!(f, "LD [I], V{n_registers}")?
            }
            Instruction::SaveImmediateToNumRegisters { n_registers } => {
                write!(f, "LD V{n_registers}, [I]")?
            }
        };
        Ok(())
    }
}
