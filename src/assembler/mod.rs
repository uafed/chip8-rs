use nom::{
    IResult, Parser,
    branch::alt,
    character::complete::newline,
    combinator::{all_consuming, map},
    multi::separated_list1,
};

use crate::{
    Instruction,
    assembler::{
        encode_arithmetic::encode_arithmetic, encode_control_flow::encode_control_flow,
        encode_data_transfer::encode_data_transfer, encode_drawing::encode_drawing,
        encode_keyboard::encode_keyboard, encode_logical::encode_logical,
        encode_timer::encode_timer, parse_arithmetic::parse_arithmetic_instruction,
        parse_control_flow::parse_control_flow_instruction,
        parse_data_transfer::parse_data_transfer_instruction,
        parse_drawing::parse_drawing_instruction, parse_keyboard::parse_keyboard_instruction,
        parse_logical::parse_logical_instruction, parse_timer::parse_timer_instruction,
    },
};

mod encode_arithmetic;
mod encode_control_flow;
mod encode_data_transfer;
mod encode_drawing;
mod encode_keyboard;
mod encode_logical;
mod encode_timer;
mod parse_arithmetic;
mod parse_control_flow;
mod parse_data_transfer;
mod parse_drawing;
mod parse_keyboard;
mod parse_logical;
mod parse_timer;
mod primitives;

pub fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    all_consuming(separated_list1(
        newline,
        alt((
            map(parse_control_flow_instruction, |control_flow| {
                Instruction::ControlFlow(control_flow)
            }),
            map(parse_arithmetic_instruction, |arithmetic| {
                Instruction::Arithmetic(arithmetic)
            }),
            map(parse_data_transfer_instruction, |data_transfer| {
                Instruction::DataTransfer(data_transfer)
            }),
            map(parse_logical_instruction, |logical| {
                Instruction::Logical(logical)
            }),
            map(parse_drawing_instruction, |drawing| {
                Instruction::Drawing(drawing)
            }),
            map(parse_timer_instruction, |timer| Instruction::Timer(timer)),
            map(parse_keyboard_instruction, |keyboard| {
                Instruction::Keyboard(keyboard)
            }),
        )),
    ))
    .parse(input)
}

pub fn encode_single_instruction(instruction: &Instruction) -> u16 {
    match instruction {
        Instruction::ControlFlow(instruction) => encode_control_flow(instruction),
        Instruction::Arithmetic(instruction) => encode_arithmetic(instruction),
        Instruction::DataTransfer(instruction) => encode_data_transfer(instruction),
        Instruction::Drawing(instruction) => encode_drawing(instruction),
        Instruction::Keyboard(instruction) => encode_keyboard(instruction),
        Instruction::Timer(instruction) => encode_timer(instruction),
        Instruction::Logical(instruction) => encode_logical(instruction),
    }
}

pub fn encode_instructions(instructions: &[Instruction]) -> Vec<u16> {
    instructions
        .iter()
        .map(|instruction| encode_single_instruction(instruction))
        .collect::<Vec<u16>>()
}
