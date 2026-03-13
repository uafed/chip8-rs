use chip8_rs::{Arithmetic::*, Chip8, Instruction::Arithmetic};

#[test]
fn add_immediate_to_register_works() {
    let mut chip8 = Chip8::new_from_program(&[0x7109]);

    chip8.tick().unwrap();

    assert_eq!(
        chip8.current_instruction,
        Some(Arithmetic(AddImmediateToRegister {
            x_register: 1,
            value: 0x09
        }))
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
        Some(Arithmetic(AddImmediateToRegister {
            x_register: 1,
            value: 0x09
        }))
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
fn add_register_x_to_immediate_works() {
    let mut chip8 = Chip8::new_from_program(&[0xF21E]);

    chip8.general_registers[2] = 24;
    chip8.index_register = 32;

    chip8.tick().unwrap();

    assert_eq!(
        chip8.current_instruction,
        Some(Arithmetic(AddRegisterXToImmediate { x_register: 2 }))
    );

    assert_eq!(chip8.general_registers[2], 24);
    assert_eq!(chip8.index_register, 56);
}

#[test]
fn add_register_x_to_register_y_works() {
    let mut chip8 = Chip8::new_from_program(&[0x8234]);

    chip8.general_registers[2] = 24;
    chip8.general_registers[3] = 42;

    chip8.tick().unwrap();

    assert_eq!(
        chip8.current_instruction,
        Some(Arithmetic(AddRegisterYToRegisterX {
            x_register: 2,
            y_register: 3
        }))
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
        Some(Arithmetic(AddRegisterYToRegisterX {
            x_register: 2,
            y_register: 3
        }))
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
        Some(Arithmetic(SubtractRegisterYFromRegisterX {
            x_register: 2,
            y_register: 3
        }))
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
        Some(Arithmetic(SubtractRegisterYFromRegisterX {
            x_register: 2,
            y_register: 3
        }))
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
        Some(Arithmetic(SubtractRegisterYFromRegisterX {
            x_register: 2,
            y_register: 3
        }))
    );

    assert_eq!(chip8.general_registers[2], 238);
    assert_eq!(chip8.general_registers[3], 42);
    assert_eq!(chip8.get_flag_register(), 0);
}
