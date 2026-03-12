use crate::{Chip8, core::Instruction};
use std::io::{Error, ErrorKind, Result};

impl Chip8 {
    pub fn decode_instruction(&self, opcode: u16) -> Result<Instruction> {
        match opcode {
            0x00E0 => Ok(Instruction::ClearScreen),
            code if (code & 0xF000) == 0x2000 => Ok(Instruction::CallSubroutine {
                address: (code & 0xfff),
            }),
            0x00EE => Ok(Instruction::ReturnFromSubroutine),
            code if (code & 0xF000) == 0x3000 => {
                Ok(Instruction::SkipNextIfRegisterXEqualsImmediate {
                    x_register: ((code & 0x0f00) >> 8) as u8,
                    value: (code & 0xff) as u8,
                })
            }
            code if (code & 0xF000) == 0x4000 => {
                Ok(Instruction::SkipNextIfRegisterXNotEqualsImmediate {
                    x_register: ((code & 0x0f00) >> 8) as u8,
                    value: (code & 0xff) as u8,
                })
            }
            code if (code & 0xF000) == 0x6000 => Ok(Instruction::LoadImmediateToRegister {
                register: ((code & 0x0f00) >> 8) as u8,
                value: (code & 0xff) as u8,
            }),
            code if (code & 0xF000) == 0x7000 => Ok(Instruction::AddImmediateToRegister {
                register: ((code & 0x0f00) >> 8) as u8,
                value: (code & 0xff) as u8,
            }),
            code if (code & 0xF00F) == 0x9000 => {
                Ok(Instruction::SkipNextIfRegisterYNotEqualRegisterX {
                    x_register: ((code & 0x0f00) >> 8) as u8,
                    y_register: ((code & 0x00f0) >> 4) as u8,
                })
            }
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
            code if (code & 0xF00F) == 0x5000 => {
                Ok(Instruction::SkipNextIfRegisterXEqualsRegisterY {
                    x_register: ((code & 0x0f00) >> 8) as u8,
                    y_register: ((code & 0x00f0) >> 4) as u8,
                })
            }
            code if (code & 0xF00F) == 0x8000 => Ok(Instruction::LoadRegisterYToRegisterX {
                x_register: ((code & 0x0f00) >> 8) as u8,
                y_register: ((code & 0x00f0) >> 4) as u8,
            }),
            code if (code & 0xF00F) == 0x8001 => Ok(Instruction::OrRegisterXWithRegisterY {
                x_register: ((code & 0x0f00) >> 8) as u8,
                y_register: ((code & 0x00f0) >> 4) as u8,
            }),
            code if (code & 0xF00F) == 0x8002 => Ok(Instruction::AndRegisterXWithRegisterY {
                x_register: ((code & 0x0f00) >> 8) as u8,
                y_register: ((code & 0x00f0) >> 4) as u8,
            }),
            code if (code & 0xF00F) == 0x8003 => Ok(Instruction::XorRegisterXWithRegisterY {
                x_register: ((code & 0x0f00) >> 8) as u8,
                y_register: ((code & 0x00f0) >> 4) as u8,
            }),
            code if (code & 0xF00F) == 0x8004 => Ok(Instruction::AddRegisterYToRegisterX {
                x_register: ((code & 0x0f00) >> 8) as u8,
                y_register: ((code & 0x00f0) >> 4) as u8,
            }),
            code if (code & 0xF00F) == 0x8005 => Ok(Instruction::SubtractRegisterYFromRegisterX {
                x_register: ((code & 0x0f00) >> 8) as u8,
                y_register: ((code & 0x00f0) >> 4) as u8,
            }),
            code if (code & 0xF00F) == 0x8006 => {
                Ok(Instruction::ShiftRegisterXRightWithRegisterY {
                    x_register: ((code & 0x0f00) >> 8) as u8,
                    y_register: ((code & 0x00f0) >> 4) as u8,
                })
            }
            code if (code & 0xF00F) == 0x800E => Ok(Instruction::ShiftRegisterXLeftWithRegisterY {
                x_register: ((code & 0x0f00) >> 8) as u8,
                y_register: ((code & 0x00f0) >> 4) as u8,
            }),
            code if (code & 0xF00F) == 0x8007 => Ok(Instruction::SubtractNRegisterXFromRegisterY {
                x_register: ((code & 0x0f00) >> 8) as u8,
                y_register: ((code & 0x00f0) >> 4) as u8,
            }),
            code if (code & 0xF0FF) == 0xF01E => Ok(Instruction::AddRegisterXToImmediate {
                x_register: ((code & 0x0f00) >> 8) as u8,
            }),
            code if (code & 0xF0FF) == 0xF033 => Ok(Instruction::StoreBcdOfRegisterXAtIndex {
                x_register: ((code & 0x0f00) >> 8) as u8,
            }),
            code if (code & 0xF0FF) == 0xF055 => Ok(Instruction::SaveNumRegistersToImediate {
                n_registers: ((code & 0x0f00) >> 8) as u8,
            }),
            code if (code & 0xF0FF) == 0xF065 => Ok(Instruction::SaveImmediateToNumRegisters {
                n_registers: ((code & 0x0f00) >> 8) as u8,
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
