#[cfg(test)]
mod tests {
    use super::larger_than;

    #[test]
    fn test_5_larger_than() {
        assert_eq!(10, larger_than(5));
    }

    #[test]
    fn test_747_larger_than() {
        assert_eq!(806, larger_than(747));
    }
}

use std::collections::HashMap;

struct Spiral {
    started: bool,
    current_coord: (i32, i32),
    store: HashMap<(i32, i32), u32>,
}

impl Spiral {
    fn new() -> Self {
        Spiral {
            current_coord: (0, 0),
            started: false,
            store: HashMap::new(),
        }
    }

    fn total_neighbours(&self) -> u32 {
        let mut sum: u32 = 0;
        for x in -1..2 {
            for y in -1..2 {
                if x == 0 && y == 0 {
                    continue;
                }
                let lookup = (self.current_coord.0 + x, self.current_coord.1 + y);
                if let Some(n) = self.store.get(&lookup) {
                    sum += *n;
                }
            }
        }
        sum
    }
}

impl Iterator for Spiral {
    type Item = (u32, (i32, i32));
    fn next(&mut self) -> Option<Self::Item> {
        if !self.started {
            self.started = true;
            self.store.insert(self.current_coord, 1);
            return Some((1, self.current_coord));
        }

        let x = self.current_coord.0;
        let y = self.current_coord.1;
        if x.abs() <= y.abs() && (x != y || x >= 0) {
            self.current_coord.0 += match y >= 0 {
                true  => 1,
                false => -1,
            };
        } else {
            self.current_coord.1 += match x >= 0 {
                true  => -1,
                false => 1,
            };
        }
        let n = self.total_neighbours();
        self.store.insert(self.current_coord, n);
        Some((n, self.current_coord))
    }
}

pub fn larger_than(x: u32) -> u32 {
    let spiral = Spiral::new();
    match spiral.filter(|val| val.0 > x).next() {
        Some((i, _)) => i,
        None         => 0,
    }
}
