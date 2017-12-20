#[macro_use] extern crate lazy_static;
extern crate regex;
use regex::Regex;
use std::str::FromStr;
use std::ops::AddAssign;

#[derive(Clone)]
struct Vector3<T> {
    x: T,
    y: T,
    z: T,
}
impl <T> Vector3<T> {
    fn new(x: T, y: T, z: T) -> Self {
        Vector3 { x, y, z }
    }
}
impl <T: AddAssign + Clone> AddAssign for Vector3<T> {
    fn add_assign(&mut self, rhs: Vector3<T>) {
        self.x += rhs.x.clone();
        self.y += rhs.y.clone();
        self.z += rhs.z.clone();
    }
}

struct Particle {
    pos: Vector3<isize>,
    vel: Vector3<isize>,
    acc: Vector3<isize>,
}
impl Particle {
    fn tick(&mut self) {
        self.vel += self.acc.clone();
        self.pos += self.vel.clone();
    }

    fn distance(&self) -> usize {
        (self.pos.x.abs() + self.pos.y.abs() + self.pos.z.abs()) as usize
    }
}
impl FromStr for Particle {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?x)^
                p=<\s?(?P<px>-?\d+),\s?(?P<py>-?\d+),\s?(?P<pz>-?\d+)>,\s
                v=<\s?(?P<vx>-?\d+),\s?(?P<vy>-?\d+),\s?(?P<vz>-?\d+)>,\s
                a=<\s?(?P<ax>-?\d+),\s?(?P<ay>-?\d+),\s?(?P<az>-?\d+)>
            $").unwrap();
        }
        let cap = RE.captures(s).unwrap();
        let pos: Vector3<isize> = Vector3::new(
            cap["px"].parse()?,
            cap["py"].parse()?,
            cap["pz"].parse()?,
        );
        let vel: Vector3<isize> = Vector3::new(
            cap["vx"].parse()?,
            cap["vy"].parse()?,
            cap["vz"].parse()?,
        );
        let acc: Vector3<isize> = Vector3::new(
            cap["ax"].parse()?,
            cap["ay"].parse()?,
            cap["az"].parse()?,
        );
        Ok(Particle { pos, vel, acc })
    }
}

pub fn closest(input: &[String]) -> usize {
    let mut particles: Vec<Particle> = input.into_iter()
        .filter_map(|line| line.parse().ok())
        .collect();

    // Simulate 10_000 times
    for _ in 0..10_000 {
        particles.iter_mut().for_each(Particle::tick);
    }

    particles.into_iter()
        .enumerate()
        .min_by_key(|&(_, ref p)| p.distance())
        .map(|(i, _)| i)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::closest;
    #[test]
    fn it_works() {
        let input = vec![
            String::from("p=< 3,0,0>, v=< 2,0,0>, a=<-1,0,0>"),
            String::from("p=< 4,0,0>, v=< 0,0,0>, a=<-2,0,0>"),
        ];
        assert_eq!(closest(&input), 0);
    }
}
