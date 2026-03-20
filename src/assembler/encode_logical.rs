use crate::Logical;

pub fn encode_logical(instruction: &Logical) -> u16 {
    match *instruction {
        Logical::OrRegisterXWithRegisterY {
            x_register,
            y_register,
        } => 0x8001 | ((x_register as u16) << 8) | ((y_register as u16) << 4),
        Logical::AndRegisterXWithRegisterY {
            x_register,
            y_register,
        } => 0x8002 | ((x_register as u16) << 8) | ((y_register as u16) << 4),
        Logical::XorRegisterXWithRegisterY {
            x_register,
            y_register,
        } => 0x8003 | ((x_register as u16) << 8) | ((y_register as u16) << 4),
        Logical::ShiftRegisterXLeftWithRegisterY {
            x_register,
            y_register,
        } => 0x8006 | ((x_register as u16) << 8) | ((y_register as u16) << 4),
        Logical::ShiftRegisterXRightWithRegisterY {
            x_register,
            y_register,
        } => 0x8007 | ((x_register as u16) << 8) | ((y_register as u16) << 4),
    }
}
