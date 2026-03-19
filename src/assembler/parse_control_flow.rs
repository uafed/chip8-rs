use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag_no_case,
    character::complete::{alphanumeric1, space1},
    combinator::map,
    sequence::separated_pair,
};

use crate::{
    ControlFlow::{self, *},
    assembler::primitives::{
        arguments_separator, parse_address, parse_byte_value, parse_general_register,
    },
};

fn parse_call_instruction(input: &str) -> IResult<&str, ControlFlow> {
    let (input, _) = (tag_no_case("call"), space1).parse(input)?;
    let (input, address) = parse_address.parse(input)?;

    Ok((input, CallSubroutine { address }))
}

fn parse_jump_to_address_instruction(input: &str) -> IResult<&str, ControlFlow> {
    let (input, _) = (tag_no_case("jmp"), space1).parse(input)?;
    let (input, address) = parse_address.parse(input)?;

    Ok((input, JumpToAddress { address }))
}

fn parse_jump_to_label_instruction(input: &str) -> IResult<&str, AddressControlFlow> {
    let (input, _) = (tag_no_case("jmp"), space1).parse(input)?;
    let (input, address) = alphanumeric1.parse(input)?;

    Ok((input, AddressControlFlow::JumpTolabel(address.to_string())))
}

fn parse_return_from_subroutine(input: &str) -> IResult<&str, ControlFlow> {
    let (input, _) = (tag_no_case("ret"), space1).parse(input)?;
    Ok((input, ReturnFromSubroutine))
}

enum ComparisonRhs {
    Byte(u8),
    Register(u8),
}

enum SkipIf {
    Equal,
    NotEqual,
}

fn parse_skip_instruction(input: &str) -> IResult<&str, ControlFlow> {
    let (input, (command, _)) = (
        alt((
            map(tag_no_case("se"), |_| SkipIf::Equal),
            map(tag_no_case("sne"), |_| SkipIf::NotEqual),
        )),
        space1,
    )
        .parse(input)?;
    let (input, (x_register, rhs)) = (separated_pair(
        parse_general_register,
        arguments_separator,
        alt((
            map(parse_byte_value, |byte| ComparisonRhs::Byte(byte)),
            map(parse_general_register, |register| {
                ComparisonRhs::Register(register)
            }),
        )),
    ))
    .parse(input)?;

    let instruction = match (command, rhs) {
        (SkipIf::Equal, ComparisonRhs::Register(y_register)) => {
            ControlFlow::SkipNextIfRegisterXEqualsRegisterY {
                x_register,
                y_register,
            }
        }
        (SkipIf::Equal, ComparisonRhs::Byte(value)) => {
            ControlFlow::SkipNextIfRegisterXEqualsImmediate { x_register, value }
        }
        (SkipIf::NotEqual, ComparisonRhs::Register(y_register)) => {
            ControlFlow::SkipNextIfRegisterYNotEqualRegisterX {
                x_register,
                y_register,
            }
        }
        (SkipIf::NotEqual, ComparisonRhs::Byte(value)) => {
            ControlFlow::SkipNextIfRegisterXNotEqualsImmediate { x_register, value }
        }
    };

    Ok((input, instruction))
}

#[derive(Clone, PartialEq)]
pub enum AddressControlFlow {
    JumpTolabel(String),
}

#[derive(Clone, PartialEq)]
pub enum AssemblyControlFlow {
    NonAddress(ControlFlow),
    Address(AddressControlFlow),
}

pub fn parse_control_flow_instruction(input: &str) -> IResult<&str, AssemblyControlFlow> {
    alt((
        map(parse_call_instruction, |instruction| {
            AssemblyControlFlow::NonAddress(instruction)
        }),
        map(parse_jump_to_address_instruction, |instruction| {
            AssemblyControlFlow::NonAddress(instruction)
        }),
        map(parse_return_from_subroutine, |instruction| {
            AssemblyControlFlow::NonAddress(instruction)
        }),
        map(parse_jump_to_label_instruction, |instruction| {
            AssemblyControlFlow::Address(instruction)
        }),
        map(parse_skip_instruction, |instruction| {
            AssemblyControlFlow::NonAddress(instruction)
        }),
    ))
    .parse(input)
}
