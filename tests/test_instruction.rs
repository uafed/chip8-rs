use chip8_rs::{Chip8, PROGRAM_START_OFFSET};

#[test]
fn jump_to_addr_works() {
    let mut chip8 = Chip8::new_from_program(&[0x1234]);

    chip8.tick().unwrap();
    assert_eq!(chip8.program_counter, 0x0234);
}

#[test]
fn set_index_register_works() {
    let mut chip8 = Chip8::new_from_program(&[0xa123]);

    chip8.tick().unwrap();
    assert_eq!(chip8.index_register, 0x0123);
}

#[test]
fn load_byte_to_register_works() {
    let mut chip8 = Chip8::new_from_program(&[0x6234]);

    chip8.tick().unwrap();
    assert_eq!(chip8.general_registers[2], 0x0034);
}

#[test]
fn load_register_x_to_register_y() {
    let mut chip8 = Chip8::new_from_program(&[0x8120]);

    chip8.general_registers[2] = 0xcd;
    chip8.general_registers[1] = 0xab;

    chip8.tick().unwrap();
    assert_eq!(chip8.general_registers[2], 0xcd);
    assert_eq!(chip8.general_registers[1], 0xcd);

    let mut chip8 = Chip8::new_from_program(&[0x8210]);

    chip8.general_registers[2] = 0xcd;
    chip8.general_registers[1] = 0xab;

    chip8.tick().unwrap();
    assert_eq!(chip8.general_registers[2], 0xab);
    assert_eq!(chip8.general_registers[1], 0xab);
}

#[test]
fn add_register_to_index_register_works() {
    let mut chip8 = Chip8::new_from_program(&[0xf11e]);

    assert!(chip8.index_register == 0);
    chip8.general_registers[1] = 64;

    chip8.tick().unwrap();
    assert!(chip8.index_register == 64);
}

#[test]
fn store_registers_at_index_address_works() {
    let mut chip8 = Chip8::new_from_program(&[0xff55]);
    let start = (PROGRAM_START_OFFSET + 200) as u16;

    for index in 0..chip8.general_registers.len() {
        chip8.general_registers[index] = (chip8.general_registers.len() - 1 - index) as u8;
    }
    chip8.index_register = start;

    chip8.tick().unwrap();

    assert!(chip8.index_register == 712);
    for index in 0..15 {
        assert_eq!(
            chip8.memory[start as usize + index],
            (chip8.general_registers.len() - 1 - index) as u8
        );
    }
}

#[test]
fn load_index_address_to_register_works() {
    let mut chip8 = Chip8::new_from_program(&[0xff65]);
    let start = (PROGRAM_START_OFFSET + 200) as u16;

    for index in 0..chip8.general_registers.len() {
        chip8.memory[start as usize + index] = (chip8.general_registers.len() - 1 - index) as u8;
    }
    chip8.index_register = start;

    chip8.tick().unwrap();

    assert!(chip8.index_register == 712);
    for index in 0..chip8.general_registers.len() {
        assert_eq!(
            chip8.general_registers[index],
            (chip8.general_registers.len() - 1 - index) as u8
        );
    }
}

#[test]
fn add_value_to_register_works() {
    let mut chip8 = Chip8::new_from_program(&[0xff65]);
    let start = (PROGRAM_START_OFFSET + 200) as u16;

    for index in 0..chip8.general_registers.len() {
        chip8.memory[start as usize + index] = (chip8.general_registers.len() - 1 - index) as u8;
    }
    chip8.index_register = start;

    chip8.tick().unwrap();

    assert!(chip8.index_register == 712);
    for index in 0..chip8.general_registers.len() {
        assert_eq!(
            chip8.general_registers[index],
            (chip8.general_registers.len() - 1 - index) as u8
        );
    }
}
