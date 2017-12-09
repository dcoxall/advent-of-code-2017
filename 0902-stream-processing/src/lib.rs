pub fn garbage(stream: &str) -> i32 {
    let mut total = 0;
    let mut garbage = false;
    let mut stream = stream.chars();

    while let Some(c) = stream.next() {
        match (c, garbage) {
            ('<', false) => garbage = true,
            ('>', true)  => garbage = false,
            ('!', _)     => {stream.next();}, // skip next character
            (_, true)    => total += 1,
            _            => (), // do nothing
        };
    }

    total
}

#[cfg(test)]
mod tests {
    use super::garbage;

    #[test]
    fn test_it_works() {
        assert_eq!(garbage(r#"<>"#), 0);
        assert_eq!(garbage(r#"<random characters>"#), 17);
        assert_eq!(garbage(r#"<<<<>"#), 3);
        assert_eq!(garbage(r#"<{!>}>"#), 2);
        assert_eq!(garbage(r#"<!!>"#), 0);
        assert_eq!(garbage(r#"<!!!>>"#), 0);
        assert_eq!(garbage(r#"<{o"i!a,<{i<a>"#), 10);
    }

}
