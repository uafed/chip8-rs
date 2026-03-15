#![allow(dead_code)]

use nom::{
    AsChar, IResult, Parser,
    bytes::{
        complete::{tag_no_case, take_while_m_n},
        tag,
    },
    character::complete::{char, space0},
    combinator::map_res,
    sequence::delimited,
};

pub fn from_hex_digit(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}

pub fn is_hex_digit(c: char) -> bool {
    c.is_hex_digit()
}

pub fn arguments_separator(input: &str) -> IResult<&str, ()> {
    let (input, _) = (space0, tag(","), space0).parse(input)?;
    Ok((input, ()))
}

pub fn hex_byte(input: &str) -> IResult<&str, u8> {
    map_res(take_while_m_n(2, 2, is_hex_digit), from_hex_digit).parse(input)
}

pub fn parse_byte_value(input: &str) -> IResult<&str, u8> {
    let (input, _) = tag("0x").parse(input)?;
    let (input, byte) = hex_byte.parse(input)?;
    Ok((input, byte))
}

pub fn parse_address(input: &str) -> IResult<&str, u16> {
    let (input, _) = tag("0x").parse(input)?;
    let (input, (high, low)) = (hex_byte, hex_byte).parse(input)?;
    Ok((input, ((high as u16) << 8) | (low as u16)))
}

pub fn parse_general_register(input: &str) -> IResult<&str, u8> {
    let (input, _) = tag_no_case("v")(input)?;
    let (input, index) =
        (map_res(take_while_m_n(1, 1, is_hex_digit), from_hex_digit)).parse(input)?;

    Ok((input, index))
}

pub fn parse_index_register(input: &str) -> IResult<&str, ()> {
    let (input, _) = tag_no_case("I")(input)?;

    Ok((input, ()))
}

pub fn parse_addressed_index_register(input: &str) -> IResult<&str, ()> {
    delimited(char('('), parse_index_register, char(')')).parse(input)
}
