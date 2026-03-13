use chip8_rs::{Chip8, ControlFlow::*, Instruction::ControlFlow, PROGRAM_START_OFFSET};

#[test]
fn skip_next_if_register_x_equals_register_y_works() {
    let mut chip8 = Chip8::new_from_program(&[0x5230]);

    chip8.general_registers[2] = 23;
    chip8.general_registers[3] = 23;

    assert_eq!(chip8.program_counter, PROGRAM_START_OFFSET as u16);
    chip8.tick().unwrap();

    assert_eq!(
        chip8.current_instruction,
        Some(ControlFlow(SkipNextIfRegisterXEqualsRegisterY {
            x_register: 2,
            y_register: 3
        }))
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
        Some(ControlFlow(SkipNextIfRegisterXEqualsRegisterY {
            x_register: 2,
            y_register: 3
        }))
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
        Some(ControlFlow(SkipNextIfRegisterXEqualsImmediate {
            x_register: 2,
            value: 0xFE
        }))
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
        Some(ControlFlow(SkipNextIfRegisterXEqualsImmediate {
            x_register: 2,
            value: 0xFE
        }))
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
        Some(ControlFlow(SkipNextIfRegisterYNotEqualRegisterX {
            x_register: 2,
            y_register: 3
        }))
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
        Some(ControlFlow(SkipNextIfRegisterYNotEqualRegisterX {
            x_register: 2,
            y_register: 3
        }))
    );

    assert_eq!(chip8.general_registers[2], 0xfe);
    assert_eq!(chip8.general_registers[3], 0xfe);
    assert_eq!(chip8.program_counter, PROGRAM_START_OFFSET as u16 + 2);
}
