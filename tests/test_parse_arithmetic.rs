use chip8_rs::{Arithmetic, Instruction, assembler::parse_instructions};

#[test]
fn parse_add_immediate_to_register_works() {
    let (_, instructions) =
        parse_instructions("add v4, 0xfe").expect("Failed to parse instructions");

    assert_eq!(
        instructions.as_slice(),
        &[Instruction::Arithmetic(
            Arithmetic::AddImmediateToRegister {
                x_register: 4,
                value: 0xfe
            }
        )]
    )
}
