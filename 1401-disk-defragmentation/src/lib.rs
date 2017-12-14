extern crate knothash;

pub fn used(input: &str) -> u32 {
    (0..128).map(|n| format!("{}-{}", input, n))
        .map(|input| knothash::digest(&input))
        .flat_map(|string| bits(&string).into_iter())
        .fold(0, |t, b| if b { t + 1 } else { t })
}

fn bits(input: &str) -> Vec<bool> {
    let bit_string: String = input.chars().filter_map(|c| hex_to_bits(c)).collect();
    bit_string.chars().map(|b| b == '1').collect()
}

fn hex_to_bits(c: char) -> Option<String> {
    match c {
        '0' => Some(String::from("0000")),
        '1' => Some(String::from("0001")),
        '2' => Some(String::from("0010")),
        '3' => Some(String::from("0011")),
        '4' => Some(String::from("0100")),
        '5' => Some(String::from("0101")),
        '6' => Some(String::from("0110")),
        '7' => Some(String::from("0111")),
        '8' => Some(String::from("1000")),
        '9' => Some(String::from("1001")),
        'a' => Some(String::from("1010")),
        'b' => Some(String::from("1011")),
        'c' => Some(String::from("1100")),
        'd' => Some(String::from("1101")),
        'e' => Some(String::from("1110")),
        'f' => Some(String::from("1111")),
        _   => None,
    }
}

#[cfg(test)]
mod tests {
    use super::used;
    #[test]
    fn it_works() {
        assert_eq!(used("flqrgnkx"), 8108);
    }
}
