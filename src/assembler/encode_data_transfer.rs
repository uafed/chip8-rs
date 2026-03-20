use crate::DataTransfer;

pub fn encode_data_transfer(instruction: &DataTransfer) -> u16 {
    match *instruction {
        DataTransfer::LoadImmediateToIndexRegister { address } => 0xA000 | address,
        DataTransfer::LoadImmediateToRegister { x_register, value } => {
            0x6000 | ((x_register as u16) << 8) | (value as u16)
        }
        DataTransfer::LoadRegisterYToRegisterX {
            x_register,
            y_register,
        } => 0x8000 | ((x_register as u16) << 8) | ((y_register as u16) << 8),
        DataTransfer::StoreBcdOfRegisterXAtIndex { x_register } => {
            0xF033 | ((x_register as u16) << 8)
        }
        DataTransfer::SaveNumRegistersToImediate { n_registers } => {
            0xF055 | ((n_registers as u16) << 8)
        }
        DataTransfer::SaveImmediateToNumRegisters { n_registers } => {
            0xF065 | ((n_registers as u16) << 8)
        }
    }
}
