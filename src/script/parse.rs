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


pub fn load_script(filename: &str) -> Result<(), String> {
    let body = std::fs::read_to_string(filename)
        .map_err(|err| {
            format!("{}: {}", filename, err.to_string())
        })?;

    let res = parse_comment::<nom::error::VerboseError<&str>>(&body);
    // res: Result<(&str, ()), nom::Err<VerboseError<&str>>>
    
    if let Err(err) = res {
        match err {
            nom::Err::Error(verberr) => {
                let errstr = nom::error::convert_error::<&str>(&body, verberr);
                return Err(format!("{}: script format:\n... {}", filename, errstr));
            },
            nom::Err::Failure(verberr) => {
                let errstr = nom::error::convert_error::<&str>(&body, verberr);
                return Err(format!("{}: script format:\n... {}", filename, errstr));
            },
            nom::Err::Incomplete(_) => {
                return Err(format!("{}: incomplete parse", filename));
            },
        }
    }

    Ok(())
}
