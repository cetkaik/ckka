type Header = Vec<HeaderElem>;

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
