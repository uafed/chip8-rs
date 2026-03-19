use std::collections::HashMap;

use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, line_ending, space0},
    combinator::{all_consuming, map},
    multi::{many1, separated_list1},
};

use crate::{
    Instruction, PROGRAM_START_OFFSET,
    assembler::{
        encode_arithmetic::encode_arithmetic,
        encode_control_flow::encode_control_flow,
        encode_data_transfer::encode_data_transfer,
        encode_drawing::encode_drawing,
        encode_keyboard::encode_keyboard,
        encode_logical::encode_logical,
        encode_timer::encode_timer,
        parse_arithmetic::parse_arithmetic_instruction,
        parse_control_flow::{
            AddressControlFlow, AssemblyControlFlow, parse_control_flow_instruction,
        },
        parse_data_transfer::parse_data_transfer_instruction,
        parse_drawing::parse_drawing_instruction,
        parse_keyboard::parse_keyboard_instruction,
        parse_logical::parse_logical_instruction,
        parse_timer::parse_timer_instruction,
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

fn parse_label(input: &str) -> IResult<&str, &str> {
    let (input, (label, _)) = ((alphanumeric1, tag(":"))).parse(input)?;

    Ok((input, label))
}

#[derive(Debug, Clone, PartialEq)]
pub enum InstructionWithLabel {
    JumpToLabel(String),
}

#[derive(PartialEq, Debug)]
pub enum AssemblerLine<'a> {
    Label(&'a str),
    Instruction(Instruction),
    InstructionWithLabel(InstructionWithLabel),
}

pub fn parse_instructions<'a>(input: &'a str) -> IResult<&'a str, Vec<AssemblerLine<'a>>> {
    all_consuming(separated_list1(
        many1((space0, line_ending)),
        alt((
            map((space0, parse_label), |(_, label)| {
                AssemblerLine::Label(label)
            }),
            map(
                (space0, parse_control_flow_instruction),
                |(_, control_flow)| match control_flow {
                    AssemblyControlFlow::Address(AddressControlFlow::JumpTolabel(address)) => {
                        AssemblerLine::InstructionWithLabel(InstructionWithLabel::JumpToLabel(
                            address,
                        ))
                    }
                    AssemblyControlFlow::NonAddress(instruction) => {
                        AssemblerLine::Instruction(Instruction::ControlFlow(instruction))
                    }
                },
            ),
            map((space0, parse_arithmetic_instruction), |(_, arithmetic)| {
                AssemblerLine::Instruction(Instruction::Arithmetic(arithmetic))
            }),
            map(
                (space0, parse_data_transfer_instruction),
                |(_, data_transfer)| {
                    AssemblerLine::Instruction(Instruction::DataTransfer(data_transfer))
                },
            ),
            map((space0, parse_logical_instruction), |(_, logical)| {
                AssemblerLine::Instruction(Instruction::Logical(logical))
            }),
            map((space0, parse_drawing_instruction), |(_, drawing)| {
                AssemblerLine::Instruction(Instruction::Drawing(drawing))
            }),
            map((space0, parse_timer_instruction), |(_, timer)| {
                AssemblerLine::Instruction(Instruction::Timer(timer))
            }),
            map((space0, parse_keyboard_instruction), |(_, keyboard)| {
                AssemblerLine::Instruction(Instruction::Keyboard(keyboard))
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

pub fn encode_single_instruction_with_label(
    instruction: &InstructionWithLabel,
    label_table: &HashMap<String, u16>,
) -> u16 {
    match instruction {
        InstructionWithLabel::JumpToLabel(label) => {
            let address = label_table.get(label).expect(
                format!("Failed to obtain address for label: '{}'", label.clone()).as_str(),
            );
            println!("jump to: {:#06x} {}", 0x1000 | (*address), address);
            0x1000 | (*address)
        }
    }
}

pub fn encode_instructions(lines: &[AssemblerLine]) -> Vec<u16> {
    let mut label_table: HashMap<String, u16> = HashMap::new();

    lines
        .iter()
        .enumerate()
        .map(|(idx, line)| match line {
            AssemblerLine::Instruction(instruction) => Some(encode_single_instruction(instruction)),
            AssemblerLine::InstructionWithLabel(instruction) => Some(
                encode_single_instruction_with_label(instruction, &label_table),
            ),
            AssemblerLine::Label(label) => {
                let address = (PROGRAM_START_OFFSET as u16) + ((idx as u16) * 2);
                label_table.insert(String::from(*label), address);
                None
            }
        })
        .flatten()
        .collect::<Vec<u16>>()
}
