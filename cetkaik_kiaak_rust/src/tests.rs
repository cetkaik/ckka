use super::header::*;
use big_s::S;

#[test]
fn sample1() {
    use super::*;
    parse_ckka(r#"
    {https://drive.google.com/drive/folders/183ENcTW65lPGVONnnhkJ1nGKDUYeXWHX?usp=sharing}
    {:2020-10-15}
    [SY]二十 [ぶちょー]二十
    CI兵XIXU無撃裁         LAU弓LAILY橋三
    LE弓LILU橋二           MAI兵MY無撃裁
    PE巫MECE橋二           XAU虎CAIXAU橋三
    CE巫CI無撃裁           ZIA王TAUNAU無撃裁
    XI兵XUXY無撃裁         XAU虎ZAIXY無撃裁 手兵
    XU兵XY無撃裁 手虎      NAI兵NY無撃裁
    CI巫KIALIA橋二 手馬 
    [SY]為(同色獣)再行 
                            LY弓LU無撃裁 手弓
    LIA巫NIANAU無撃裁 手王 
    [SY]為(同色獣)(王)終季 手二十
    
    星一周"#).unwrap();
}

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
