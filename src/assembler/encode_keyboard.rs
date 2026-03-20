use crate::Keyboard;

pub fn encode_keyboard(instruction: &Keyboard) -> u16 {
    match *instruction {
        Keyboard::SkipIfKeyInRegisterXIsPressed { x_register } => {
            0xE09E | ((x_register as u16) << 8)
        }
        Keyboard::SkipIfKeyInRegisterXIsNotPressed { x_register } => {
            0xE0A1 | ((x_register as u16) << 8)
        }
        Keyboard::WaitUntilKeyIsPressedPressed { x_register } => {
            0xF00A | ((x_register as u16) << 8)
        }
    }
}
