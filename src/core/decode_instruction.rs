use crate::{Chip8, core::Instruction};
use std::io::{Error, ErrorKind, Result};

impl Chip8 {
    pub fn decode_instruction(&self, opcode: u16) -> Result<Instruction> {
        match opcode {
            0x00e0 => Ok(Instruction::ClearScreen),
            code if (code & 0xF000) == 0x6000 => Ok(Instruction::LoadImmediateToRegister {
                register: ((code & 0x0f00) >> 8) as u8,
                value: (code & 0xff) as u8,
            }),
            code if (code & 0xF000) == 0x7000 => Ok(Instruction::AddImmediateToRegister {
                register: ((code & 0x0f00) >> 8) as u8,
                value: (code & 0xff) as u8,
            }),
            code if (code & 0xF000) == 0xA000 => Ok(Instruction::LoadImmediateToIndexRegister {
                value: (code & 0xfff),
            }),
            code if (code & 0xF000) == 0xD000 => Ok(Instruction::DrawSpriteToScreen {
                x_register: ((code & 0x0f00) >> 8) as u8,
                y_register: ((code & 0x00f0) >> 4) as u8,
                n_rows: (code & 0xf) as u8,
            }),
            code if (code & 0xF000) == 0x1000 => Ok(Instruction::JumpToAddress {
                address: (code & 0xfff),
            }),
            _ => {
                return Err(Error::new(
                    ErrorKind::Other,
                    format!(
                        "Unrecognized instruction: {opcode:#06X} {0} ({0:#06X})",
                        self.program_counter - 2,
                    )
                    .to_owned(),
                ));
            }
        }
    }
}
