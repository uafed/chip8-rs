use nom::{
    IResult, Parser, branch::alt, bytes::complete::tag_no_case, character::complete::space1,
    combinator::map, sequence::separated_pair,
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

pub fn parse_control_flow_instruction(input: &str) -> IResult<&str, ControlFlow> {
    alt((
        parse_call_instruction,
        parse_jump_to_address_instruction,
        parse_return_from_subroutine,
        parse_skip_instruction,
    ))
    .parse(input)
}
