use crate::{
    Arithmetic, Chip8, ControlFlow, DataTransfer, Drawing, Keyboard, Logical, Timer,
    core::Instruction,
};
use std::io::{Error, ErrorKind, Result};

impl Chip8 {
    pub fn decode_instruction(&self, opcode: u16) -> Result<Instruction> {
        // Constant or one-off patterns
        match opcode {
            0x00E0 => return Ok(Instruction::Drawing(Drawing::ClearScreen)),
            0x00EE => return Ok(Instruction::ControlFlow(ControlFlow::ReturnFromSubroutine)),
            code if (code & 0xF000) == 0xD000 => {
                return Ok(Instruction::Drawing(Drawing::DrawSpriteToScreen {
                    x_register: ((code & 0x0f00) >> 8) as u8,
                    y_register: ((code & 0x00f0) >> 4) as u8,
                    n_rows: (code & 0xf) as u8,
                }));
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
            0x3000 => Some(Instruction::ControlFlow(
                ControlFlow::SkipNextIfRegisterXEqualsImmediate { x_register, value },
            )),
            0x4000 => Some(Instruction::ControlFlow(
                ControlFlow::SkipNextIfRegisterXNotEqualsImmediate { x_register, value },
            )),
            0x6000 => Some(Instruction::DataTransfer(
                DataTransfer::LoadImmediateToRegister { x_register, value },
            )),
            0x7000 => Some(Instruction::Arithmetic(
                Arithmetic::AddImmediateToRegister { x_register, value },
            )),
            _ => None,
        }
    }

    fn takes_an_address(&self, opcode: u16) -> Option<Instruction> {
        let address = opcode & 0xFFF;
        let code = opcode & 0xF000;
        match code {
            0x1000 => Some(Instruction::ControlFlow(ControlFlow::JumpToAddress {
                address,
            })),
            0x2000 => Some(Instruction::ControlFlow(ControlFlow::CallSubroutine {
                address,
            })),
            0xA000 => Some(Instruction::DataTransfer(
                DataTransfer::LoadImmediateToIndexRegister { address },
            )),
            _ => None,
        }
    }

    fn takes_x_and_y_register(&self, opcode: u16) -> Option<Instruction> {
        let x_register = ((opcode & 0x0F00) >> 8) as u8;
        let y_register = ((opcode & 0x00F0) >> 4) as u8;

        let code = opcode & 0xF00F;
        match code {
            0x5000 => Some(Instruction::ControlFlow(
                ControlFlow::SkipNextIfRegisterXEqualsRegisterY {
                    x_register,
                    y_register,
                },
            )),
            0x9000 => Some(Instruction::ControlFlow(
                ControlFlow::SkipNextIfRegisterYNotEqualRegisterX {
                    x_register,
                    y_register,
                },
            )),
            0x8000 => Some(Instruction::DataTransfer(
                DataTransfer::LoadRegisterYToRegisterX {
                    x_register,
                    y_register,
                },
            )),
            0x8001 => Some(Instruction::Logical(Logical::OrRegisterXWithRegisterY {
                x_register,
                y_register,
            })),
            0x8002 => Some(Instruction::Logical(Logical::AndRegisterXWithRegisterY {
                x_register,
                y_register,
            })),
            0x8003 => Some(Instruction::Logical(Logical::XorRegisterXWithRegisterY {
                x_register,
                y_register,
            })),
            0x8004 => Some(Instruction::Arithmetic(
                Arithmetic::AddRegisterYToRegisterX {
                    x_register,
                    y_register,
                },
            )),
            0x8005 => Some(Instruction::Arithmetic(
                Arithmetic::SubtractRegisterYFromRegisterX {
                    x_register,
                    y_register,
                },
            )),
            0x8006 => Some(Instruction::Logical(
                Logical::ShiftRegisterXRightWithRegisterY {
                    x_register,
                    y_register,
                },
            )),
            0x8007 => Some(Instruction::Arithmetic(
                Arithmetic::SubtractNRegisterXFromRegisterY {
                    x_register,
                    y_register,
                },
            )),
            0x800E => Some(Instruction::Logical(
                Logical::ShiftRegisterXLeftWithRegisterY {
                    x_register,
                    y_register,
                },
            )),
            _ => None,
        }
    }

    fn takes_an_x_register(&self, opcode: u16) -> Option<Instruction> {
        let x_register = ((opcode & 0x0F00) >> 8) as u8;

        let code = opcode & 0xF0FF;
        match code {
            0xE09E => Some(Instruction::Keyboard(
                Keyboard::SkipIfKeyInRegisterXIsPressed { x_register },
            )),
            0xE0A1 => Some(Instruction::Keyboard(
                Keyboard::SkipIfKeyInRegisterXIsNotPressed { x_register },
            )),
            0xF007 => Some(Instruction::Timer(Timer::LoadDelayTimerToRegisterX {
                x_register,
            })),
            0xF00A => Some(Instruction::Keyboard(
                Keyboard::WaitUntilKeyIsPressedPressed { x_register },
            )),
            0xF015 => Some(Instruction::Timer(Timer::LoadRegisterXToDelayTimer {
                x_register,
            })),
            0xF018 => Some(Instruction::Timer(Timer::LoadRegisterXToSoundTimer {
                x_register,
            })),
            0xF01E => Some(Instruction::Arithmetic(
                Arithmetic::AddRegisterXToImmediate { x_register },
            )),
            0xF033 => Some(Instruction::DataTransfer(
                DataTransfer::StoreBcdOfRegisterXAtIndex { x_register },
            )),
            0xF055 => Some(Instruction::DataTransfer(
                DataTransfer::SaveNumRegistersToImediate {
                    n_registers: x_register,
                },
            )),
            0xF065 => Some(Instruction::DataTransfer(
                DataTransfer::SaveImmediateToNumRegisters {
                    n_registers: x_register,
                },
            )),
            _ => None,
        }
    }
}
