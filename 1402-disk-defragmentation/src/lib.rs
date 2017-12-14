extern crate knothash;

use std::collections::HashSet;

type Coords = (usize, usize);

pub fn groups(input: &str) -> u32 {
    let mut grid: Vec<Vec<bool>> = Vec::with_capacity(128);
    for y in 0..128 {
        let input = format!("{}-{}", input, y);
        let input = knothash::digest(&input);
        grid.push(bits(&input));
    }

    let mut visited: HashSet<Coords> = HashSet::new();
    let mut groups = 0;
    for y in 0..128 {
        for x in 0..128 {
            let coord = (x, y);
            if visited.contains(&coord) {
                continue;
            }
            let row = grid.get(coord.0).unwrap();
            if let Some(&false) = row.get(coord.1) {
                continue;
            }
            groups += 1;
            depth_search(coord, &mut visited, &grid);
        }
    }
    groups
}

fn depth_search(coord: Coords, visited: &mut HashSet<Coords>, grid: &Vec<Vec<bool>>) -> () {
    if visited.contains(&coord) {
        return;
    }

    let row = grid.get(coord.0).unwrap();
    if let Some(&false) = row.get(coord.1) {
        return;
    }

    visited.insert(coord);
    if coord.0 > 0 {
        depth_search((coord.0 - 1, coord.1), visited, grid);
    }
    if coord.1 > 0 {
        depth_search((coord.0, coord.1 - 1), visited, grid);
    }
    if coord.0 < 127 {
        depth_search((coord.0 + 1, coord.1), visited, grid);
    }
    if coord.1 < 127 {
        depth_search((coord.0, coord.1 + 1), visited, grid);
    }
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
    use super::groups;
    #[test]
    fn it_works() {
        assert_eq!(groups("flqrgnkx"), 1242);
    }
}
