#[cfg(test)]
mod tests {
    use super::is_valid;

    #[test]
    fn test_valid_1() {
        assert!(is_valid(String::from("aa bb cc dd ee")));
    }

    #[test]
    fn test_valid_2() {
        assert!(is_valid(String::from("aa bb cc dd aaa")));
    }

    #[test]
    fn test_valid_3() {
        assert!(!is_valid(String::from("aa bb cc dd aa")));
    }
}

use std::collections::HashMap;

pub fn is_valid(passphrase: String) -> bool {
    let mut map: HashMap<&str, u32> = HashMap::new();
    for word in passphrase.split_whitespace() {
        let count = map.entry(&word).or_insert(0);
        *count += 1;
    }
    map.values().all(|n| *n == 1)
}
