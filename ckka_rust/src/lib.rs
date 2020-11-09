/*use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::*;
use nom::multi::many0;
use nom::multi::many_m_n;
use nom::IResult;*/

mod pekzep_numeral;

#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;

type CKKA = (header::Header, String);

pub fn parse_ckka(s: &str) -> Result<CKKA, ()> {
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

    match header::header_parser(&header) {
        Ok(("", parsed_head)) => Ok((parsed_head, body)),
        Ok((a, _)) => panic!("Unparsable fragment `{}` left", a),
        Err(_) => Err(()),
    }
}
mod header;
#[cfg(test)]
mod tests;
