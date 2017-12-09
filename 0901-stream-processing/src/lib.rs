pub fn score(stream: &str) -> i32 {
    let mut total = 0;
    let mut garbage = false;
    let mut level = 0;
    let mut stream = stream.chars();

    while let Some(c) = stream.next() {
        match (c, garbage) {
            ('{', false) => {
                level += 1;
                total += level;
            },
            ('}', false) => level -= 1,
            ('<', false) => garbage = true,
            ('>', true)  => garbage = false,
            ('!', _)     => {stream.next();}, // skip next character
            _            => (), // do nothing
        };
    }

    total
}

#[cfg(test)]
mod tests {
    use super::score;

    #[test]
    fn score_1_1() {
        assert_eq!(1, score("{}"));
    }

    #[test]
    fn score_6() {
        assert_eq!(6, score("{{{}}}"));
    }

    #[test]
    fn score_5() {
        assert_eq!(5, score("{{},{}}"));
    }

    #[test]
    fn score_16() {
        assert_eq!(16, score("{{{},{},{{}}}}"));
    }

    #[test]
    fn score_1_2() {
        assert_eq!(1, score("{<a>,<a>,<a>,<a>}"));
    }

    #[test]
    fn score_9_1() {
        assert_eq!(9, score("{{<ab>},{<ab>},{<ab>},{<ab>}}"));
    }

    #[test]
    fn score_9_2() {
        assert_eq!(9, score("{{<!!>},{<!!>},{<!!>},{<!!>}}"));
    }

    #[test]
    fn score_3() {
        assert_eq!(3, score("{{<a!>},{<a!>},{<a!>},{<ab>}}"));
    }
}
