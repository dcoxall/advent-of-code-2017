pub fn digest(lengths: &str) -> String {
    let mut current_position: usize = 0;
    let mut skip: usize = 0;
    let mut elements: Vec<_> = (0u16..256u16).map(|n| n as u8).collect();
    let mut lengths: Vec<u8> = lengths.bytes().collect();
    lengths.extend(&[17, 31, 73, 47, 23]);

    for _ in 0..64 {
        for length in lengths.iter() {
            let length = *length as usize;
            for p in 0..(length / 2) {
                let p = p as usize;
                let a = (current_position + p) % 256;
                let b = (current_position + length - p - 1) % 256;
                elements.swap(a, b);
            }
            current_position += length + skip;
            skip += 1;
        }
    }

    elements.chunks(16)
        .map(|c| c.iter().fold(0, |a, b| a ^ b))
        .map(|v| format!("{:02x}", v))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::digest;
    #[test]
    fn it_works() {
        assert_eq!(digest(""), "a2582a3a0e66e6e86e3812dcb672a272");
        assert_eq!(digest("AoC 2017"), "33efeb34ea91902bb2f59c9920caa6cd");
        assert_eq!(digest("1,2,3"), "3efbe78a8d82f29979031a4aa0b16a9d");
        assert_eq!(digest("1,2,4"), "63960835bcdc130f0b66d7ff4f6a5a8e");
    }
}
