use nom::character::complete::*;
use nom::multi::many_m_n;
use nom::IResult;

mod header;
mod pekzep_numeral;

#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;

use cetkaik_core::absolute;

type CKKA = (header::Header, String);

use nom::branch::alt;
use nom::combinator::map;

type PossiblyUnknown<T> = Option<T>;

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Move {
    NoStepAndNoStick {
        src: absolute::Coord,
        prof: PossiblyUnknown<cetkaik_core::Profession>,
        dest: absolute::Coord,
    },

    NoStepAndWaterStick {
        src: absolute::Coord,
        prof: PossiblyUnknown<cetkaik_core::Profession>,
        dest: absolute::Coord,
        water_stick_size: PossiblyUnknown<i32>,
        water_stick_successful: bool,
    },

    StepAndNoStick {
        src: absolute::Coord,
        prof: PossiblyUnknown<cetkaik_core::Profession>,
        step: absolute::Coord,
        dest: absolute::Coord,
    },

    StepAndWaterStick {
        src: absolute::Coord,
        prof: PossiblyUnknown<cetkaik_core::Profession>,
        step: absolute::Coord,
        dest: absolute::Coord,
        water_stick_size: PossiblyUnknown<i32>,
        water_stick_successful: bool,
    },

    StepAndBridgeStick {
        src: absolute::Coord,
        prof: PossiblyUnknown<cetkaik_core::Profession>,
        step: absolute::Coord,
        dest: absolute::Coord,
        bridge_stick_size: PossiblyUnknown<i32>,
        bridge_stick_successful: bool,
    },

    StepAndBridgeStickAndWaterStick {
        src: absolute::Coord,
        prof: PossiblyUnknown<cetkaik_core::Profession>,
        step: absolute::Coord,
        dest: absolute::Coord,
        bridge_stick_size: PossiblyUnknown<i32>,
        /* The fact that water_stick_size exists assert that bridge_stick was successful */
        water_stick_size: PossiblyUnknown<i32>,
        water_stick_successful: bool,
    },

    TamNoStep {
        src: absolute::Coord,
        first_dest: PossiblyUnknown<absolute::Coord>,
        second_dest: absolute::Coord,
    },

    TamStepUnspecified {
        src: absolute::Coord,
        step: absolute::Coord,
        second_dest: absolute::Coord,
    },

    TamStepDuringFormer {
        src: absolute::Coord,
        step: absolute::Coord,
        first_dest: PossiblyUnknown<absolute::Coord>,
        second_dest: absolute::Coord,
    },

    TamStepDuringLatter {
        src: absolute::Coord,
        first_dest: PossiblyUnknown<absolute::Coord>,
        step: absolute::Coord,
        second_dest: absolute::Coord,
    },

    Parachute {
        color: cetkaik_core::Color,
        prof: cetkaik_core::Profession,
        dest: absolute::Coord,
    },
}

use nom::bytes::complete::tag;

/// Examples:
/// ```
/// use ckka_rust::{parse_no_step_and_no_stick, Move};
/// use cetkaik_core::Profession;
/// use cetkaik_core::absolute::*;
/// assert_eq!(
///     parse_no_step_and_no_stick("XU兵XY無撃裁"), 
///     Ok((
///         "", 
///         Move::NoStepAndNoStick { 
///             src: (Row::U, Column::X),
///             prof: Some(Profession::Kauk2), 
///             dest: (Row::Y, Column::X),
///         }
///     ))
/// );
/// ```
///
pub fn parse_no_step_and_no_stick(s: &str) -> IResult<&str, Move> {
    let (rest, src) = parse_square(s)?;
    let (rest, prof) = parse_profession_or_wildcard(rest)?;
    let (rest, dest) = parse_square(rest)?;
    let (rest, _) = tag("無撃裁")(rest)?;

    Ok((rest, Move::NoStepAndNoStick { src, prof, dest }))
}

/// Examples:
/// ```
/// use ckka_rust::*;
/// use cetkaik_core::Profession;
/// use cetkaik_core::absolute::*;
/// assert_eq!(
///     parse_no_step_and_water_stick("LY弓ZY水或此無"), 
///     Ok((
///         "", 
///         Move::NoStepAndWaterStick { 
///             src: (Row::Y, Column::L),
///             prof: Some(Profession::Gua2), 
///             dest: (Row::Y, Column::Z),
///             water_stick_size: None,
///             water_stick_successful: false,
///         }
///     ))
/// );
/// assert_eq!(
///     parse_no_step_and_water_stick("LY弓ZY水一此無"), 
///     Ok((
///         "", 
///         Move::NoStepAndWaterStick { 
///             src: (Row::Y, Column::L),
///             prof: Some(Profession::Gua2), 
///             dest: (Row::Y, Column::Z),
///             water_stick_size: Some(1),
///             water_stick_successful: false,
///         }
///     ))
/// );
/// assert_eq!(
///     parse_no_step_and_water_stick("LY弓ZY水五"), 
///     Ok((
///         "", 
///         Move::NoStepAndWaterStick { 
///             src: (Row::Y, Column::L),
///             prof: Some(Profession::Gua2), 
///             dest: (Row::Y, Column::Z),
///             water_stick_size: Some(5),
///             water_stick_successful: true,
///         }
///     ))
/// );
/// assert_eq!(
///     parse_no_step_and_water_stick("LY弓ZY水或"), 
///     Ok((
///         "", 
///         Move::NoStepAndWaterStick { 
///             src: (Row::Y, Column::L),
///             prof: Some(Profession::Gua2), 
///             dest: (Row::Y, Column::Z),
///             water_stick_size: None,
///             water_stick_successful: true,
///         }
///     ))
/// );
/// ```
///
pub fn parse_no_step_and_water_stick(s: &str) -> IResult<&str, Move> {
    let (rest, src) = parse_square(s)?;
    let (rest, prof) = parse_profession_or_wildcard(rest)?;
    let (rest, dest) = parse_square(rest)?;
    let (rest, (water_stick_size, water_stick_successful)) = parse_water_stick(rest)?;

    Ok((
        rest,
        Move::NoStepAndWaterStick {
            src,
            prof,
            dest,
            water_stick_size,
            water_stick_successful,
        },
    ))
}

pub fn parse_step_and_no_stick(s: &str) -> IResult<&str, Move> {
    let (rest, src) = parse_square(s)?;
    let (rest, prof) = parse_profession_or_wildcard(rest)?;
    let (rest, step) = parse_square(rest)?;
    let (rest, dest) = parse_square(rest)?;
    let (rest, _) = tag("無撃裁")(rest)?;

    Ok((
        rest,
        Move::StepAndNoStick {
            src,
            prof,
            step,
            dest,
        },
    ))
}

pub fn parse_step_and_water_stick(s: &str) -> IResult<&str, Move> {
    let (rest, src) = parse_square(s)?;
    let (rest, prof) = parse_profession_or_wildcard(rest)?;
    let (rest, step) = parse_square(rest)?;
    let (rest, dest) = parse_square(rest)?;
    let (rest, (water_stick_size, water_stick_successful)) = parse_water_stick(rest)?;

    Ok((
        rest,
        Move::StepAndWaterStick {
            src,
            prof,
            step,
            dest,
            water_stick_size,
            water_stick_successful,
        },
    ))
}

pub fn parse_step_and_bridge_stick(s: &str) -> IResult<&str, Move> {
    let (rest, src) = parse_square(s)?;
    let (rest, prof) = parse_profession_or_wildcard(rest)?;
    let (rest, step) = parse_square(rest)?;
    let (rest, dest) = parse_square(rest)?;
    let (rest, bridge_stick_size) = parse_bridge_stick_size(rest)?;
    let (rest, fail_vec) = many_m_n(0, 1, tag("此無"))(rest)?;

    Ok((
        rest,
        Move::StepAndBridgeStick {
            src,
            prof,
            step,
            dest,
            bridge_stick_size,
            bridge_stick_successful: fail_vec.is_empty(),
        },
    ))
}

pub fn parse_step_and_bridge_stick_and_water_stick(s: &str) -> IResult<&str, Move> {
    let (rest, src) = parse_square(s)?;
    let (rest, prof) = parse_profession_or_wildcard(rest)?;
    let (rest, step) = parse_square(rest)?;
    let (rest, dest) = parse_square(rest)?;
    let (rest, bridge_stick_size) = parse_bridge_stick_size(rest)?;
    let (rest, (water_stick_size, water_stick_successful)) = parse_water_stick(rest)?;

    Ok((
        rest,
        Move::StepAndBridgeStickAndWaterStick {
            src,
            prof,
            step,
            dest,
            bridge_stick_size,
            water_stick_size,
            water_stick_successful,
        },
    ))
}

pub fn parse_tam_no_step(s: &str) -> IResult<&str, Move> {
    let (rest, src) = parse_square(s)?;
    let (rest, _) = char('皇')(rest)?;
    let (rest, vec) = many_m_n(0, 1, parse_tam_sqbracket)(rest)?;
    let first_dest: Option<absolute::Coord> = match vec.as_slice() {
        [] | [None] => None,
        [Some(a)] => Some(*a),
        _ => unreachable!(),
    };
    let (rest, second_dest) = parse_square(rest)?;

    Ok((
        rest,
        Move::TamNoStep {
            src,
            first_dest,
            second_dest,
        },
    ))
}

pub fn parse_tam_step_unspecified(s: &str) -> IResult<&str, Move> {
    let (rest, src) = parse_square(s)?;
    let (rest, _) = char('皇')(rest)?;
    let (rest, step) = parse_square(rest)?;
    let (rest, second_dest) = parse_square(rest)?;
    Ok((
        rest,
        Move::TamStepUnspecified {
            src,
            step,
            second_dest,
        },
    ))
}

pub fn parse_tam_step_during_former(s: &str) -> IResult<&str, Move> {
    let (rest, src) = parse_square(s)?;
    let (rest, _) = char('皇')(rest)?;
    let (rest, step) = parse_square(rest)?;
    let (rest, first_dest) = parse_tam_sqbracket(rest)?;
    let (rest, second_dest) = parse_square(rest)?;
    Ok((
        rest,
        Move::TamStepDuringFormer {
            src,
            step,
            first_dest,
            second_dest,
        },
    ))
}

pub fn parse_tam_step_during_latter(s: &str) -> IResult<&str, Move> {
    let (rest, src) = parse_square(s)?;
    let (rest, _) = char('皇')(rest)?;
    let (rest, first_dest) = parse_tam_sqbracket(rest)?;
    let (rest, step) = parse_square(rest)?;
    let (rest, second_dest) = parse_square(rest)?;
    Ok((
        rest,
        Move::TamStepDuringLatter {
            src,
            step,
            first_dest,
            second_dest,
        },
    ))
}

pub fn parse_parachute(s: &str) -> IResult<&str, Move> {
    let (rest, color) = one_of("黒赤")(s)?;
    let color = match color {
        '黒' => cetkaik_core::Color::Huok2,
        '赤' => cetkaik_core::Color::Kok1,
        _ => unreachable!(),
    };
    let (rest, prof) = parse_profession(rest)?;
    let (rest, dest) = parse_square(rest)?;
    Ok((rest, Move::Parachute { color, prof, dest }))
}

/// Examples:
/// ```
/// use ckka_rust::parse_tam_sqbracket;
/// use cetkaik_core::absolute;
/// assert_eq!(parse_tam_sqbracket("[TY]"), Ok(("", Some((absolute::Row::Y, absolute::Column::T)))));
/// assert_eq!(parse_tam_sqbracket("[或]"), Ok(("", None)))
/// ```
///
pub fn parse_tam_sqbracket(s: &str) -> IResult<&str, PossiblyUnknown<absolute::Coord>> {
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
pub fn parse_profession_or_wildcard(
    s: &str,
) -> IResult<&str, PossiblyUnknown<cetkaik_core::Profession>> {
    use std::str::FromStr;
    let (rest, prof) = one_of("船兵弓車虎馬筆巫将王片")(s)?;
    if prof == '片' {
        Ok((rest, None))
    } else {
        let prof = cetkaik_core::Profession::from_str(&prof.to_string()).unwrap();
        Ok((rest, Some(prof)))
    }
}

pub fn parse_bridge_stick_size(s: &str) -> IResult<&str, PossiblyUnknown<i32>> {
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

pub fn parse_water_stick(s: &str) -> IResult<&str, (PossiblyUnknown<i32>, bool)> {
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
#[cfg(test)]
mod tests;
