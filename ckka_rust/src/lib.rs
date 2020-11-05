struct Header {
    info: Vec<HeaderElem>,
    players: Option<(PlayerAndPoint, PlayerAndPoint)>
}

struct PlayerAndPoint {
    player_name: String,
    point: i64
}

enum HeaderElem {
    Value(String),
    KeyedValue(String, String)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
