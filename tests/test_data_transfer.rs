use chip8_rs::{Chip8, DataTransfer::*, Instruction::DataTransfer};

#[test]
fn load_immediate_to_register_works() {
    let mut chip8 = Chip8::new_from_program(&[0x61FE]);

    chip8.tick().unwrap();

    assert_eq!(
        chip8.current_instruction,
        Some(DataTransfer(LoadImmediateToRegister {
            x_register: 1,
            value: 0xfe
        }))
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
fn load_immediate_to_index_register_works() {
    let mut chip8 = Chip8::new_from_program(&[0xA123]);

    chip8.tick().unwrap();

    assert_eq!(
        chip8.current_instruction,
        Some(DataTransfer(LoadImmediateToIndexRegister {
            address: 0x123
        }))
    );
    assert_eq!(chip8.index_register, 0x123);
}

#[test]
fn load_register_y_to_register_x_works() {
    let mut chip8 = Chip8::new_from_program(&[0x8120]);

    chip8.general_registers[1] = 0xfe;
    chip8.general_registers[2] = 0xed;

    chip8.tick().unwrap();

    assert_eq!(
        chip8.current_instruction,
        Some(DataTransfer(LoadRegisterYToRegisterX {
            x_register: 1,
            y_register: 2
        }))
    );

    assert_eq!(chip8.general_registers[1], 0xed);
    assert_eq!(chip8.general_registers[2], 0xed);
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
        Some(DataTransfer(SaveNumRegistersToImediate {
            n_registers: (data.len() - 1) as u8
        })),
    );

    assert_eq!(
        &chip8.memory[chip8.index_register as usize
            ..chip8.index_register as usize + chip8.general_registers.len()],
        data.as_slice()
    );
}
