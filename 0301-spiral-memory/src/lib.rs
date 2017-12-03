#[cfg(test)]
mod tests {
    use super::distance;

    #[test]
    fn test_1_distance() {
        assert_eq!(0, distance(1));
    }

    #[test]
    fn test_12_distance() {
        assert_eq!(3, distance(12));
    }

    #[test]
    fn test_23_distance() {
        assert_eq!(2, distance(23));
    }

    #[test]
    fn test_1024_distance() {
        assert_eq!(31, distance(1024));
    }
}

struct Spiral {
    started: bool,
    current_coord: (i32, i32),
}

impl Spiral {
    fn new() -> Self {
        Spiral {
            current_coord: (0, 0),
            started: false,
        }
    }
}

impl Iterator for Spiral {
    type Item = (i32, i32);
    fn next(&mut self) -> Option<Self::Item> {
        if !self.started {
            self.started = true;
            return Some(self.current_coord);
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
        Some(self.current_coord)
    }
}

pub fn distance(square: u32) -> u32 {
    let spiral = Spiral::new();
    match spiral.skip((square - 1) as usize).next() {
        Some((x, y)) => (x.abs() + y.abs()) as u32,
        _ => 0,
    }
}
