use nom::{
    IResult, Parser, branch::alt, bytes::complete::tag_no_case, character::complete::space1,
    combinator::map, sequence::separated_pair,
};

use crate::{
    Timer,
    assembler::primitives::{
        arguments_separator, parse_delay_timer_register, parse_general_register,
        parse_sound_timer_register,
    },
};

// TODO: refactor this to a common function in primitives since we use this a
// lot
fn parse_load_command(input: &str) -> IResult<&str, ()> {
    map((tag_no_case("ld"), space1), |_| ()).parse(input)
}

fn parse_load_from_register_to_sound(input: &str) -> IResult<&str, Timer> {
    let (input, _) = parse_load_command(input)?;
    let (input, (_, x_register)) = separated_pair(
        parse_sound_timer_register,
        arguments_separator,
        parse_general_register,
    )
    .parse(input)?;
    Ok((input, Timer::LoadRegisterXToSoundTimer { x_register }))
}
fn parse_load_between_register_and_delay_timer(input: &str) -> IResult<&str, Timer> {
    let (input, _) = parse_load_command(input)?;
    let (input, instruction) = (alt((
        map(
            separated_pair(
                parse_general_register,
                arguments_separator,
                parse_delay_timer_register,
            ),
            |(x_register, _)| Timer::LoadRegisterXToDelayTimer { x_register },
        ),
        map(
            separated_pair(
                parse_delay_timer_register,
                arguments_separator,
                parse_general_register,
            ),
            |(_, x_register)| Timer::LoadDelayTimerToRegisterX { x_register },
        ),
    )))
    .parse(input)?;

    Ok((input, instruction))
}

pub fn parse_timer_instruction(input: &str) -> IResult<&str, Timer> {
    alt((
        parse_load_between_register_and_delay_timer,
        parse_load_from_register_to_sound,
    ))
    .parse(input)
}
