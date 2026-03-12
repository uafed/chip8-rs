use std::{
    fs::File,
    io::{Read, Result},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Instruction {
    ClearScreen,
    LoadImmediateToRegister {
        x_register: u8,
        value: u8,
    },
    CallSubroutine {
        address: u16,
    },
    ReturnFromSubroutine,
    SkipNextIfRegisterXEqualsImmediate {
        x_register: u8,
        value: u8,
    },
    SkipNextIfRegisterXNotEqualsImmediate {
        x_register: u8,
        value: u8,
    },
    SkipNextIfRegisterYNotEqualRegisterX {
        x_register: u8,
        y_register: u8,
    },
    AddImmediateToRegister {
        x_register: u8,
        value: u8,
    },
    LoadImmediateToIndexRegister {
        address: u16,
    },
    DrawSpriteToScreen {
        x_register: u8,
        y_register: u8,
        n_rows: u8,
    },
    JumpToAddress {
        address: u16,
    },
    LoadRegisterYToRegisterX {
        x_register: u8,
        y_register: u8,
    },
    AddRegisterXToImmediate {
        x_register: u8,
    },
    SkipNextIfRegisterXEqualsRegisterY {
        x_register: u8,
        y_register: u8,
    },
    OrRegisterXWithRegisterY {
        x_register: u8,
        y_register: u8,
    },
    AndRegisterXWithRegisterY {
        x_register: u8,
        y_register: u8,
    },
    XorRegisterXWithRegisterY {
        x_register: u8,
        y_register: u8,
    },
    AddRegisterYToRegisterX {
        x_register: u8,
        y_register: u8,
    },
    SubtractRegisterYFromRegisterX {
        x_register: u8,
        y_register: u8,
    },
    ShiftRegisterXRightWithRegisterY {
        x_register: u8,
        y_register: u8,
    },
    ShiftRegisterXLeftWithRegisterY {
        x_register: u8,
        y_register: u8,
    },
    SubtractNRegisterXFromRegisterY {
        x_register: u8,
        y_register: u8,
    },
    StoreBcdOfRegisterXAtIndex {
        x_register: u8,
    },
    SaveNumRegistersToImediate {
        n_registers: u8,
    },
    SaveImmediateToNumRegisters {
        n_registers: u8,
    },
}

#[derive(Debug)]
pub struct Chip8 {
    pub current_instruction: Option<Instruction>,
    pub delay_register: u8,
    pub frame_buffer: [[u8; 64]; 32],
    pub general_registers: [u8; 16],
    pub index_register: u16,
    pub memory: [u8; 4096],
    pub program_counter: u16,
    pub stack_memory: [u16; 16],
    pub stack_pointer: u8,
    pub timer_register: u8,
}

const SPRITES: [[u8; 5]; 16] = [
    [0xF0, 0x90, 0x90, 0x90, 0xF0], // '0'
    [0x20, 0x60, 0x20, 0x20, 0x70], // '1'
    [0xF0, 0x10, 0xf0, 0x80, 0xf0], // '2'
    [0xF0, 0x10, 0xf0, 0x10, 0xF0], // '3'
    [0x90, 0x90, 0xf0, 0x10, 0x10], // '4'
    [0xF0, 0x80, 0xf0, 0x10, 0xf0], // '5'
    [0xF0, 0x80, 0xf0, 0x90, 0xF0], // '6'
    [0x70, 0x10, 0x20, 0x40, 0x40], // '7'
    [0xF0, 0x90, 0xf0, 0x90, 0xF0], // '8'
    [0xF0, 0x90, 0xf0, 0x10, 0xF0], // '9'
    [0xF0, 0x90, 0x90, 0x90, 0xF0], // 'A'
    [0xF0, 0x90, 0x90, 0x90, 0xF0], // 'B'
    [0xF0, 0x90, 0x90, 0x90, 0xF0], // 'C'
    [0xF0, 0x90, 0x90, 0x90, 0xF0], // 'D'
    [0xF0, 0x90, 0x90, 0x90, 0xF0], // 'E'
    [0xF0, 0x90, 0x90, 0x90, 0xF0], // 'F'
];

pub const PROGRAM_START_OFFSET: usize = 0x200;

impl Chip8 {
    pub fn new() -> Self {
        let mut instance = Self {
            current_instruction: None,
            delay_register: 0,
            frame_buffer: [[0; 64]; 32],
            general_registers: [0; 16],
            index_register: 0,
            memory: [0; 4096],
            program_counter: PROGRAM_START_OFFSET as u16,
            stack_memory: [0; 16],
            stack_pointer: 0,
            timer_register: 0,
        };

        SPRITES.iter().enumerate().for_each(|(index, item)| {
            let start = index * item.len();
            let end = start + item.len();
            instance.memory[start..end].copy_from_slice(item);
        });

        instance
    }

    pub fn new_from_program(insructions: &[u16]) -> Self {
        let mut instance = Self::new();

        for (index, &instr) in insructions.iter().enumerate() {
            let bytes = instr.to_be_bytes();
            let start = index * 2;

            let destinations = &mut instance.memory[PROGRAM_START_OFFSET as usize + start..];
            destinations[0..2].copy_from_slice(&bytes);
        }

        instance
    }

    pub fn new_from_program_file(program_path: &str) -> Result<Self> {
        let mut instance = Self::new();
        instance.load_program(program_path)?;
        Ok(instance)
    }

    pub fn load_program(&mut self, path: &str) -> Result<()> {
        let mut file = File::open(path)?;
        file.read(&mut self.memory[(PROGRAM_START_OFFSET as usize)..])?;
        Ok(())
    }

    pub fn fetch_instruction(&mut self) -> u16 {
        let start = self.program_counter as usize;
        let bytes = &self.memory[start..start + 2];

        let instruction = ((bytes[0] as u16) << 8) | bytes[1] as u16;

        self.program_counter += 2;
        return instruction;
    }

    pub fn tick(&mut self) -> Result<()> {
        let opcode = self.fetch_instruction();
        let instruction = self.decode_instruction(opcode)?;
        self.current_instruction = Some(instruction);
        self.execute_instruction(instruction);
        Ok(())
    }
}
