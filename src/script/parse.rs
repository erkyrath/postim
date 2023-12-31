use std::rc::Rc;

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

// Nom parser docs: https://docs.rs/nom/latest/nom/

fn parse_comment<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, ScriptToken, E> {
    combinator::value(
        ScriptToken::Comment,
        sequence::pair(
            character::complete::char('#'),
            bytes::complete::take_till(|ch| ch == '\n' || ch == '\r')
        )
    )(input)
}

fn parse_whitespace<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, ScriptToken, E> {
    combinator::value(
        ScriptToken::Whitespace,
        character::complete::multispace1
    )(input)
}

fn parse_string<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, ScriptToken, E> {
    combinator::map(
        // for escaped chars, see https://github.com/rust-bakery/nom/blob/main/examples/string.rs
        sequence::delimited(
            character::complete::char('"'),
            combinator::opt(bytes::complete::is_not("\"")),
            character::complete::char('"')
        ),
        |res: Option<&str>| {
            if let Some(val) = res {
                ScriptToken::String(val.to_string())
            }
            else {
                ScriptToken::String(String::default())
            }
        }
    )(input)
}

fn parse_tokterminator<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &str, E> {
    branch::alt((
        combinator::eof,
        character::complete::multispace1,
        bytes::complete::take_while1(|ch: char| ch == '#' || ch == '-' || ch == '+' || ch == '*' || ch == '/' || ch == '=' || ch == '<' || ch == '>' || ch == '&' || ch == '|' || ch == '}' || ch == '{' || ch == ']' || ch == '[')
    ))(input)
}

fn parse_name<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, ScriptToken, E> {
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

fn parse_operator<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, ScriptToken, E> {
    combinator::map(
        combinator::recognize(
            multi::many1(
                character::complete::one_of("+-*/<>%&|=!")
            )
        ),
        |val: &str| ScriptToken::Operator(val.to_string())
    )(input)
}

fn parse_delimiter<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, ScriptToken, E> {
    combinator::map(
        combinator::recognize(
            character::complete::one_of("{}[]")
        ),
        |val: &str| ScriptToken::Delimiter(val.to_string())
    )(input)
}

fn parse_integer<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, ScriptToken, E> {
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

fn parse_float<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, ScriptToken, E> {
    combinator::map(
        sequence::terminated(
            nom::number::complete::float,
            combinator::peek(parse_tokterminator)
        ),
        |val: f32| ScriptToken::Float(val)
    )(input)
}

fn parse_size<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, ScriptToken, E> {
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

fn parse_color<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, ScriptToken, E> {
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

fn parse_anytoken<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, ScriptToken, E> {
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
        parse_delimiter,
    ))(input)
}

fn parse_anytokenlist<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Vec<ScriptToken>, E> {
    multi::many0(
        parse_anytoken
    )(input)
}

fn parse_with_termination<'a, R, F, E: ParseError<&'a str>>(input: &'a str, func: F) -> IResult<&'a str, R, E>
where F: Fn(&'a str) -> IResult<&'a str, R, E> {
    sequence::terminated(
        func,
        combinator::eof
    )(input)
}

pub fn load_script_text(body: &str) -> Result<Script, String> {
    load_script(body, "<ARG>")
}

pub fn load_script_file(filename: &str) -> Result<Script, String> {
    let body = std::fs::read_to_string(filename)
        .map_err(|err| {
            format!("{}: {}", filename, err.to_string())
        })?;

    load_script(&body, filename)
}

fn load_script(body: &str, source: &str) -> Result<Script, String> {
    // parser returns Result<(&str, Vec<ScriptToken>), nom::Err<VerboseError<&str>>>
    
    let (_, rawtokens): (_, Vec<ScriptToken>) = parse_with_termination::<_, _, VerboseError<&str>>(&body, parse_anytokenlist)
        .map_err(|err| {
            match err {
                Err::Error(verberr) => {
                    let errstr = nom::error::convert_error::<&str>(&body, verberr);
                    format!("{}: script format:\n... {}", source, errstr)
                },
                Err::Failure(verberr) => {
                    let errstr = nom::error::convert_error::<&str>(&body, verberr);
                    format!("{}: script format:\n... {}", source, errstr)
                },
                Err::Incomplete(_) => {
                    format!("{}: incomplete parse", source)
                },
            }
        })?;

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
                    return Err(format!("{}: arrow needs name, found {:?}", source, val));
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
                    return Err(format!("{}: arrow needs name, found {:?}", source, other));
                }
                tokens.push(other);
            },
        }
    }
    if wasarrow {
        return Err(format!("{}: arrow needs name", source));
    }

    fn buildwrap(iter: &mut std::vec::IntoIter<ScriptToken>, istop: bool) -> Result<Rc<Vec<ScriptToken>>, String> {
        let mut ls: Vec<ScriptToken> = Vec::new();
        while let Some(tok) = iter.next() {
            if let ScriptToken::Delimiter(delim) = tok {
                if delim == "}" {
                    if istop {
                        return Err(format!("unmatched close brace"));
                    }
                    return Ok(Rc::new(ls));
                }
                else if delim == "{" {
                    let proc = buildwrap(iter, false)?;
                    ls.push(ScriptToken::Proc(proc));
                    continue;
                }
                else if delim == "[" || delim == "]" {
                    ls.push(ScriptToken::Name(delim));
                    continue;
                }
                else {
                    return Err(format!("unknown delimiter: {}", delim));
                }
            }
            ls.push(tok);
        }
        if !istop {
            return Err(format!("unclosed open brace"));
        }
        Ok(Rc::new(ls))
    }
    let wrappedtokens = buildwrap(&mut tokens.into_iter(), true)?; // consume original

    Ok(Script::new(source, wrappedtokens))
}

pub fn match_color(body: &str) -> Option<(u8, u8, u8)>
{
    if let Ok((_, ScriptToken::Color(rval, gval, bval))) = parse_with_termination::<_, _, ()>(&body, parse_color) {
        Some((rval, gval, bval))
    }
    else {
        None
    }
}

pub fn match_size(body: &str) -> Option<(i32, i32)>
{
    if let Ok((_, ScriptToken::Size(width, height))) = parse_with_termination::<_, _, ()>(&body, parse_size) {
        Some((width, height))
    }
    else {
        None
    }
}

