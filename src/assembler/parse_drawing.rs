use nom::{
    IResult, Parser, branch::alt, bytes::complete::tag_no_case, combinator::map,
    sequence::separated_pair,
};

use crate::{
    Drawing,
    assembler::primitives::{arguments_separator, parse_byte_value, parse_general_register},
};

fn parse_clear_instruction(input: &str) -> IResult<&str, Drawing> {
    (map(tag_no_case("clear"), |_| Drawing::ClearScreen)).parse(input)
}

fn parse_draw_instruction(input: &str) -> IResult<&str, Drawing> {
    let (input, _) = tag_no_case("draw").parse(input)?;
    let (input, ((x_register, y_register), n_rows)) = separated_pair(
        separated_pair(
            parse_general_register,
            arguments_separator,
            parse_general_register,
        ),
        arguments_separator,
        parse_byte_value,
    )
    .parse(input)?;
    Ok((
        input,
        Drawing::DrawSpriteToScreen {
            x_register,
            y_register,
            n_rows,
        },
    ))
}

pub fn parse_drawing_instruction(input: &str) -> IResult<&str, Drawing> {
    alt((parse_clear_instruction, parse_draw_instruction)).parse(input)
}
