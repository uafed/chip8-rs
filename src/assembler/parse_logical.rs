use nom::{
    IResult, Parser, branch::alt, bytes::complete::tag_no_case, character::complete::space1,
    combinator::map, sequence::separated_pair,
};

use crate::{
    Logical,
    assembler::primitives::{arguments_separator, parse_general_register},
};

pub fn parse_logical_instruction(input: &str) -> IResult<&str, Logical> {
    enum Command {
        Or,
        And,
        Xor,
        Shr,
        Shl,
    }

    let (input, (command, _)) = (
        alt((
            map(tag_no_case("or"), |_| Command::Or),
            map(tag_no_case("and"), |_| Command::And),
            map(tag_no_case("xor"), |_| Command::Xor),
            map(tag_no_case("shr"), |_| Command::Shr),
            map(tag_no_case("shl"), |_| Command::Shl),
        )),
        space1,
    )
        .parse(input)?;

    let (input, (x_register, y_register)) = separated_pair(
        parse_general_register,
        arguments_separator,
        parse_general_register,
    )
    .parse(input)?;

    Ok((
        input,
        match command {
            Command::Or => Logical::OrRegisterXWithRegisterY {
                x_register,
                y_register,
            },
            Command::And => Logical::AndRegisterXWithRegisterY {
                x_register,
                y_register,
            },
            Command::Xor => Logical::XorRegisterXWithRegisterY {
                x_register,
                y_register,
            },
            Command::Shl => Logical::ShiftRegisterXLeftWithRegisterY {
                x_register,
                y_register,
            },
            Command::Shr => Logical::ShiftRegisterXRightWithRegisterY {
                x_register,
                y_register,
            },
        },
    ))
}
