use crate::{Chip8, core::Instruction};
use std::io::{Error, ErrorKind, Result};

impl Chip8 {
    pub fn decode_instruction(&self, opcode: u16) -> Result<Instruction> {
        // Constant or one-off patterns
        match opcode {
            0x00E0 => return Ok(Instruction::ClearScreen),
            0x00EE => return Ok(Instruction::ReturnFromSubroutine),
            code if (code & 0xF000) == 0xD000 => {
                return Ok(Instruction::DrawSpriteToScreen {
                    x_register: ((code & 0x0f00) >> 8) as u8,
                    y_register: ((code & 0x00f0) >> 4) as u8,
                    n_rows: (code & 0xf) as u8,
                });
            }
            _ => {}
        };

        if let Some(instruction) = self.takes_an_x_register(opcode) {
            return Ok(instruction);
        }
        if let Some(instruction) = self.takes_x_register_and_byte(opcode) {
            return Ok(instruction);
        }
        if let Some(instruction) = self.takes_an_address(opcode) {
            return Ok(instruction);
        }
        if let Some(instruction) = self.takes_x_and_y_register(opcode) {
            return Ok(instruction);
        }

        return Err(Error::new(
            ErrorKind::Other,
            format!(
                "Unrecognized instruction: {opcode:#06X} {0} ({0:#06X})",
                self.program_counter - 2,
            )
            .to_owned(),
        ));
    }

    fn takes_x_register_and_byte(&self, opcode: u16) -> Option<Instruction> {
        let x_register = ((opcode & 0x0F00) >> 8) as u8;
        let value = (opcode & 0xFF) as u8;

        let code = opcode & 0xF000;
        match code {
            0x3000 => Some(Instruction::SkipNextIfRegisterXEqualsImmediate { x_register, value }),
            0x4000 => {
                Some(Instruction::SkipNextIfRegisterXNotEqualsImmediate { x_register, value })
            }
            0x6000 => Some(Instruction::LoadImmediateToRegister { x_register, value }),
            0x7000 => Some(Instruction::AddImmediateToRegister { x_register, value }),
            _ => None,
        }
    }

    fn takes_an_address(&self, opcode: u16) -> Option<Instruction> {
        let address = opcode & 0xFFF;
        let code = opcode & 0xF000;
        match code {
            0x1000 => Some(Instruction::JumpToAddress { address }),
            0x2000 => Some(Instruction::CallSubroutine { address }),
            0xA000 => Some(Instruction::LoadImmediateToIndexRegister { address }),
            _ => None,
        }
    }

    fn takes_x_and_y_register(&self, opcode: u16) -> Option<Instruction> {
        let x_register = ((opcode & 0x0F00) >> 8) as u8;
        let y_register = ((opcode & 0x00F0) >> 4) as u8;

        let code = opcode & 0xF00F;
        match code {
            0x5000 => Some(Instruction::SkipNextIfRegisterXEqualsRegisterY {
                x_register,
                y_register,
            }),
            0x9000 => Some(Instruction::SkipNextIfRegisterYNotEqualRegisterX {
                x_register,
                y_register,
            }),
            0x8000 => Some(Instruction::LoadRegisterYToRegisterX {
                x_register,
                y_register,
            }),
            0x8001 => Some(Instruction::OrRegisterXWithRegisterY {
                x_register,
                y_register,
            }),
            0x8002 => Some(Instruction::AndRegisterXWithRegisterY {
                x_register,
                y_register,
            }),
            0x8003 => Some(Instruction::XorRegisterXWithRegisterY {
                x_register,
                y_register,
            }),
            0x8004 => Some(Instruction::AddRegisterYToRegisterX {
                x_register,
                y_register,
            }),
            0x8005 => Some(Instruction::SubtractRegisterYFromRegisterX {
                x_register,
                y_register,
            }),
            0x8006 => Some(Instruction::ShiftRegisterXRightWithRegisterY {
                x_register,
                y_register,
            }),
            0x8007 => Some(Instruction::SubtractNRegisterXFromRegisterY {
                x_register,
                y_register,
            }),
            0x800E => Some(Instruction::ShiftRegisterXLeftWithRegisterY {
                x_register,
                y_register,
            }),
            _ => None,
        }
    }

    fn takes_an_x_register(&self, opcode: u16) -> Option<Instruction> {
        let x_register = ((opcode & 0x0F00) >> 8) as u8;

        let code = opcode & 0xF0FF;
        match code {
            0xF01E => Some(Instruction::AddRegisterXToImmediate { x_register }),
            0xF033 => Some(Instruction::StoreBcdOfRegisterXAtIndex { x_register }),
            0xF055 => Some(Instruction::SaveNumRegistersToImediate {
                n_registers: x_register,
            }),
            0xF065 => Some(Instruction::SaveImmediateToNumRegisters {
                n_registers: x_register,
            }),
            _ => None,
        }
    }
}
