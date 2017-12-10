pub fn confirm(elements: &mut [u32], lengths: &[usize]) -> u32 {
    let mut current_position = 0;
    let mut skip = 0;
    let len = elements.len();

    for length in lengths {
        for p in 0..(length / 2) {
            let a = (current_position + p) % len;
            let b = (current_position + length - p - 1) % len;
            elements.swap(a, b);
        }
        current_position += length + skip;
        skip += 1;
    }

    elements[0] * elements[1]
}

#[cfg(test)]
mod tests {
    use super::confirm;
    #[test]
    fn it_works() {
        let mut elements = vec![0, 1, 2, 3, 4];
        let lengths  = vec![3, 4, 1, 5];
        assert_eq!(12, confirm(&mut elements, &lengths));
    }
}
