use nom::IResult;
use nom::Err;
use nom::error::ParseError;

use nom::{
    combinator::value,
    sequence::pair,
    branch::alt,
    bytes::complete::is_not,
    character::complete::char,
    character::complete::multispace1,
};

enum ScriptToken {
    Whitespace,
    Comment,
}

pub fn parse_comment<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, (), E> {
    value(
        (),
        pair(char('#'), is_not("\n\r"))
    )(input)
}

pub fn parse_whitespace<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, (), E> {
    value(
        (),
        multispace1
    )(input)
}

pub fn parse_anytoken<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, (), E> {
    alt((
        parse_comment,
        parse_whitespace
    ))(input)
}

pub fn load_script(filename: &str) -> Result<(), String> {
    let body = std::fs::read_to_string(filename)
        .map_err(|err| {
            format!("{}: {}", filename, err.to_string())
        })?;

    // parser returns Result<(&str,()), nom::Err<VerboseError<&str>>>
    let res = parse_anytoken::<nom::error::VerboseError<&str>>(&body)
        .map_err(|err| {
            match err {
                Err::Error(verberr) => {
                    let errstr = nom::error::convert_error::<&str>(&body, verberr);
                    format!("{}: script format:\n... {}", filename, errstr)
                },
                Err::Failure(verberr) => {
                    let errstr = nom::error::convert_error::<&str>(&body, verberr);
                    format!("{}: script format:\n... {}", filename, errstr)
                },
                Err::Incomplete(_) => {
                    format!("{}: incomplete parse", filename)
                },
            }
        })?;
    
    Ok(res.1)
}
