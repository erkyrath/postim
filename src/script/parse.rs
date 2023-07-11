use std::error::Error;

use nom::IResult;
use nom::error::ParseError;

use nom::{
    combinator::value,
    sequence::pair,
    bytes::complete::is_not,
    character::complete::char,
};

pub fn parse_comment<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, (), E> {
    value(
        (),
        pair(char('#'), is_not("\n\r"))
    )(input)
}


pub fn load_script(filename: &str) -> Result<(), Box<dyn Error>> {
    let body = std::fs::read_to_string(filename)?;

    // See nom "error_management.md" for this to_owned() business.
    
    parse_comment(&body)
        .map_err(|e: nom::Err<nom::error::Error<&str>>| e.to_owned())?;

    Ok(())
}
