use chip8_rs::Chip8;
use chip8_rs::Instruction;
use chip8_rs::PROGRAM_START_OFFSET;

#[test]
fn clear_screen_works() {
    let mut chip8 = Chip8::new_from_program(&[0x00E0]);
    chip8.frame_buffer = [[255; 64]; 32];
    assert_eq!(chip8.frame_buffer, [[255; 64]; 32]);

    chip8.tick().unwrap();
    assert_eq!(chip8.current_instruction, Some(Instruction::ClearScreen));
    assert_eq!(chip8.frame_buffer, [[0; 64]; 32]);
}

#[test]
fn load_immediate_to_register_works() {
    let mut chip8 = Chip8::new_from_program(&[0x61FE]);

    chip8.tick().unwrap();

    assert_eq!(
        chip8.current_instruction,
        Some(Instruction::LoadImmediateToRegister {
            register: 1,
            value: 0xfe
        })
    );
    for (i, &value) in chip8.general_registers.iter().enumerate() {
        if i == 1 {
            assert_eq!(value, 0xfe);
        } else {
            assert_eq!(value, 0x00);
        }
    }
}

#[test]
fn add_immediate_to_register_works() {
    let mut chip8 = Chip8::new_from_program(&[0x7109]);

    chip8.tick().unwrap();

    assert_eq!(
        chip8.current_instruction,
        Some(Instruction::AddImmediateToRegister {
            register: 1,
            value: 0x09
        })
    );
    for (i, &value) in chip8.general_registers.iter().enumerate() {
        if i == 1 {
            assert_eq!(value, 0x09);
        } else {
            assert_eq!(value, 0x00);
        }
    }
}

#[test]
fn add_immediate_to_register_works_with_overflow() {
    let mut chip8 = Chip8::new_from_program(&[0x7109]);

    chip8.general_registers[1] = 255;

    chip8.tick().unwrap();

    assert_eq!(
        chip8.current_instruction,
        Some(Instruction::AddImmediateToRegister {
            register: 1,
            value: 0x09
        })
    );
    for (i, &value) in chip8.general_registers.iter().enumerate() {
        if i == 1 {
            assert_eq!(value, 0x08);
        } else {
            assert_eq!(value, 0x00);
        }
    }
}

#[test]
fn load_immediate_to_index_register_works() {
    let mut chip8 = Chip8::new_from_program(&[0xA123]);

    chip8.tick().unwrap();

    assert_eq!(
        chip8.current_instruction,
        Some(Instruction::LoadImmediateToIndexRegister { value: 0x123 })
    );
    assert_eq!(chip8.index_register, 0x123);
}

#[test]
fn draw_sprite_to_screen_works() {
    let mut chip8 = Chip8::new_from_program(&[0xD125]);
    let x_pos = 2;
    let y_pos = 4;

    chip8.general_registers[1] = x_pos;
    chip8.general_registers[2] = y_pos;
    chip8.index_register = 0x100;

    let sprite = &[0xff, 0xff, 0xff, 0xff, 0xff];
    chip8.memory[(chip8.index_register as usize)..(chip8.index_register as usize) + sprite.len()]
        .copy_from_slice(sprite);

    for y in 0..sprite.len() {
        assert_eq!(
            chip8.frame_buffer[y_pos as usize + y][x_pos as usize..x_pos as usize + 8],
            [0; 8]
        );
    }

    chip8.tick().unwrap();

    assert_eq!(
        chip8.current_instruction,
        Some(Instruction::DrawSpriteToScreen {
            x_register: 1,
            y_register: 2,
            n_rows: 5
        })
    );

    for y in 0..sprite.len() {
        assert_eq!(
            chip8.frame_buffer[y_pos as usize + y][x_pos as usize..x_pos as usize + 8],
            [1; 8]
        );
    }
}

#[test]
fn load_register_y_to_register_x_works() {
    let mut chip8 = Chip8::new_from_program(&[0x8120]);

    chip8.general_registers[1] = 0xfe;
    chip8.general_registers[2] = 0xed;

    chip8.tick().unwrap();

    assert_eq!(
        chip8.current_instruction,
        Some(Instruction::LoadRegisterYToRegisterX {
            x_register: 1,
            y_register: 2
        })
    );

    assert_eq!(chip8.general_registers[1], 0xed);
    assert_eq!(chip8.general_registers[2], 0xed);
}

#[test]
fn add_register_x_to_immediate_works() {
    let mut chip8 = Chip8::new_from_program(&[0xF21E]);

    chip8.general_registers[2] = 24;
    chip8.index_register = 32;

    chip8.tick().unwrap();

    assert_eq!(
        chip8.current_instruction,
        Some(Instruction::AddRegisterXToImmediate { x_register: 2 })
    );

    assert_eq!(chip8.general_registers[2], 24);
    assert_eq!(chip8.index_register, 56);
}

#[test]
fn save_num_registers_to_immediate_works() {
    let mut chip8 = Chip8::new_from_program(&[0xFF55]);

    let data: Vec<_> = (0..chip8.general_registers.len() as u8).collect();

    chip8.general_registers.copy_from_slice(data.as_slice());
    chip8.index_register = 0x220;

    assert_ne!(
        &chip8.memory[chip8.index_register as usize
            ..chip8.index_register as usize + chip8.general_registers.len()],
        data.as_slice()
    );

    chip8.tick().unwrap();

    assert_eq!(
        chip8.current_instruction,
        Some(Instruction::SaveNumRegistersToImediate {
            n_registers: (data.len() - 1) as u8
        }),
    );

    assert_eq!(
        &chip8.memory[chip8.index_register as usize
            ..chip8.index_register as usize + chip8.general_registers.len()],
        data.as_slice()
    );
}

#[test]
fn add_register_x_to_register_y_works() {
    let mut chip8 = Chip8::new_from_program(&[0x8234]);

    chip8.general_registers[2] = 24;
    chip8.general_registers[3] = 42;

    chip8.tick().unwrap();

    assert_eq!(
        chip8.current_instruction,
        Some(Instruction::AddRegisterYToRegisterX {
            x_register: 2,
            y_register: 3
        })
    );

    assert_eq!(chip8.general_registers[2], 66);
    assert_eq!(chip8.general_registers[3], 42);
    assert_eq!(chip8.get_flag_register(), 0);
}

#[test]
fn add_register_x_to_register_y_works_with_overflow() {
    let mut chip8 = Chip8::new_from_program(&[0x8234]);

    chip8.general_registers[2] = 1;
    chip8.general_registers[3] = 255;

    chip8.tick().unwrap();

    assert_eq!(
        chip8.current_instruction,
        Some(Instruction::AddRegisterYToRegisterX {
            x_register: 2,
            y_register: 3
        })
    );

    assert_eq!(chip8.general_registers[2], 0);
    assert_eq!(chip8.general_registers[3], 255);
    assert_eq!(chip8.get_flag_register(), 1);
}

#[test]
fn subtract_register_y_from_register_x_works() {
    let mut chip8 = Chip8::new_from_program(&[0x8235]);

    chip8.general_registers[2] = 42;
    chip8.general_registers[3] = 24;

    chip8.tick().unwrap();

    assert_eq!(
        chip8.current_instruction,
        Some(Instruction::SubtractRegisterYFromRegisterX {
            x_register: 2,
            y_register: 3
        })
    );

    assert_eq!(chip8.general_registers[2], 18);
    assert_eq!(chip8.general_registers[3], 24);
    assert_eq!(chip8.get_flag_register(), 1);
}

#[test]
fn subtract_register_y_from_register_x_works_with_overflow() {
    let mut chip8 = Chip8::new_from_program(&[0x8235]);

    chip8.general_registers[2] = 24;
    chip8.general_registers[3] = 25;

    chip8.tick().unwrap();

    assert_eq!(
        chip8.current_instruction,
        Some(Instruction::SubtractRegisterYFromRegisterX {
            x_register: 2,
            y_register: 3
        })
    );

    assert_eq!(chip8.general_registers[2], 255);
    assert_eq!(chip8.general_registers[3], 25);
    assert_eq!(chip8.get_flag_register(), 0);
}

#[test]
fn subtract_register_y_from_register_x_with_borrom_works() {
    let mut chip8 = Chip8::new_from_program(&[0x8235]);

    chip8.general_registers[2] = 24;
    chip8.general_registers[3] = 42;

    chip8.tick().unwrap();

    assert_eq!(
        chip8.current_instruction,
        Some(Instruction::SubtractRegisterYFromRegisterX {
            x_register: 2,
            y_register: 3
        })
    );

    assert_eq!(chip8.general_registers[2], 238);
    assert_eq!(chip8.general_registers[3], 42);
    assert_eq!(chip8.get_flag_register(), 0);
}

#[test]
fn xor_register_x_with_register_y_works() {
    let mut chip8 = Chip8::new_from_program(&[0x8233]);

    chip8.general_registers[2] = 0b01010101;
    chip8.general_registers[3] = 0b10101010;

    chip8.tick().unwrap();

    assert_eq!(
        chip8.current_instruction,
        Some(Instruction::XorRegisterXWithRegisterY {
            x_register: 2,
            y_register: 3
        })
    );

    assert_eq!(chip8.general_registers[2], 0b11111111);
    assert_eq!(chip8.general_registers[3], 0b10101010);
}

#[test]
fn skip_next_if_register_x_equals_register_y_works() {
    let mut chip8 = Chip8::new_from_program(&[0x5230]);

    chip8.general_registers[2] = 23;
    chip8.general_registers[3] = 23;

    assert_eq!(chip8.program_counter, PROGRAM_START_OFFSET as u16);
    chip8.tick().unwrap();

    assert_eq!(
        chip8.current_instruction,
        Some(Instruction::SkipNextIfRegisterXEqualsRegisterY {
            x_register: 2,
            y_register: 3
        })
    );

    assert_eq!(chip8.general_registers[2], 23);
    assert_eq!(chip8.general_registers[3], 23);
    assert_eq!(chip8.program_counter, PROGRAM_START_OFFSET as u16 + 4);
}

#[test]
fn skip_next_if_register_x_equals_register_y_works_if_not_equal() {
    let mut chip8 = Chip8::new_from_program(&[0x5230]);

    chip8.general_registers[2] = 23;
    chip8.general_registers[3] = 24;

    assert_eq!(chip8.program_counter, PROGRAM_START_OFFSET as u16);
    chip8.tick().unwrap();

    assert_eq!(
        chip8.current_instruction,
        Some(Instruction::SkipNextIfRegisterXEqualsRegisterY {
            x_register: 2,
            y_register: 3
        })
    );

    assert_eq!(chip8.general_registers[2], 23);
    assert_eq!(chip8.general_registers[3], 24);
    assert_eq!(chip8.program_counter, PROGRAM_START_OFFSET as u16 + 2);
}

#[test]
fn skip_next_if_register_x_equals_immediate_works() {
    let mut chip8 = Chip8::new_from_program(&[0x32FE]);

    chip8.general_registers[2] = 0xFE;

    assert_eq!(chip8.program_counter, PROGRAM_START_OFFSET as u16);
    chip8.tick().unwrap();

    assert_eq!(
        chip8.current_instruction,
        Some(Instruction::SkipNextIfRegisterXEqualsImmediate {
            x_register: 2,
            value: 0xFE
        })
    );

    assert_eq!(chip8.general_registers[2], 0xFE);
    assert_eq!(chip8.program_counter, PROGRAM_START_OFFSET as u16 + 4);
}

#[test]
fn skip_next_if_register_x_equals_immediate_works_if_not_equal() {
    let mut chip8 = Chip8::new_from_program(&[0x32FE]);

    chip8.general_registers[2] = 0xED;

    assert_eq!(chip8.program_counter, PROGRAM_START_OFFSET as u16);
    chip8.tick().unwrap();

    assert_eq!(
        chip8.current_instruction,
        Some(Instruction::SkipNextIfRegisterXEqualsImmediate {
            x_register: 2,
            value: 0xFE
        })
    );

    assert_eq!(chip8.general_registers[2], 0xED);
    assert_eq!(chip8.program_counter, PROGRAM_START_OFFSET as u16 + 2);
}

#[test]
fn skip_next_if_register_x_not_equals_register_y_works() {
    let mut chip8 = Chip8::new_from_program(&[0x9230]);

    chip8.general_registers[2] = 0xfe;
    chip8.general_registers[3] = 0xed;

    assert_eq!(chip8.program_counter, PROGRAM_START_OFFSET as u16);
    chip8.tick().unwrap();

    assert_eq!(
        chip8.current_instruction,
        Some(Instruction::SkipNextIfRegisterYNotEqualRegisterX {
            x_register: 2,
            y_register: 3
        })
    );

    assert_eq!(chip8.general_registers[2], 0xfe);
    assert_eq!(chip8.general_registers[3], 0xed);
    assert_eq!(chip8.program_counter, PROGRAM_START_OFFSET as u16 + 4);
}

#[test]
fn skip_next_if_register_x_not_equals_register_y_works_if_equal() {
    let mut chip8 = Chip8::new_from_program(&[0x9230]);

    chip8.general_registers[2] = 0xfe;
    chip8.general_registers[3] = 0xfe;

    assert_eq!(chip8.program_counter, PROGRAM_START_OFFSET as u16);
    chip8.tick().unwrap();

    assert_eq!(
        chip8.current_instruction,
        Some(Instruction::SkipNextIfRegisterYNotEqualRegisterX {
            x_register: 2,
            y_register: 3
        })
    );

    assert_eq!(chip8.general_registers[2], 0xfe);
    assert_eq!(chip8.general_registers[3], 0xfe);
    assert_eq!(chip8.program_counter, PROGRAM_START_OFFSET as u16 + 2);
}

#[test]
fn or_register_x_with_register_y_works() {
    let mut chip8 = Chip8::new_from_program(&[0x8231]);

    chip8.general_registers[2] = 0b01000001;
    chip8.general_registers[3] = 0b11000010;

    chip8.tick().unwrap();

    assert_eq!(
        chip8.current_instruction,
        Some(Instruction::OrRegisterXWithRegisterY {
            x_register: 2,
            y_register: 3
        })
    );

    assert_eq!(chip8.general_registers[2], 0b11000011);
    assert_eq!(chip8.general_registers[3], 0b11000010);
}

#[test]
fn and_register_x_with_register_y_works() {
    let mut chip8 = Chip8::new_from_program(&[0x8232]);

    chip8.general_registers[2] = 0b01010101;
    chip8.general_registers[3] = 0b10101011;

    chip8.tick().unwrap();

    assert_eq!(
        chip8.current_instruction,
        Some(Instruction::AndRegisterXWithRegisterY {
            x_register: 2,
            y_register: 3
        })
    );

    assert_eq!(chip8.general_registers[2], 0b00000001);
    assert_eq!(chip8.general_registers[3], 0b10101011);
}
