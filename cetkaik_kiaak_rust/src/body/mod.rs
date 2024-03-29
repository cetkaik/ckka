use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, one_of};
use nom::combinator::map;
use nom::multi::many1;
use nom::IResult;
use std::collections::HashSet;

pub mod movement;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Body(pub Vec<Elem>);

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Elem {
    Move(movement::Move),
    CaptureComment(cetkaik_core::Profession),
    TaXotTyMok(HandCreation, Action),
    SeasonEnd(Season),
    GameEnd,
}

use nom::combinator::eof;

pub fn parse_body_elem(s: &str) -> IResult<&str, Elem> {
    let (r, body_elem) = alt((
        map(movement::parse, Elem::Move),
        map(parse_game_end, |_| Elem::GameEnd),
        map(parse_season_end, Elem::SeasonEnd),
        map(parse_ty_mok_ta_xot, |(a, b)| Elem::TaXotTyMok(a, b)),
        map(parse_capture_comment, Elem::CaptureComment),
    ))(s)?;
    let (no_used, _) = alt((
        map(many1(one_of("\t\r\n \u{00a0}\u{3000}")), |_| ()),
        map(eof, |_| ()),
    ))(r)?;

    Ok((no_used, body_elem))
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum HandAuthor {
    Unnamed,
    Name(String),
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct HandCreation {
    pub player_name: HandAuthor,
    pub hands: HashSet<String>,
}

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

pub fn parse_capture_comment(s: &str) -> IResult<&str, cetkaik_core::Profession> {
    let (rest, _) = char('手')(s)?;
    let (rest, prof) = movement::parse_profession(rest)?;

    Ok((rest, prof))
}

/// ```
/// use cetkaik_kiaak::body::{parse_ty_mok_ta_xot, HandCreation, Action, HandAuthor};
/// use cetkaik_core::Profession;
/// use cetkaik_core::absolute::*;
/// use std::collections::HashSet;
/// use std::iter::FromIterator;
/// assert_eq!(
///     parse_ty_mok_ta_xot("[SY]為(獣)(同色馬弓兵)再行"),
///     Ok((
///         "",
///         (HandCreation {
///             player_name: HandAuthor::Name(String::from("SY")),
///             hands: HashSet::from_iter(vec![String::from("獣"), String::from("同色馬弓兵")].into_iter())
///         }, Action::TyMok)
///     ))
/// );
/// assert_eq!(
///     parse_ty_mok_ta_xot("[SY]為(獣)(同色馬弓兵)終季 手十"),
///     Ok((
///         "",
///         (HandCreation {
///             player_name: HandAuthor::Name(String::from("SY")),
///             hands: HashSet::from_iter(vec![String::from("獣"), String::from("同色馬弓兵")].into_iter())
///         }, Action::TaXot(10))
///     ))
/// );
/// assert_eq!(
///     parse_ty_mok_ta_xot("[SY]為(獣)(同色馬弓兵)終季 手20"),
///     Ok((
///         "",
///         (HandCreation {
///             player_name: HandAuthor::Name(String::from("SY")),
///             hands: HashSet::from_iter(vec![String::from("獣"), String::from("同色馬弓兵")].into_iter())
///         }, Action::TaXot(20))
///     ))
/// );
/// ```
pub fn parse_ty_mok_ta_xot(s: &str) -> IResult<&str, (HandCreation, Action)> {
    let (rest, hand_creation) = parse_hand_creation(s)?;
    let (rest, action) = alt((map(tag("再行"), |_| Action::TyMok), |s| {
        let (r, _) = tag("終季")(s)?;
        let (r, _) = many1(one_of("\t\r\n \u{00a0}\u{3000}"))(r)?;
        let (r, _) = tag("手")(r)?;
        let (r, num) = super::parse_numeral(r)?;
        Ok((r, Action::TaXot(num)))
    }))(rest)?;

    Ok((rest, (hand_creation, action)))
}

/// ```
/// use cetkaik_kiaak::body::{parse_hand_creation, HandCreation, HandAuthor};
/// use cetkaik_core::Profession;
/// use cetkaik_core::absolute::*;
/// use std::collections::HashSet;
/// use std::iter::FromIterator;
/// assert_eq!(
///     parse_hand_creation("[SY]為(獣)(同色馬弓兵)"),
///     Ok((
///         "",
///         HandCreation {
///             player_name: HandAuthor::Name(String::from("SY")),
///             hands: HashSet::from_iter(vec![String::from("獣"), String::from("同色馬弓兵")].into_iter())
///         }
///     ))
/// );
///
/// assert_eq!(
///     parse_hand_creation("或為(馬弓兵)(王)(戦集)(助友)而手十六"),
///     Ok((
///         "而手十六",
///         HandCreation {
///             player_name: HandAuthor::Unnamed,
///             hands: HashSet::from_iter(vec![
///                 String::from("馬弓兵"),
///                 String::from("王"),
///                 String::from("戦集"),
///                 String::from("助友"),
///             ].into_iter())
///         }
///     ))
/// );
/// ```
pub fn parse_hand_creation(s: &str) -> IResult<&str, HandCreation> {
    let (rest, player_name) = alt((
        map(tag("或"), |_| crate::body::HandAuthor::Unnamed),
        map(
            |s| super::parse_braced_string(s, '[', ']'),
            |name| crate::body::HandAuthor::Name(name.to_owned()),
        ),
    ))(s)?;
    let (rest, _) = char('為')(rest)?;
    let (rest, hands) = many1(|s| super::parse_braced_string(s, '(', ')'))(rest)?;

    Ok((
        rest,
        HandCreation {
            player_name,
            hands: hands
                .into_iter()
                .map(std::borrow::ToOwned::to_owned)
                .collect(),
        },
    ))
}
