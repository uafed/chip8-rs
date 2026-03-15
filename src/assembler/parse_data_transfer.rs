use nom::{
    IResult, Parser, branch::alt, bytes::complete::tag_no_case, character::complete::space1,
    combinator::map, sequence::separated_pair,
};

use crate::{
    DataTransfer,
    assembler::primitives::{
        arguments_separator, parse_address, parse_addressed_index_register, parse_byte_value,
        parse_general_register, parse_index_register,
    },
};

pub fn parse_store_bcd_instruction(input: &str) -> IResult<&str, DataTransfer> {
    let (input, _) = (tag_no_case("bcd"), space1).parse(input)?;
    let (input, (_, x_register)) = separated_pair(
        parse_addressed_index_register,
        arguments_separator,
        parse_general_register,
    )
    .parse(input)?;

    Ok((
        input,
        DataTransfer::StoreBcdOfRegisterXAtIndex { x_register },
    ))
}

pub fn parse_transfer_between_registers_and_immediate(input: &str) -> IResult<&str, DataTransfer> {
    let (input, _) = (tag_no_case("ld"), space1).parse(input)?;
    // Note:
    // Each case is laid out like this since I don't want to be able to successfully parse:
    // LD I, I or
    // LD [I], I or other similar cases
    (alt((
        map(
            separated_pair(
                parse_addressed_index_register,
                arguments_separator,
                parse_general_register,
            ),
            |(_, n_registers)| DataTransfer::SaveNumRegistersToImediate { n_registers },
        ),
        map(
            separated_pair(
                parse_general_register,
                arguments_separator,
                parse_addressed_index_register,
            ),
            |(n_registers, _)| DataTransfer::SaveImmediateToNumRegisters { n_registers },
        ),
        map(
            separated_pair(parse_index_register, arguments_separator, parse_address),
            |(_, address)| DataTransfer::LoadImmediateToIndexRegister { address },
        ),
        map(
            separated_pair(
                parse_general_register,
                arguments_separator,
                parse_byte_value,
            ),
            |(x_register, value)| DataTransfer::LoadImmediateToRegister { x_register, value },
        ),
        map(
            separated_pair(
                parse_general_register,
                arguments_separator,
                parse_general_register,
            ),
            |(x_register, y_register)| DataTransfer::LoadRegisterYToRegisterX {
                x_register,
                y_register,
            },
        ),
    )))
    .parse(input)
}

pub fn parse_data_transfer_instruction(input: &str) -> IResult<&str, DataTransfer> {
    alt((
        parse_store_bcd_instruction,
        parse_transfer_between_registers_and_immediate,
    ))
    .parse(input)
}
