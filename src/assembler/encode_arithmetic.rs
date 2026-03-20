use crate::Arithmetic;

pub fn encode_arithmetic(instruction: &Arithmetic) -> u16 {
    match *instruction {
        Arithmetic::AddImmediateToRegister { x_register, value } => {
            0x7000 | ((x_register as u16) << 8) | (value as u16)
        }
        Arithmetic::AddRegisterXToIndex { x_register } => 0xF01E | ((x_register as u16) << 8),
        Arithmetic::AddRegisterYToRegisterX {
            x_register,
            y_register,
        } => 0x8004 | ((x_register as u16) << 8) | ((y_register as u16) << 4),
        Arithmetic::SubtractRegisterYFromRegisterX {
            x_register,
            y_register,
        } => 0x8005 | ((x_register as u16) << 8) | ((y_register as u16) << 4),
        Arithmetic::SubtractNRegisterXFromRegisterY {
            x_register,
            y_register,
        } => 0x8007 | ((x_register as u16) << 8) | ((y_register as u16) << 4),
    }
}
