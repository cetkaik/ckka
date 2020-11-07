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

mod pekzep_numeral;

pub fn parse_pekzep_numeral(s: &str) -> IResult<&str, i64> {
    let (no_used, vec) =
        many_m_n(1, 1000, one_of("無下一二三四五六七八九十百万億"))(s)?;
    match pekzep_numeral::analyze(&vec) {
        Some(n) => Ok((no_used, n)),
        None => panic!("unparsable pekzep numeral `{}`", s),
    }
}

pub fn parse_braced_string(s: &str, open: char, close: char) -> IResult<&str, &str> {
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

    let (no_used, _) = many0(one_of("\t\r\n \u{00a0}\u{3000}"))(no_used)?;

    Ok((no_used, in_string))
}

pub fn header_elem_parser(s: &str) -> IResult<&str, HeaderElem> {
    let (no_used, in_string) = parse_braced_string(s, '{', '}')?;
    Ok((no_used, {
        let mut splitter = in_string.splitn(2, ':');
        let first = splitter.next().unwrap();
        let second = splitter.next();
        match (first, second) {
            ("", Some(val)) => HeaderElem::Value(val.to_owned()),
            (key, Some(value)) => HeaderElem::KeyedValue(key.to_owned(), value.to_owned()),
            (val, None) => HeaderElem::Value(val.to_owned()),
        }
    }))
}

pub fn player_and_point_parser(s: &str) -> IResult<&str, (String, Option<i64>)> {
    // TODO implement parsing point
    let (no_used, player_name) = parse_braced_string(s, '[', ']')?;
    let (no_used, _) = many0(one_of("\t\r\n \u{00a0}\u{3000}"))(no_used)?;
    let (no_used, v) = many_m_n(0, 1, parse_pekzep_numeral)(no_used)?;
    let (no_used, _) = many0(one_of("\t\r\n \u{00a0}\u{3000}"))(no_used)?;
    Ok((
        no_used,
        (
            player_name.to_owned(),
            match &v.as_slice() {
                &[] => None,
                &[num] => Some(*num),
                _ => unreachable!(),
            },
        ),
    ))
}

pub fn header_parser(s: &str) -> IResult<&str, Header> {
    let (no_used, _) = many0(one_of("\t\r\n \u{00a0}\u{3000}"))(s)?;
    let (no_used, info) = many0(header_elem_parser)(no_used)?;

    let (no_used, vec2) = many_m_n(0, 2, player_and_point_parser)(no_used)?;

    let players = match &vec2.as_slice() {
        &[] => None,
        &[q, r] => match (q.clone(), r.clone()) {
            ((a, Some(b)), (c, Some(d))) => Some((
                PlayerAndPoint {
                    player_name: a,
                    point: b,
                },
                PlayerAndPoint {
                    player_name: c,
                    point: d,
                },
            )),

            ((a, Some(b)), (c, None)) => Some((
                PlayerAndPoint {
                    player_name: a,
                    point: b,
                },
                PlayerAndPoint {
                    player_name: c,
                    point: 40 - b,
                },
            )),

            ((a, None), (c, Some(d))) => Some((
                PlayerAndPoint {
                    player_name: a,
                    point: 40 - d,
                },
                PlayerAndPoint {
                    player_name: c,
                    point: d,
                },
            )),

            ((a, None), (c, None)) => Some((
                PlayerAndPoint {
                    player_name: a,
                    point: 20,
                },
                PlayerAndPoint {
                    player_name: c,
                    point: 20,
                },
            )),
        },
        _ => panic!("only one player found!"),
    };

    Ok((no_used, Header { info, players }))
}

#[cfg(test)]
mod tests;
