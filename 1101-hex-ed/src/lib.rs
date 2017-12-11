use std::str::FromStr;

struct Direction(i32, i32, i32);

impl Direction {
    fn q(&self) -> i32 { self.0 }
    fn r(&self) -> i32 { self.1 }
    fn s(&self) -> i32 { self.2 }
}

impl FromStr for Direction {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ne" => Ok(Direction(-1, 0, 1)),
            "n"  => Ok(Direction(0, -1, 1)),
            "nw" => Ok(Direction(1, -1, 0)),
            "se" => Ok(Direction(-1, 1, 0)),
            "s"  => Ok(Direction(0, 1, -1)),
            "sw" => Ok(Direction(1, 0, -1)),
            _    => Err("invalid direction"),
        }
    }
}

pub fn distance(steps: &str) -> u32 {
    let (q, r, s) = steps.split(',')
        .filter_map(|d| d.parse::<Direction>().ok())
        .fold((0, 0, 0), |(q, r, s), d| {
            (q + d.q(), r + d.r(), s + d.s())
        });

    ((q.abs() + r.abs() + s.abs()) / 2i32) as u32
}

#[cfg(test)]
mod tests {
    use super::distance;
    #[test]
    fn it_works() {
        assert_eq!(distance("ne,ne,ne"), 3);
        assert_eq!(distance("ne,ne,sw,sw"), 0);
        assert_eq!(distance("ne,ne,s,s"), 2);
        assert_eq!(distance("se,sw,se,sw,sw"), 3);
    }
}
