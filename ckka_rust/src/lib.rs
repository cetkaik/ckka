#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Header {
    info: Vec<HeaderElem>,
    players: Option<(PlayerAndPoint, PlayerAndPoint)>,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct PlayerAndPoint {
    player_name: String,
    point: i64,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum HeaderElem {
    Value(String),
    KeyedValue(String, String),
}

use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::*;
use nom::multi::many0;
use nom::multi::many_m_n;
use nom::IResult;

use std::collections::HashMap;

pub fn parse_braced_string(
    s: &str,
    open_close: HashMap<char, char>,
) -> IResult<&str, (&str, (char, char))> {
    let (no_used, vec) = many0(char('#'))(s)?;
    let (no_used, open) = one_of(&*open_close.keys().collect::<String>())(no_used)?;

    let close = *open_close.get(&open).unwrap();

    // `}####` if vec.len() == 4
    let end_pattern = format!(
        "{}{}",
        close,
        (0..vec.len()).map(|_| "#").collect::<String>()
    );
    let (no_used, in_string) = take_until(&*end_pattern)(no_used)?;
    let (no_used, _) = tag(&*end_pattern)(no_used)?;

    let (no_used, _) = many0(one_of("\t\r\n \u{00a0}\u{3000}"))(no_used)?;

    Ok((no_used, (in_string, (open, close))))
}

pub fn header_elem_parser(s: &str) -> IResult<&str, HeaderElem> {
    let (no_used, (in_string, (open, close))) =
        parse_braced_string(s, maplit::hashmap! {'{' => '}'})?;
    assert_eq!(open, '{');
    assert_eq!(close, '}');
    Ok((no_used, {
        let mut splitter = in_string.splitn(2, ':');
        let first = splitter.next().unwrap();
        let second = splitter.next();
        match (first, second) {
            (key, Some(value)) => HeaderElem::KeyedValue(key.to_owned(), value.to_owned()),
            (val, None) => HeaderElem::Value(val.to_owned()),
        }
    }))
}

pub fn player_and_point_parser(s: &str) -> IResult<&str, PlayerAndPoint> {
    // TODO implement parsing point
    let (no_used, (player_name, (open, close))) =
        parse_braced_string(s, maplit::hashmap! {'[' => ']'})?;
    assert_eq!(open, '[');
    assert_eq!(close, ']');

    Ok((
        no_used,
        PlayerAndPoint {
            player_name: player_name.to_owned(),
            point: 20, /* FIXME */
        },
    ))
}

pub fn header_parser(s: &str) -> IResult<&str, Header> {
    let (no_used, _) = many0(one_of("\t\r\n \u{00a0}\u{3000}"))(s)?;
    let (no_used, info) = many0(header_elem_parser)(no_used)?;

    let (no_used, vec2) = many_m_n(0, 2, player_and_point_parser)(no_used)?;

    let players = match &vec2.as_slice() {
        &[] => None,
        &[a, b] => Some((a.clone(), b.clone())),
        _ => panic!("only one player!"),
    };

    Ok((no_used, Header { info, players }))
}

#[cfg(test)]
mod tests;
