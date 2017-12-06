#[cfg(test)]
mod tests {
    use super::realloc;

    #[test]
    fn it_works() {
        let mut banks = vec![0, 2, 7, 0];
        assert_eq!(4, realloc(&mut banks));
    }
}

use std::collections::HashMap;

pub fn realloc(banks: &mut Vec<u32>) -> u32 {
    let mut set: HashMap<Vec<u32>, usize> = HashMap::new();
    let mut snapshot = banks.clone();
    let mut iteration = 0;

    while !set.contains_key(&snapshot) {
        set.insert(snapshot, iteration);
        iteration += 1;

        let mut blocks = 0;
        let mut index = 0;

        for (i, n) in banks.iter().enumerate() {
            if *n > blocks {
                blocks = *n;
                index = i;
            }
        }

        banks[index] = 0;

        while blocks > 0 {
            index += 1;
            let i = index % banks.len();
            banks[i] += 1;
            blocks -= 1;
        }

        snapshot = banks.clone();
    }

    let loop_start = set.get(&snapshot).expect("It's here! I know it!");
    (iteration - loop_start) as u32
}
