use crate::ControlFlow;

pub fn encode_control_flow(instruction: &ControlFlow) -> u16 {
    match instruction {
        &ControlFlow::SkipNextIfRegisterXEqualsImmediate { x_register, value } => {
            (0x3000) | ((x_register as u16) << 8) | (value as u16)
        }
        &ControlFlow::SkipNextIfRegisterXNotEqualsImmediate { x_register, value } => {
            (0x3000) | ((x_register as u16) << 8) | (value as u16)
        }
        &ControlFlow::SkipNextIfRegisterXEqualsRegisterY {
            x_register,
            y_register,
        } => 0x5000 | ((x_register as u16) << 8) | ((y_register as u16) << 4),
        &ControlFlow::SkipNextIfRegisterYNotEqualRegisterX {
            x_register,
            y_register,
        } => 0x9000 | ((x_register as u16) << 8) | ((y_register as u16) << 4),
        &ControlFlow::JumpToAddress { address } => 0x1000 | address,
        &ControlFlow::CallSubroutine { address } => 0x2000 | address,
        ControlFlow::ReturnFromSubroutine => 0x00EE,
    }
}
