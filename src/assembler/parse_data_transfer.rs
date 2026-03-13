use nom::{IResult, branch::alt, combinator::map};

use crate::DataTransfer;

pub fn parse_data_transfer_instruction(input: &str) -> IResult<&str, DataTransfer> {
    todo!();
}
