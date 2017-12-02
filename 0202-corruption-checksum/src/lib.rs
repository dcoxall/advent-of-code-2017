#[cfg(test)]
mod tests {
    use super::corruption_checksum;

    #[test]
    fn it_works() {
        let spreadsheet = vec![
            vec![5, 9, 2, 8],
            vec![9, 4, 7, 3],
            vec![3, 8, 6, 5],
        ];

        assert_eq!(9, corruption_checksum(spreadsheet));
    }
}

pub fn corruption_checksum(spreadsheet: Vec<Vec<u32>>) -> u32 {
    let mut permutations: Vec<Vec<(u32, u32)>> = Vec::new();
    spreadsheet.iter().for_each(|row| {
        let mut row_perms: Vec<(u32, u32)> = Vec::new();
        row.iter().for_each(|n| {
            row.iter().filter(|m| *m != n).for_each(|m| {
                row_perms.push((*m.min(n), *m.max(n)));
            });
        });
        permutations.push(row_perms);
    });
    permutations.iter()
        .flat_map(|row| {
            row.iter().filter(|tuple| {
                tuple.1 % tuple.0 == 0
            }).take(1)
        })
        .map(|tuple| tuple.1 / tuple.0)
        .sum()
}
