use std::fmt::{self};

use crate::{
    Arithmetic, ControlFlow, DataTransfer, Drawing, Instruction, Keyboard, Logical, Timer,
};

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Instruction::Arithmetic(inner) => inner.fmt(f)?,
            Instruction::ControlFlow(inner) => inner.fmt(f)?,
            Instruction::DataTransfer(inner) => inner.fmt(f)?,
            Instruction::Drawing(inner) => inner.fmt(f)?,
            Instruction::Logical(inner) => inner.fmt(f)?,
            Instruction::Timer(inner) => inner.fmt(f)?,
            Instruction::Keyboard(inner) => inner.fmt(f)?,
        }
        Ok(())
    }
}

impl fmt::Display for Drawing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Drawing::ClearScreen => write!(f, "CLEAR")?,
            Drawing::DrawSpriteToScreen {
                x_register,
                y_register,
                n_rows,
            } => write!(f, "DRAW V{}, V{}, {}", x_register, y_register, n_rows)?,
        }
        Ok(())
    }
}

impl fmt::Display for ControlFlow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            ControlFlow::CallSubroutine { address } => write!(f, "CALL {0:#06X}", address)?,
            ControlFlow::ReturnFromSubroutine => write!(f, "RET")?,
            ControlFlow::JumpToAddress { address } => write!(f, "JMP {:#06X}", address)?,
            ControlFlow::SkipNextIfRegisterXEqualsImmediate { x_register, value } => {
                write!(f, "SE V{x_register}, {value:#04X}")?
            }
            ControlFlow::SkipNextIfRegisterXNotEqualsImmediate { x_register, value } => {
                write!(f, "SNE V{x_register}, {value:#04X}")?
            }
            ControlFlow::SkipNextIfRegisterXEqualsRegisterY {
                x_register,
                y_register,
            } => write!(f, "SE V{x_register}, V{y_register}")?,
            ControlFlow::SkipNextIfRegisterYNotEqualRegisterX {
                x_register,
                y_register,
            } => write!(f, "SNE V{x_register}, V{y_register}")?,
        };
        Ok(())
    }
}

impl fmt::Display for Arithmetic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Arithmetic::AddImmediateToRegister { x_register, value } => {
                write!(f, "ADD V{0}, {1:#06X}", x_register, value)?
            }
            Arithmetic::AddRegisterXToIndex { x_register } => write!(f, "ADD I, V{x_register}")?,
            Arithmetic::AddRegisterYToRegisterX {
                x_register,
                y_register,
            } => write!(f, "ADD V{x_register}, V{y_register}")?,
            Arithmetic::SubtractRegisterYFromRegisterX {
                x_register,
                y_register,
            } => write!(f, "SUB V{x_register}, V{y_register}")?,
            Arithmetic::SubtractNRegisterXFromRegisterY {
                x_register,
                y_register,
            } => write!(f, "SUBN V{x_register}, V{y_register}")?,
        };
        Ok(())
    }
}

impl fmt::Display for DataTransfer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            DataTransfer::LoadImmediateToRegister { x_register, value } => {
                write!(f, "LD V{0}, {1:#06X}", x_register, value)?
            }
            DataTransfer::LoadImmediateToIndexRegister { address } => {
                write!(f, "LD I, {0:#06X}", address)?
            }
            DataTransfer::LoadRegisterYToRegisterX {
                x_register,
                y_register,
            } => write!(f, "LD V{}, V{}", x_register, y_register)?,
            DataTransfer::StoreBcdOfRegisterXAtIndex { x_register } => {
                write!(f, "BCD [I], V{x_register}")?
            }
            DataTransfer::SaveNumRegistersToImediate { n_registers } => {
                write!(f, "LD [I], V{n_registers}")?
            }
            DataTransfer::SaveImmediateToNumRegisters { n_registers } => {
                write!(f, "LD V{n_registers}, [I]")?
            }
        };
        Ok(())
    }
}

impl fmt::Display for Logical {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Logical::OrRegisterXWithRegisterY {
                x_register,
                y_register,
            } => write!(f, "OR V{x_register}, V{y_register}")?,
            Logical::AndRegisterXWithRegisterY {
                x_register,
                y_register,
            } => write!(f, "AND V{x_register}, V{y_register}")?,
            Logical::XorRegisterXWithRegisterY {
                x_register,
                y_register,
            } => write!(f, "XOR V{x_register}, V{y_register}")?,
            Logical::ShiftRegisterXRightWithRegisterY {
                x_register,
                y_register,
            } => write!(f, "SHR V{x_register}, V{y_register}")?,
            Logical::ShiftRegisterXLeftWithRegisterY {
                x_register,
                y_register,
            } => write!(f, "SHL V{x_register}, V{y_register}")?,
        };
        Ok(())
    }
}

impl fmt::Display for Timer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Timer::LoadDelayTimerToRegisterX { x_register } => write!(f, "LD V{x_register}, DT")?,
            Timer::LoadRegisterXToDelayTimer { x_register } => write!(f, "LD DT, V{x_register}")?,
            Timer::LoadRegisterXToSoundTimer { x_register } => write!(f, "LD ST, V{x_register}")?,
        }
        Ok(())
    }
}

impl fmt::Display for Keyboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Keyboard::SkipIfKeyInRegisterXIsPressed { x_register } => {
                write!(f, "SKP V{x_register}")?
            }
            Keyboard::SkipIfKeyInRegisterXIsNotPressed { x_register } => {
                write!(f, "SNKP V{x_register}")?
            }
            Keyboard::WaitUntilKeyIsPressedPressed { x_register } => {
                write!(f, "LD V{x_register}, K")?
            }
        }
        Ok(())
    }
}
