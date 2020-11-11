#![warn(clippy::pedantic)]
#![allow(clippy::non_ascii_literal)]
#![allow(clippy::missing_errors_doc)]
mod header;
mod pekzep_numeral;

#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;

use body::{parse_body_elem, Body, Elem};

type CKKA = (header::Header, Body);

pub mod body;

use nom::character::complete::one_of;
use nom::multi::many0;
use nom::IResult;

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

    Ok((parsed_head, Body(parsed_body)))
}

fn skip_spaces_and_newlines(s: &str) -> IResult<&str, ()> {
    let (no_used, _) = many0(one_of("\t\r\n \u{00a0}\u{3000}"))(s)?;
    Ok((no_used, ()))
}

fn parse_body(s: &str) -> IResult<&str, Vec<Elem>> {
    let (rest, _) = skip_spaces_and_newlines(s)?;
    let (rest, vec) = many0(parse_body_elem)(rest)?;

    Ok((rest, vec))
}
#[cfg(test)]
mod tests;
