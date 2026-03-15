use nom::{
    IResult, Parser, branch::alt, bytes::complete::tag_no_case, character::complete::space1,
    combinator::map, sequence::separated_pair,
};

use crate::{
    Arithmetic,
    assembler::primitives::{
        arguments_separator, parse_byte_value, parse_general_register, parse_index_register,
    },
};

fn add_instruction(input: &str) -> IResult<&str, ()> {
    let (input, _) = (tag_no_case("add"), space1).parse(input)?;
    Ok((input, ()))
}

fn parse_add_immediate_to_register(input: &str) -> IResult<&str, Arithmetic> {
    let (input, _) = add_instruction.parse(input)?;
    let (input, (x_register, value)) = (separated_pair(
        parse_general_register,
        arguments_separator,
        parse_byte_value,
    ))
    .parse(input)?;

    Ok((
        input,
        Arithmetic::AddImmediateToRegister { x_register, value },
    ))
}

fn parse_add_register_to_index(input: &str) -> IResult<&str, Arithmetic> {
    let (input, _) = add_instruction.parse(input)?;
    let (input, (_, x_register)) = (separated_pair(
        parse_index_register,
        arguments_separator,
        parse_general_register,
    ))
    .parse(input)?;

    Ok((input, Arithmetic::AddRegisterXToIndex { x_register }))
}

fn parse_add_two_general_registers(input: &str) -> IResult<&str, Arithmetic> {
    let (input, _) = add_instruction.parse(input)?;
    let (input, (x_register, y_register)) = (separated_pair(
        parse_general_register,
        arguments_separator,
        parse_general_register,
    ))
    .parse(input)?;

    Ok((
        input,
        Arithmetic::AddRegisterYToRegisterX {
            x_register,
            y_register,
        },
    ))
}

enum Subtract {
    Sub,
    SubN,
}

fn parse_subtract_two_general_registers(input: &str) -> IResult<&str, Arithmetic> {
    let (input, (command, _)) = (
        alt((
            map(tag_no_case("sub"), |_| Subtract::Sub),
            map(tag_no_case("subn"), |_| Subtract::SubN),
        )),
        space1,
    )
        .parse(input)?;
    let (input, (x_register, y_register)) = (separated_pair(
        parse_general_register,
        arguments_separator,
        parse_general_register,
    ))
    .parse(input)?;

    Ok((
        input,
        match command {
            Subtract::SubN => Arithmetic::SubtractNRegisterXFromRegisterY {
                x_register,
                y_register,
            },
            Subtract::Sub => Arithmetic::SubtractRegisterYFromRegisterX {
                x_register,
                y_register,
            },
        },
    ))
}

pub fn parse_arithmetic_instruction(input: &str) -> IResult<&str, Arithmetic> {
    alt((
        parse_add_immediate_to_register,
        parse_add_register_to_index,
        parse_add_two_general_registers,
        parse_subtract_two_general_registers,
    ))
    .parse(input)
}
