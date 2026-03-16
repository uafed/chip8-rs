use nom::{
    IResult, Parser, branch::alt, bytes::complete::tag_no_case, character::complete::space1,
    combinator::map, sequence::separated_pair,
};

use crate::{
    Keyboard,
    assembler::primitives::{arguments_separator, parse_general_register, parse_keyboard_register},
};

fn parse_wait_key_press_register(input: &str) -> IResult<&str, Keyboard> {
    let (input, _) = (tag_no_case("ld"), space1).parse(input)?;
    let (input, (x_register, _)) = separated_pair(
        parse_general_register,
        arguments_separator,
        parse_keyboard_register,
    )
    .parse(input)?;

    Ok((input, Keyboard::WaitUntilKeyIsPressedPressed { x_register }))
}

fn parse_conditional_skip_instruction(input: &str) -> IResult<&str, Keyboard> {
    enum SkipIf {
        IsPressed,
        NotPressed,
    }
    let (input, (command, _)) = (
        alt((
            map(tag_no_case("skp"), |_| SkipIf::IsPressed),
            map(tag_no_case("snkp"), |_| SkipIf::NotPressed),
        )),
        space1,
    )
        .parse(input)?;

    let (input, x_register) = parse_general_register.parse(input)?;

    Ok((
        input,
        match command {
            SkipIf::IsPressed => Keyboard::SkipIfKeyInRegisterXIsPressed { x_register },
            SkipIf::NotPressed => Keyboard::SkipIfKeyInRegisterXIsNotPressed { x_register },
        },
    ))
}

pub fn parse_keyboard_instruction(input: &str) -> IResult<&str, Keyboard> {
    alt((
        parse_wait_key_press_register,
        parse_conditional_skip_instruction,
    ))
    .parse(input)
}
