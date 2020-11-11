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
        }
        _ => None,
    }
}

fn less_than_10000_0000(input: &[char]) -> Option<i64> {
    match input {
        ['万'] => Some(10000),
        [head @ .., '万'] => less_than_10000(head).map(|w| w * 10000),
        ['万', tail @ ..] => Some(10000 + less_than_10000(tail)?),

        [a, '万', tail @ ..] => Some(less_than_10000(&[*a])? * 10000 + less_than_10000(tail)?),
        [a, b, '万', tail @ ..] => {
            Some(less_than_10000(&[*a, *b])? * 10000 + less_than_10000(tail)?)
        }
        [a, b, c, '万', tail @ ..] => {
            Some(less_than_10000(&[*a, *b, *c])? * 10000 + less_than_10000(tail)?)
        }
        [a, b, c, d, '万', tail @ ..] => {
            Some(less_than_10000(&[*a, *b, *c, *d])? * 10000 + less_than_10000(tail)?)
        }
        [a, b, c, d, e, '万', tail @ ..] => {
            Some(less_than_10000(&[*a, *b, *c, *d, *e])? * 10000 + less_than_10000(tail)?)
        }
        _ => less_than_10000(input),
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
            Some(less_than_100_nun1_elided(&[*a])? * 100 + less_than_100_nun1_elided(tail)?)
        }
        [a, b, '百', tail @ ..] => {
            Some(less_than_100_nun1_elided(&[*a, *b])? * 100 + less_than_100_nun1_elided(tail)?)
        }
        _ => less_than_100(s),
    }
}

fn positive(s: &[char]) -> Option<i64> {
    match s {
        ['億'] => Some(1_0000_0000),
        [head @ .., '億'] => less_than_10000_0000(head).map(|w| w * 1_0000_0000),
        ['億', tail @ ..] => Some(1_0000_0000 + less_than_10000_0000(tail)?),

        [a, '億', tail @ ..] => {
            Some(less_than_10000_0000(&[*a])? * 1_0000_0000 + less_than_10000_0000(tail)?)
        }
        [a, b, '億', tail @ ..] => {
            Some(less_than_10000_0000(&[*a, *b])? * 1_0000_0000 + less_than_10000_0000(tail)?)
        }
        [a, b, c, '億', tail @ ..] => {
            Some(less_than_10000_0000(&[*a, *b, *c])? * 1_0000_0000 + less_than_10000_0000(tail)?)
        }
        // only need to handle till 21億
        _ => less_than_10000_0000(s),
    }
}
