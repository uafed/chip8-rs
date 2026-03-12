use std::fmt;

use crate::Instruction;

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Instruction::ClearScreen => write!(f, "clear")?,
            Instruction::LoadImmediateToRegister { register, value } => {
                write!(f, "V{0} := {1} ({1:#06X})", register, value)?
            }
            Instruction::AddImmediateToRegister { register, value } => {
                write!(f, "V{0} += {1} ({1:#06X})", register, value)?
            }
            Instruction::LoadImmediateToIndexRegister { value } => {
                write!(f, "VI := {0} ({0:#06X})", value)?
            }
            Instruction::DrawSpriteToScreen {
                x_register,
                y_register,
                n_rows,
            } => write!(f, "draw (V{}, V{}) n={}", x_register, y_register, n_rows)?,
            Instruction::JumpToAddress { address } => write!(f, "jmp {:#06X}", address)?,
        };
        Ok(())
    }
}
