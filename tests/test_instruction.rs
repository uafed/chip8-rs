use chip8_rs::Chip8;
use chip8_rs::Instruction;

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
