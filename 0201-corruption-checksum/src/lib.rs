#[cfg(test)]
mod tests {
    use super::corruption_checksum;

    #[test]
    fn it_works() {
        let spreadsheet = vec![
            vec![5, 1, 9, 5],
            vec![7, 5, 3],
            vec![2, 4, 6, 8],
        ];

        assert_eq!(18, corruption_checksum(spreadsheet));
    }
}

pub fn corruption_checksum(spreadsheet: Vec<Vec<u32>>) -> u32 {
    let initial: Option<(u32, u32)> = None;
    spreadsheet.iter()
        .map(|row| {
            row.iter().fold(initial, |val, n| {
                match val {
                    Some((min, max)) => Some((min.min(*n), max.max(*n))),
                    None             => Some((*n, *n)),
                }
            })
        })
        .map(|val| {
            match val {
                Some((min, max)) => max - min,
                None             => 0,
            }
        })
        .sum()
}
