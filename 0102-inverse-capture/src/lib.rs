#[cfg(test)]
mod tests {
    use super::inverse_capture;

    #[test]
    fn test_1212() {
        assert_eq!(6, inverse_capture(vec![1,2,1,2]));
    }

    #[test]
    fn test_1221() {
        assert_eq!(0, inverse_capture(vec![1,2,2,1]));
    }

    #[test]
    fn test_123425() {
        assert_eq!(4, inverse_capture(vec![1,2,3,4,2,5]));
    }

    #[test]
    fn test_123123() {
        assert_eq!(12, inverse_capture(vec![1,2,3,1,2,3]));
    }

    #[test]
    fn test_12131415() {
        assert_eq!(4, inverse_capture(vec![1,2,1,3,1,4,1,5]));
    }
}

pub fn inverse_capture(input: Vec<u32>) -> u32 {
    let input_clone = input.to_vec();
    let cyclic = input_clone.iter().cycle().skip(input.len() / 2);
    input.iter().zip(cyclic).map(|(n, m)| {
        match n == m {
            true  => *n,
            false => 0,
        }
    }).sum()
}
