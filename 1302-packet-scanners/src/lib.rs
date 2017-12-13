use std::collections::HashMap;

pub fn delay(input: &[String]) -> u32 {
    let layers: HashMap<u32, u32> = input.iter().map(|line| {
        let parts: Vec<_> = line.split(": ").collect();
        (parts[0].parse::<u32>().unwrap(), parts[1].parse::<u32>().unwrap())
    }).collect();

    let mut d = 0;
    while caught(&layers, d) {
        d += 1;
    }
    d
}

fn caught(layers: &HashMap<u32, u32>, offset: u32) -> bool {
    let max = layers.keys().max().unwrap();
    for depth in 0..(*max + 1) {
        let time = depth + offset;
        if let Some(range) = layers.get(&depth) {
            if time % ((range - 1) * 2) == 0 {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::delay;
    #[test]
    fn it_works() {
        let input = vec![
            String::from("0: 3"),
            String::from("1: 2"),
            String::from("4: 4"),
            String::from("6: 4"),
        ];
        assert_eq!(delay(&input), 10);
    }
}
