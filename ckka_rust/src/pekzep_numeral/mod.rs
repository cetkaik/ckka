mod tests;

pub fn analyze(s: &[char]) -> Option<i64> {
    match s {
        ['無'] => Some(0),
        ['下', tail @ ..] => positive(tail).map(|a| -a),
        simple => positive(simple),
    }
}

fn less_than_10(s: char) -> Option<i64> {
    match s {
        '一' => Some(1),
        '二' => Some(2),
        '三' => Some(3),
        '四' => Some(4),
        '五' => Some(5),
        '六' => Some(6),
        '七' => Some(7),
        '八' => Some(8),
        '九' => Some(9),
        _ => None,
    }
}

fn less_than_100(s: &[char]) -> Option<i64> {
    match s {
        ['十'] => Some(10),
        [digit] => less_than_10(*digit),
        [digit, '十'] => less_than_10(*digit).map(|a| a * 10),
        ['十', digit] => less_than_10(*digit).map(|a| a + 10),
        [digit1, '十', digit2] => {
            let d1 = less_than_10(*digit1)?;
            let d2 = less_than_10(*digit2)?;
            Some(d1 * 10 + d2)
        }
        _ => None,
    }
}

fn less_than_100_nun1_elided(s: &[char]) -> Option<i64> {
    match s {
        ['十'] => Some(10),
        [digit] => less_than_10(*digit),
        [digit, '十'] => less_than_10(*digit).map(|a| a * 10),
        ['十', digit] => less_than_10(*digit).map(|a| a + 10),
        [digit1, digit2] => {
            let d1 = less_than_10(*digit1)?;
            let d2 = less_than_10(*digit2)?;
            Some(d1 * 10 + d2)
        },
        _ => None,
    }
}

pub fn less_than_10000(s: &[char]) -> Option<i64> {
    match s {
        ['百'] => Some(100),
        [a, '百'] => less_than_100_nun1_elided(&[*a]).map(|w| w * 100),
        [a, b, '百'] => less_than_100_nun1_elided(&[*a, *b]).map(|w| w * 100),

        ['百', c] => less_than_100_nun1_elided(&[*c]).map(|a| 100 + a),
        ['百', c, d] => less_than_100_nun1_elided(&[*c, *d]).map(|a| 100 + a),
        ['百', c, d, e] => less_than_100(&[*c, *d, *e]).map(|a| 100 + a), // only for pure hundred
        [a, '百', tail @ ..] => {
            let w = less_than_100_nun1_elided(&[*a])?;
            let x = less_than_100_nun1_elided(tail)?;
            Some(w * 100 + x)
        }
        [a, b, '百', tail @ ..] =>  {
            let w = less_than_100_nun1_elided(&[*a, *b])?;
            let x = less_than_100_nun1_elided(tail)?;
            Some(w * 100 + x)
        }
        _ => less_than_100(s),
    }
}

fn positive(s: &[char]) -> Option<i64> { unimplemented!() }