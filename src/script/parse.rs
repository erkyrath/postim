use nom::IResult;
use nom::Err;
use nom::error::ParseError;
use nom::error::VerboseError;

use nom::{
    combinator::value,
    combinator::opt,
    combinator::recognize,
    combinator::map,
    combinator::eof,
    sequence::pair,
    sequence::terminated,
    branch::alt,
    multi::many0,
    bytes::complete::is_not,
    character::complete::char,
    character::complete::digit1,
    character::complete::multispace1,
};

#[derive(Debug, Clone)]
pub enum ScriptToken {
    Whitespace,
    Comment,
    Integer(i32),
    Float(f32),
    BadToken(String),
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

pub fn parse_integer<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, ScriptToken, E> {
   let (pinput, pstr) = recognize(
       pair(
           opt(char('-')),
           digit1
       )
   )(input)?;

   if let Ok(ival) = i32::from_str_radix(pstr, 10) {
       return Ok( (pinput, ScriptToken::Integer(ival)) );
   }
   else {
       //return Err(nom::Err::Failure(E::from_error_kind(pinput, nom::error::ErrorKind::Fail)));
       return nom::combinator::fail(input);
   }
}

pub fn parse_float<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, ScriptToken, E> {
    map(
        nom::number::complete::float,
        |val: f32| ScriptToken::Float(val)
    )(input)
}

pub fn parse_anytoken<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, ScriptToken, E> {
    alt((
        parse_comment,
        parse_whitespace,
        parse_integer,
        parse_float,
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
    
    let res = parse_anytokenlist::<VerboseError<&str>>(&body)
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
