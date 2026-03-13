use chip8_rs::{Chip8, Instruction::Logical, Logical::*};

#[test]
fn xor_register_x_with_register_y_works() {
    let mut chip8 = Chip8::new_from_program(&[0x8233]);

    chip8.general_registers[2] = 0b01010101;
    chip8.general_registers[3] = 0b10101010;

    chip8.tick().unwrap();

    assert_eq!(
        chip8.current_instruction,
        Some(Logical(XorRegisterXWithRegisterY {
            x_register: 2,
            y_register: 3
        }))
    );

    assert_eq!(chip8.general_registers[2], 0b11111111);
    assert_eq!(chip8.general_registers[3], 0b10101010);
}

#[test]
fn or_register_x_with_register_y_works() {
    let mut chip8 = Chip8::new_from_program(&[0x8231]);

    chip8.general_registers[2] = 0b01000001;
    chip8.general_registers[3] = 0b11000010;

    chip8.tick().unwrap();

    assert_eq!(
        chip8.current_instruction,
        Some(Logical(OrRegisterXWithRegisterY {
            x_register: 2,
            y_register: 3
        }))
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
        Some(Logical(AndRegisterXWithRegisterY {
            x_register: 2,
            y_register: 3
        }))
    );

    assert_eq!(chip8.general_registers[2], 0b00000001);
    assert_eq!(chip8.general_registers[3], 0b10101011);
}
