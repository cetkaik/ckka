#[warn(clippy::pedantic)]
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Header {
    pub info: Vec<Elem>,
    pub players: Option<(PlayerAndPoint, PlayerAndPoint)>,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct PlayerAndPoint {
    pub player_name: String,
    pub point: i64,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Elem {
    Value(String),
    KeyedValue(String, String),
}

use nom::combinator::opt;
use nom::error::{Error, ErrorKind};
use nom::multi::many0;
use nom::multi::many_m_n;
use nom::Err;
use nom::IResult;

fn elem_parser(s: &str) -> IResult<&str, Elem> {
    let (no_used, in_string) = super::parse_braced_string(s, '{', '}')?;
    Ok((no_used, {
        let mut splitter = in_string.splitn(2, ':');
        let first = splitter.next().unwrap();
        let second = splitter.next();
        match (first, second) {
            ("", Some(val)) | (val, None) => Elem::Value(val.to_owned()),
            (key, Some(value)) => Elem::KeyedValue(key.to_owned(), value.to_owned()),
        }
    }))
}

use super::skip_spaces_and_newlines;

fn player_and_point_parser(s: &str) -> IResult<&str, (String, Option<i64>)> {
    let (no_used, player_name) = super::parse_braced_string(s, '[', ']')?;
    let (no_used, _) = skip_spaces_and_newlines(no_used)?;
    let (no_used, opt_num) = opt(super::parse_pekzep_numeral)(no_used)?;
    let (no_used, _) = skip_spaces_and_newlines(no_used)?;
    Ok((no_used, (player_name.to_owned(), opt_num)))
}

pub fn parse(input: &str) -> IResult<&str, Header> {
    let (no_used, _) = skip_spaces_and_newlines(input)?;
    let (no_used, info) = many0(elem_parser)(no_used)?;
    let (no_used, vec2) = many_m_n(0, 2, player_and_point_parser)(no_used)?;
    let players = match vec2.as_slice() {
        [] => None,
        [q, r] => {
            let (n1, p1) = q.clone();
            let (n2, p2) = r.clone();
            let (p1, p2) = match (p1, p2) {
                (Some(b), Some(d)) => (b, d),
                (Some(b), None) => (b, 40 - b),
                (None, Some(d)) => (40 - d, d),
                (None, None) => (20, 20),
            };

            Some((
                PlayerAndPoint {
                    player_name: n1,
                    point: p1,
                },
                PlayerAndPoint {
                    player_name: n2,
                    point: p2,
                },
            ))
        }
        _ => return Err(Err::Error(Error::new(no_used, ErrorKind::Verify))), /* only one player found */
    };

    Ok((no_used, Header { info, players }))
}

#[cfg(test)]
mod tests_;
