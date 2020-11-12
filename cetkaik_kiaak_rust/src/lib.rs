#![warn(clippy::pedantic)]
#![allow(clippy::non_ascii_literal)]
#![allow(clippy::missing_errors_doc)]
pub mod header;
mod pekzep_numeral;

#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;

use body::{parse_body_elem, Body};

type CKKA = (header::Header, Body);

pub mod body;

use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::char;
use nom::character::complete::one_of;
use nom::error::{Error, ErrorKind};
use nom::multi::many0;
use nom::multi::many1;
use nom::Err;
use nom::IResult;

fn parse_braced_string(s: &str, open: char, close: char) -> IResult<&str, &str> {
    let (no_used, vec) = many0(char('#'))(s)?;
    let (no_used, _) = char(open)(no_used)?;

    // `}####` if vec.len() == 4
    let end_pattern = format!(
        "{}{}",
        close,
        (0..vec.len()).map(|_| "#").collect::<String>()
    );
    let (no_used, in_string) = take_until(&*end_pattern)(no_used)?;
    let (no_used, _) = tag(&*end_pattern)(no_used)?;

    let (no_used, _) = skip_spaces_and_newlines(no_used)?;

    if in_string.contains('\n') || in_string.contains('\r') {
        return Err(Err::Error(Error::new(no_used, ErrorKind::Verify)));
        /* neither key nor value in the header can contain a newline */
    }

    Ok((no_used, in_string))
}

pub fn parse_pekzep_numeral(s: &str) -> IResult<&str, i64> {
    let (no_used, vec) = many1(one_of("無下一二三四五六七八九十百万億"))(s)?;
    match pekzep_numeral::analyze(&vec) {
        Some(n) => Ok((no_used, n)),
        None => Err(Err::Error(Error::new(no_used, ErrorKind::Verify))), /* unparsable pekzep numeral */
    }
}

pub fn parse_header(input: &str) -> IResult<&str, header::Header> {
    header::parse(input)
}

pub fn parse_ckka(s: &str) -> Result<CKKA, String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r#"^\s*[KLNTZXCMP"]"#).unwrap();
    }

    let mut header = String::new();
    let mut body = String::new();
    let mut is_body = false;
    for l in s.lines() {
        if RE.is_match(l) {
            is_body = true;
        }

        if is_body {
            body.push_str(&l);
            body.push('\n');
        } else {
            header.push_str(&l);
            header.push('\n');
        }
    }

    let parsed_head = match header::parse(&header) {
        Ok(("", parsed_head)) => parsed_head,
        Ok((a, _)) => {
            return Err(format!(
                "Unparsable fragment `{}` left while parsing header",
                a
            ))
        }
        Err(e) => return Err(format!("Failed to parse header, with error `{:?}`", e)),
    };

    let parsed_body = match parse_body(&body) {
        Ok(("", parsed_body)) => parsed_body,
        Ok((a, _)) => {
            return Err(format!(
                "Unparsable fragment `{}` left while parsing body",
                a
            ))
        }
        Err(e) => return Err(format!("Failed to parse header, with error `{:?}`", e)),
    };

    Ok((parsed_head, parsed_body))
}

fn skip_spaces_and_newlines(s: &str) -> IResult<&str, ()> {
    let (no_used, _) = many0(one_of("\t\r\n \u{00a0}\u{3000}"))(s)?;
    Ok((no_used, ()))
}

pub fn parse_body(s: &str) -> IResult<&str, Body> {
    let (rest, _) = skip_spaces_and_newlines(s)?;
    let (rest, vec) = many0(parse_body_elem)(rest)?;

    Ok((rest, Body(vec)))
}
#[cfg(test)]
mod tests;
