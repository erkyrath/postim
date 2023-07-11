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

    let res = parse_comment::<nom::error::VerboseError<&str>>(&body);
    // res: Result<(&str, ()), nom::Err<VerboseError<&str>>>
    
    if let Err(err) = res {
        if let nom::Err::Error(verberr) = err {
            let errstr = nom::error::convert_error::<&str>(&body, verberr);
            println!("{filename}: {errstr}");
        }
    }

    Ok(())
}
