use chip8_rs::{Chip8, Drawing::*, Instruction::Drawing};

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
        Some(Drawing(DrawSpriteToScreen {
            x_register: 1,
            y_register: 2,
            n_rows: 5
        }))
    );

    for y in 0..sprite.len() {
        assert_eq!(
            chip8.frame_buffer[y_pos as usize + y][x_pos as usize..x_pos as usize + 8],
            [1; 8]
        );
    }
}

#[test]
fn clear_screen_works() {
    let mut chip8 = Chip8::new_from_program(&[0x00E0]);
    chip8.frame_buffer = [[255; 64]; 32];
    assert_eq!(chip8.frame_buffer, [[255; 64]; 32]);

    chip8.tick().unwrap();
    assert_eq!(chip8.current_instruction, Some(Drawing(ClearScreen)));
    assert_eq!(chip8.frame_buffer, [[0; 64]; 32]);
}
