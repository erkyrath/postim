use nom::IResult;
use nom::Err;
use nom::error::ParseError;

use nom::{
    combinator::value,
    combinator::eof,
    sequence::pair,
    sequence::terminated,
    branch::alt,
    multi::many0,
    bytes::complete::is_not,
    character::complete::char,
    character::complete::multispace1,
};

#[derive(Debug, Clone)]
pub enum ScriptToken {
    Whitespace,
    Comment,
}

pub fn parse_comment<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, ScriptToken, E> {
    value(
        ScriptToken::Comment,
        pair(char('#'), is_not("\n\r"))
    )(input)
}

pub fn parse_whitespace<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, ScriptToken, E> {
    value(
        ScriptToken::Whitespace,
        multispace1
    )(input)
}

pub fn parse_anytoken<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, ScriptToken, E> {
    alt((
        parse_comment,
        parse_whitespace
    ))(input)
}

pub fn parse_anytokenlist<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Vec<ScriptToken>, E> {
    terminated(
        many0(parse_anytoken),
        eof
    )(input)
}

pub fn load_script(filename: &str) -> Result<(), String> {
    let body = std::fs::read_to_string(filename)
        .map_err(|err| {
            format!("{}: {}", filename, err.to_string())
        })?;

    // parser returns Result<(&str, ScriptToken), nom::Err<VerboseError<&str>>>
    
    let res = parse_anytokenlist::<nom::error::VerboseError<&str>>(&body)
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

    println!("### parsed: {:?}", res.1);
    
    Ok(())
}
