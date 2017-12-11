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

pub fn max_distance(steps: &str) -> u32 {
    let (_, _, _, d) = steps.split(',')
        .filter_map(|d| d.parse::<Direction>().ok())
        .fold((0, 0, 0, 0), |(q, r, s, dist), d| {
            let (q, r, s) = (q + d.q(), r + d.r(), s + d.s());
            let distance = ((q.abs() + r.abs() + s.abs()) / 2i32) as u32;
            (q, r, s, distance.max(dist))
        });

    d
}
