mod header;
mod pekzep_numeral;

#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;

type CKKA = (header::Header, String);

pub mod movement;

use nom::character::complete::*;
use nom::multi::many_m_n;
use nom::IResult;

use std::collections::HashSet;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct HandCreation {
    pub player_name: String,
    pub hands: HashSet<String>,
}

/// ```
/// use cetkaik_kiaak::{parse_hand_creation, HandCreation};
/// use cetkaik_core::Profession;
/// use cetkaik_core::absolute::*;
/// use std::collections::HashSet;
/// use std::iter::FromIterator;
/// assert_eq!(
///     parse_hand_creation("[SY]為(獣)(同色馬弓兵)"),
///     Ok((
///         "",
///         HandCreation {
///             player_name: String::from("SY"),
///             hands: HashSet::from_iter(vec![String::from("獣"), String::from("同色馬弓兵")].into_iter())
///         }
///     ))
/// );
/// ```
pub fn parse_hand_creation(s: &str) -> IResult<&str, HandCreation> {
    let (rest, player_name) = header::parse_braced_string(s, '[', ']')?;
    let (rest, _) = char('為')(rest)?;
    let (rest, hands) = many_m_n(1, 1000, |s| header::parse_braced_string(s, '(', ')'))(rest)?;

    Ok((
        rest,
        HandCreation {
            player_name: player_name.to_owned(),
            hands: hands.into_iter().map(|a| a.to_owned()).collect(),
        },
    ))
}

pub fn parse_capture_comment(s: &str) -> IResult<&str, cetkaik_core::Profession> {
    let (rest, _) = char('手')(s)?;
    let (rest, prof) = movement::parse_profession(rest)?;

    Ok((rest, prof))
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

    match header::header_parser(&header) {
        Ok(("", parsed_head)) => Ok((parsed_head, body)),
        Ok((a, _)) => Err(format!("Unparsable fragment `{}` left", a)),
        Err(e) => Err(format!("Failed to parse header, with error `{:?}`", e)),
    }
}
#[cfg(test)]
mod tests;
