use nom::IResult;
use nom::Err;
use nom::error::ParseError;
use nom::error::VerboseError;
use nom::error::ErrorKind;

use nom::sequence;
use nom::combinator;
use nom::multi;
use nom::branch;
use nom::bytes;
use nom::character;

use crate::script::Script;
use crate::script::ScriptToken;

pub fn parse_comment<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, ScriptToken, E> {
    combinator::value(
        ScriptToken::Comment,
        sequence::pair(
            character::complete::char('#'),
            bytes::complete::is_not("\n\r")
        )
    )(input)
}

pub fn parse_whitespace<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, ScriptToken, E> {
    combinator::value(
        ScriptToken::Whitespace,
        character::complete::multispace1
    )(input)
}

pub fn parse_string<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, ScriptToken, E> {
    combinator::map(
        // for escaped chars, see https://github.com/rust-bakery/nom/blob/main/examples/string.rs
        sequence::delimited(
            character::complete::char('"'),
            bytes::complete::is_not("\""),
            character::complete::char('"')
        ),
        |val: &str| ScriptToken::String(val.to_string())
    )(input)
}

pub fn parse_tokterminator<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &str, E> {
    branch::alt((
        combinator::eof,
        character::complete::multispace1,
        bytes::complete::take_while1(|ch: char| ch == '#' || ch == '-' || ch == '+' || ch == '*' || ch == '/' || ch == '=' || ch == '<' || ch == '>' || ch == '&' || ch == '|')
    ))(input)
}

pub fn parse_name<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, ScriptToken, E> {
    combinator::map(
        combinator::recognize(
            sequence::tuple((
                bytes::complete::take_while1(|ch: char| ch == '_' || ch.is_ascii_alphabetic()),
                bytes::complete::take_while(|ch: char| ch == '_' || ch.is_ascii_alphanumeric()),
                combinator::peek(parse_tokterminator)
            ))
        ),
        |val| ScriptToken::Name(val.to_string())
    )(input)
}

pub fn parse_operator<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, ScriptToken, E> {
    combinator::map(
        combinator::recognize(
            multi::many1(
                character::complete::one_of("+-*/<>%&|=!")
            )
        ),
        |val: &str| ScriptToken::Operator(val.to_string())
    )(input)
}

pub fn parse_integer<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, ScriptToken, E> {
   let (pinput, pstr) = combinator::recognize(
       sequence::tuple((
           combinator::opt(character::complete::char('-')),
           character::complete::digit1,
           combinator::peek(parse_tokterminator)
       ))
   )(input)?;

   let ival = i32::from_str_radix(pstr, 10).
       map_err(|_err| {
           Err::Failure(E::from_error_kind(pinput, ErrorKind::Fail))
       })?;
   return Ok( (pinput, ScriptToken::Integer(ival)) );
}

pub fn parse_float<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, ScriptToken, E> {
    combinator::map(
        sequence::terminated(
            nom::number::complete::float,
            combinator::peek(parse_tokterminator)
        ),
        |val: f32| ScriptToken::Float(val)
    )(input)
}

pub fn parse_size<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, ScriptToken, E> {
    let (pinput, (pstr1, pstr2)) =
    sequence::separated_pair(
        combinator::recognize(
            sequence::tuple((
                combinator::opt(character::complete::char('-')),
                character::complete::digit1,
            ))
        ),
        character::complete::char('x'),
        combinator::recognize(
            sequence::tuple((
                combinator::opt(character::complete::char('-')),
                character::complete::digit1,
                combinator::peek(parse_tokterminator)
            ))
        )
    )(input)?;
 
    let ival1 = i32::from_str_radix(pstr1, 10).
        map_err(|_err| {
            Err::Failure(E::from_error_kind(pinput, ErrorKind::Fail))
        })?;
    let ival2 = i32::from_str_radix(pstr2, 10).
        map_err(|_err| {
            Err::Failure(E::from_error_kind(pinput, ErrorKind::Fail))
        })?;
    return Ok( (pinput, ScriptToken::Size(ival1, ival2)) );
}

pub fn parse_color<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, ScriptToken, E> {
    let (pinput, pstr) = combinator::recognize(
        sequence::tuple((
            character::complete::char('$'),
            character::complete::hex_digit1,
            combinator::peek(parse_tokterminator)
        ))
    )(input)?;

    let phex = &pstr[1..];
    
    let uval = u32::from_str_radix(phex, 16).
        map_err(|_err| {
            Err::Failure(E::from_error_kind(pinput, ErrorKind::Fail))
        })?;

    if phex.len() == 6 {
        let tok = ScriptToken::Color(
            (uval >> 16 & 0xFF) as u8,
            (uval >> 8 & 0xFF) as u8,
            (uval & 0xFF) as u8);
        Ok( (pinput, tok) )
    }
    else if phex.len() == 3 {
        let tok = ScriptToken::Color(
            0x11 * (uval >> 8 & 0x0F) as u8,
            0x11 * (uval >> 4 & 0x0F) as u8,
            0x11 * (uval & 0x0F) as u8);
        Ok( (pinput, tok) )
    }
    else {
        Err(Err::Failure(E::from_error_kind(pinput, ErrorKind::Fail)))
    }
}

pub fn parse_anytoken<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, ScriptToken, E> {
    branch::alt((
        parse_comment,
        parse_whitespace,
        parse_string,
        parse_name,
        parse_integer,
        parse_float,
        parse_size,
        parse_color,
        parse_operator,
    ))(input)
}

pub fn parse_anytokenlist<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Vec<ScriptToken>, E> {
    sequence::terminated(
        multi::many0(parse_anytoken),
        combinator::eof
    )(input)
}

pub fn load_script(filename: &str) -> Result<Script, String> {
    let body = std::fs::read_to_string(filename)
        .map_err(|err| {
            format!("{}: {}", filename, err.to_string())
        })?;

    // parser returns Result<(&str, Vec<ScriptToken>), nom::Err<VerboseError<&str>>>
    
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

    let (_, rawtokens) = res;

    let mut tokens: Vec<ScriptToken> = Vec::new();
    let mut wasarrow = false;

    for tok in rawtokens {    // consume original
        match tok {
            ScriptToken::Whitespace => {},
            ScriptToken::Comment => {},
            ScriptToken::Operator(val) => {
                if val == ">>".to_string() {
                    wasarrow = true;
                }
                else if wasarrow {
                    return Err(format!("{}: arrow needs name, found {:?}", filename, val));
                }
                else {
                    tokens.push(ScriptToken::Name(val));
                }
            }
            ScriptToken::Name(val) => {
                if wasarrow {
                    wasarrow = false;
                    tokens.push(ScriptToken::StoreTo(val));
                }
                else {
                    tokens.push(ScriptToken::Name(val));
                }
            }
            other => {
                if wasarrow {
                    return Err(format!("{}: arrow needs name, found {:?}", filename, other));
                }
                tokens.push(other);
            },
        }
    }
    if wasarrow {
        return Err(format!("{}: arrow needs name", filename));
    }

    Ok(Script::new(filename, tokens))
}
