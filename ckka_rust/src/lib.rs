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

#[cfg(test)]
mod tests {
    use big_s::S;
    use super::*;
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
