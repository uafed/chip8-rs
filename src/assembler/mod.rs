use nom::{IResult, Parser, branch::alt, combinator::map, multi::many1};

use crate::{
    Instruction,
    assembler::{
        parse_arithmetic::parse_arithmetic_instruction,
        parse_control_flow::parse_control_flow_instruction,
        parse_data_transfer::parse_data_transfer_instruction,
        parse_drawing::parse_drawing_instruction, parse_keyboard::parse_keyboard_instruction,
        parse_logical::parse_logical_instruction, parse_timer::parse_timer_instruction,
    },
};

mod parse_arithmetic;
mod parse_control_flow;
mod parse_data_transfer;
mod parse_drawing;
mod parse_keyboard;
mod parse_logical;
mod parse_timer;
mod primitives;

pub fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(alt((
        map(parse_control_flow_instruction, |control_flow| {
            Instruction::ControlFlow(control_flow)
        }),
        map(parse_arithmetic_instruction, |arithmetic| {
            Instruction::Arithmetic(arithmetic)
        }),
        map(parse_data_transfer_instruction, |data_transfer| {
            Instruction::DataTransfer(data_transfer)
        }),
        // map(parse_logical_instruction, |logical| {
        //     Instruction::Logical(logical)
        // }),
        // map(parse_drawing_instruction, |drawing| {
        //     Instruction::Drawing(drawing)
        // }),
        // map(parse_timer_instruction, |timer| Instruction::Timer(timer)),
        // map(parse_keyboard_instruction, |keyboard| {
        //     Instruction::Keyboard(keyboard)
        // }),
    )))
    .parse(input)
}
