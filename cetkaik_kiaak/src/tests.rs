use super::header::*;
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
fn it_works3() {
    assert_eq!(
        header_parser(
            r#"{:2018年4月8日 01:30頃}
[JV]二十一 [SY]十九"#
        ),
        Ok(("", Header { info: vec![
            HeaderElem::Value(S("2018年4月8日 01:30頃"))
        ], players: Some((
            PlayerAndPoint{ player_name: S("JV"), point: 21},
            PlayerAndPoint{ player_name: S("SY"), point: 19},
        )) }))
    )
}

#[test]
fn it_works2() {
    assert_eq!(
        header_parser(
            r#"{律:硬皇力}
{:2018年4月8日 18:00頃}
[SY] [補集合]"#,
        ),
        Ok((
            "",
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
        ))
    )
}

#[test]
fn it_works() {
    assert_eq!(
        header_parser(
            r#"{律:硬皇力}
{2018年4月8日 18:00頃}
[SY] [補集合]"#,
        ),
        Ok((
            "",
            Header {
                info: vec![
                    HeaderElem::KeyedValue(S("律"), S("硬皇力")),
                    HeaderElem::KeyedValue(S("2018年4月8日 18"), S("00頃"))
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
        ))
    )
}
