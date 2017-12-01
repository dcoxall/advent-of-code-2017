#[cfg(test)]
mod tests {
    use super::inverse_capture;

    #[test]
    fn test_1122() {
        assert_eq!(3, inverse_capture(vec![1,1,2,2]));
    }

    #[test]
    fn test_1111() {
        assert_eq!(4, inverse_capture(vec![1,1,1,1]));
    }

    #[test]
    fn test_1234() {
        assert_eq!(0, inverse_capture(vec![1,2,3,4]));
    }

    #[test]
    fn test_91212129() {
        assert_eq!(9, inverse_capture(vec![9,1,2,1,2,1,2,9]));
    }
}

pub fn inverse_capture(input: Vec<u32>) -> u32 {
    let mut vals = input.to_vec();
    vals.push(input[0]);
    vals.windows(2).map(|x| {
        match x[0] == x[1] {
            true  => x[1],
            false => 0,
        }
    }).sum()
}
