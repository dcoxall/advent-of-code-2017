#[cfg(test)]
mod tests {
    use super::is_valid;

    #[test]
    fn test_valid_1() {
        assert!(is_valid(String::from("abcde fghij")));
    }

    #[test]
    fn test_invalid_1() {
        assert!(!is_valid(String::from("abcde xyz ecdab")));
    }

    #[test]
    fn test_valid_2() {
        assert!(is_valid(String::from("a ab abc abd abf abj")));
    }

    #[test]
    fn test_valid_3() {
        assert!(is_valid(String::from("iiii oiii ooii oooi oooo")));
    }

    #[test]
    fn test_invalid_2() {
        assert!(!is_valid(String::from("oiii ioii iioi iiio")));
    }
}

use std::collections::HashMap;

pub fn is_valid(passphrase: String) -> bool {
    let mut map: HashMap<String, u32> = HashMap::new();
    for word in passphrase.split_whitespace() {
        let mut chars: Vec<char> = word.chars().collect();
        chars.sort();
        let word: String = chars.iter().collect();
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    map.values().all(|n| *n == 1)
}
