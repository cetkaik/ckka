mod header;
mod pekzep_numeral;

#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;

type CKKA = (header::Header, Body);

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Body(Vec<BodyElem>);

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum BodyElem {
    Move(movement::Move),
    CaptureComment(cetkaik_core::Profession),
    TaXotTyMok(HandCreation, Action),
    SeasonEnd(Season),
    GameEnd,
}
use nom::combinator::eof;

pub fn parse_body_elem(s: &str) -> IResult<&str, BodyElem> {
    let (r, body_elem) = alt((
        map(movement::parse_movement, |m| BodyElem::Move(m)),
        map(parse_game_end, |_| BodyElem::GameEnd),
        map(parse_season_end, |a| BodyElem::SeasonEnd(a)),
        map(parse_ty_mok_ta_xot, |(a, b)| BodyElem::TaXotTyMok(a, b)),
        map(parse_capture_comment, |a| BodyElem::CaptureComment(a)),
    ))(s)?;
    let (no_used, _) = alt((
        map(many1(one_of("\t\r\n \u{00a0}\u{3000}")), |_| ()),
        map(eof, |_| ()),
    ))(r)?;

    Ok((no_used, body_elem))
}

pub mod movement;

use nom::character::complete::*;
use nom::multi::many0;
use nom::multi::many1;
use nom::IResult;

use std::collections::HashSet;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct HandCreation {
    pub player_name: String,
    pub hands: HashSet<String>,
}

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Season {
    Spring,
    Summer,
    Fall,
    Winter,
    Up,
    Down,
}

pub fn parse_game_end(s: &str) -> IResult<&str, ()> {
    let (rest, _) = tag("星一周")(s)?;
    Ok((rest, ()))
}

pub fn parse_season_end(s: &str) -> IResult<&str, Season> {
    let (rest, season) = alt((
        map(tag("春"), |_| Season::Spring),
        map(tag("夏"), |_| Season::Summer),
        map(tag("秋"), |_| Season::Fall),
        map(tag("冬"), |_| Season::Winter),
        map(tag("上季"), |_| Season::Up),
        map(tag("下季"), |_| Season::Down),
    ))(s)?;
    let (rest, _) = tag("終")(rest)?;

    Ok((rest, season))
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Action {
    TaXot(i64),
    TyMok,
}

/// ```
/// use cetkaik_kiaak::{parse_ty_mok_ta_xot, HandCreation, Action};
/// use cetkaik_core::Profession;
/// use cetkaik_core::absolute::*;
/// use std::collections::HashSet;
/// use std::iter::FromIterator;
/// assert_eq!(
///     parse_ty_mok_ta_xot("[SY]為(獣)(同色馬弓兵)再行"),
///     Ok((
///         "",
///         (HandCreation {
///             player_name: String::from("SY"),
///             hands: HashSet::from_iter(vec![String::from("獣"), String::from("同色馬弓兵")].into_iter())
///         }, Action::TyMok)
///     ))
/// );
/// assert_eq!(
///     parse_ty_mok_ta_xot("[SY]為(獣)(同色馬弓兵)終季 手十"),
///     Ok((
///         "",
///         (HandCreation {
///             player_name: String::from("SY"),
///             hands: HashSet::from_iter(vec![String::from("獣"), String::from("同色馬弓兵")].into_iter())
///         }, Action::TaXot(10))
///     ))
/// );
/// ```
pub fn parse_ty_mok_ta_xot(s: &str) -> IResult<&str, (HandCreation, Action)> {
    let (rest, hand_creation) = parse_hand_creation(s)?;
    let (rest, action) = alt((map(tag("再行"), |_| Action::TyMok), |s| {
        let (r, _) = tag("終季")(s)?;
        let (r, _) = many1(one_of("\t\r\n \u{00a0}\u{3000}"))(r)?;
        let (r, _) = tag("手")(r)?;
        let (r, num) = header::parse_pekzep_numeral(r)?;
        Ok((r, Action::TaXot(num)))
    }))(rest)?;

    Ok((rest, (hand_creation, action)))
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
    let (rest, hands) = many1(|s| header::parse_braced_string(s, '(', ')'))(rest)?;

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

    let parsed_head = match header::header_parser(&header) {
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

pub fn skip_spaces_and_newlines(s: &str) -> IResult<&str, ()> {
    let (no_used, _) = many0(one_of("\t\r\n \u{00a0}\u{3000}"))(s)?;
    Ok((no_used, ()))
}

pub fn parse_body(s: &str) -> IResult<&str, Vec<BodyElem>> {
    let (rest, _) = skip_spaces_and_newlines(s)?;
    let (rest, vec) = many0(parse_body_elem)(rest)?;

    Ok((rest, vec))
}
#[cfg(test)]
mod tests;
