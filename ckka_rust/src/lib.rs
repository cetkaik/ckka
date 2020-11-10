use nom::character::complete::*;
use nom::multi::many_m_n;
use nom::IResult;

mod pekzep_numeral;

#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;

use cetkaik_core::absolute;

type CKKA = (header::Header, String);

use nom::branch::alt;
use nom::combinator::map;

/// Examples:
/// ```
/// use ckka_rust::parse_tam_sqbracket;
/// use cetkaik_core::absolute;
/// assert_eq!(parse_tam_sqbracket("[TY]"), Ok(("", Some((absolute::Row::Y, absolute::Column::T)))));
/// assert_eq!(parse_tam_sqbracket("[或]"), Ok(("", None)))
/// ```
///
pub fn parse_tam_sqbracket(s: &str) -> IResult<&str, Option<absolute::Coord>> {
    let (rest, _) = char('[')(s)?;
    let (rest, opt_coord) =
        alt((map(parse_square, |a| Some(a)), map(char('或'), |_| None)))(rest)?;
    let (rest, _) = char(']')(rest)?;

    Ok((rest, opt_coord))
}

/// Examples:
/// ```
/// use ckka_rust::parse_profession;
/// use cetkaik_core::Profession;
/// assert_eq!(parse_profession("船"), Ok(("", Profession::Nuak1)));
/// assert_eq!(parse_profession("巫"), Ok(("", Profession::Tuk2)))
/// ```
///
pub fn parse_profession(s: &str) -> IResult<&str, cetkaik_core::Profession> {
    use std::str::FromStr;
    let (rest, prof) = one_of("船兵弓車虎馬筆巫将王")(s)?;
    let prof = cetkaik_core::Profession::from_str(&prof.to_string()).unwrap();
    Ok((rest, prof))
}

/// Examples:
/// ```
/// use ckka_rust::parse_profession_or_wildcard;
/// use cetkaik_core::Profession;
/// assert_eq!(parse_profession_or_wildcard("船"), Ok(("", Some(Profession::Nuak1))));
/// assert_eq!(parse_profession_or_wildcard("巫"), Ok(("", Some(Profession::Tuk2))));
/// assert_eq!(parse_profession_or_wildcard("片"), Ok(("", None)))
/// ```
///
pub fn parse_profession_or_wildcard(s: &str) -> IResult<&str, Option<cetkaik_core::Profession>> {
    use std::str::FromStr;
    let (rest, prof) = one_of("船兵弓車虎馬筆巫将王片")(s)?;
    if prof == '片' {
        Ok((rest, None))
    } else {
        let prof = cetkaik_core::Profession::from_str(&prof.to_string()).unwrap();
        Ok((rest, Some(prof)))
    }
}

pub fn parse_bridge_stick(s: &str) -> IResult<&str, Option<i32>> {
    let (rest, _) = char('橋')(s)?;
    let (rest, size) = one_of("或無一二三四五")(rest)?;
    Ok((
        rest,
        match size {
            '或' => None,
            '無' => Some(0),
            '一' => Some(1),
            '二' => Some(2),
            '三' => Some(3),
            '四' => Some(4),
            '五' => Some(5),
            _ => unreachable!(),
        },
    ))
}

pub fn parse_water_stick(s: &str) -> IResult<&str, (Option<i32>, bool)> {
    let (rest, _) = char('水')(s)?;
    let (rest, vec) = many_m_n(1, 3, one_of("或無一二三四五此"))(rest)?;

    let result = match vec.as_slice() {
        ['無', '此', '無'] => (Some(0), false),
        ['一', '此', '無'] => (Some(1), false),
        ['二', '此', '無'] => (Some(2), false),
        ['三'] => (Some(3), true),
        ['四'] => (Some(4), true),
        ['五'] => (Some(5), true),
        ['或'] => (None, true), /* unspecified but successful */
        ['或', '此', '無'] => (None, false), /* unspecified but not successful */
        _ => panic!(
            "Unparsable fragment {:?} while parsing water stick",
            vec.into_iter().collect::<String>()
        ),
    };

    Ok((rest, result))
}

pub fn parse_square(s: &str) -> IResult<&str, absolute::Coord> {
    let (rest, column) = one_of("KLNTZXCMP")(s)?;
    let (rest, row) = many_m_n(1, 2, one_of("AEIOUY"))(rest)?;
    Ok((
        rest,
        absolute::parse_coord(&format!(
            "{}{}",
            column,
            row.into_iter().collect::<String>()
        ))
        .unwrap(),
    ))
}

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
