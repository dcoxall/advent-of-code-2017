use std::collections::HashMap;

pub fn severity(input: &[String]) -> u32 {
    let layers: HashMap<u32, u32> = input.iter().map(|line| {
        let parts: Vec<_> = line.split(": ").collect();
        (parts[0].parse::<u32>().unwrap(), parts[1].parse::<u32>().unwrap())
    }).collect();
    let max = layers.keys().max().unwrap();

    (0..(*max + 1)).into_iter().fold(0, |acc, depth| {
        match layers.get(&depth) {
            Some(range) if depth % ((range - 1) * 2) == 0 => {
                acc + depth * range
            },
            _ => acc,
        }
    })
}

#[cfg(test)]
mod tests {
    use super::severity;
    #[test]
    fn it_works() {
        let input = vec![
            String::from("0: 3"),
            String::from("1: 2"),
            String::from("4: 4"),
            String::from("6: 4"),
        ];
        assert_eq!(severity(&input), 24);
    }
}
