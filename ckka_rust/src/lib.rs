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

pub fn parse_header(s: &str) -> Header {
    unimplemented!()
}

use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::*;
use nom::multi::many0;
use nom::IResult;

pub fn header_elem_parser(s: &str) -> IResult<&str, HeaderElem> {
    let (no_used, vec) = many0(char('#'))(s)?;
    let (no_used, _) = char('{')(no_used)?;

    // `}####` if vec.len() == 4
    let end_pattern = format!("}}{}", (0..vec.len()).map(|_| "#").collect::<String>());
    let (no_used, in_string) = take_until(&*end_pattern)(no_used)?;
    let (no_used, _) = tag(&*end_pattern)(no_used)?;

    let ans = {
        let mut splitter = in_string.splitn(2, ':');
        let first = splitter.next().unwrap();
        let second = splitter.next();
        match (first, second) {
            (key, Some(value)) => HeaderElem::KeyedValue(key.to_owned(), value.to_owned()),
            (val, None) => HeaderElem::Value(val.to_owned()),
        }
    };
    Ok((no_used, ans))
}

#[cfg(test)]
mod tests {
    use super::*;
    use big_s::S;
    #[test]
    fn header_elem() {
        assert_eq!(
            header_elem_parser("{律:硬皇力}"),
            Ok(("", HeaderElem::KeyedValue(S("律"), S("硬皇力"))))
        )
    }

    #[test]
    fn header_elem_2() {
        assert_eq!(
            header_elem_parser("{硬皇力}"),
            Ok(("", HeaderElem::Value(S("硬皇力"))))
        )
    }

    #[test]
    fn header_elem_3() {
        assert_eq!(
            header_elem_parser("#{硬皇力}#"),
            Ok(("", HeaderElem::Value(S("硬皇力"))))
        )
    }

    #[test]
    fn header_elem_4() {
        assert_eq!(
            header_elem_parser("#{硬皇}力}#"),
            Ok(("", HeaderElem::Value(S("硬皇}力"))))
        )
    }

    #[test]
    fn it_works() {
        assert_eq!(
            parse_header(
                r#"{律:硬皇力}
{2018年4月8日 18:00頃}
[SY] [補集合]"#
            ),
            Header {
                info: vec![
                    HeaderElem::KeyedValue(S("律"), S("硬皇力")),
                    HeaderElem::Value(S("2018年4月8日 18:00頃"))
                ],
                players: Some((
                    PlayerAndPoint {
                        player_name: S("SY"),
                        point: 20
                    },
                    PlayerAndPoint {
                        player_name: S("補集合"),
                        point: 20
                    }
                ))
            }
        )
    }
}
