const MOD: u64 = 2147483647;
struct Generator(u64, u64);
impl Iterator for Generator {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        self.1 = (self.1 * self.0) % MOD;
        Some(self.1)
    }
}

struct Judge(Generator, Generator);
impl Iterator for Judge {
    type Item = (u64, u64);
    fn next(&mut self) -> Option<Self::Item> {
        let a = self.0.next().unwrap() & 0xffff;
        let b = self.1.next().unwrap() & 0xffff;
        Some((a, b))
    }
}

pub fn pairs(length: usize, a_seed: u64, b_seed: u64) -> u32 {
    let generator_a = Generator(16807, a_seed);
    let generator_b = Generator(48271, b_seed);
    let judge       = Judge(generator_a, generator_b);
    judge.take(length).fold(0, |acc, (a, b)| if a == b { acc + 1 } else { acc })
}

#[cfg(test)]
mod tests {
    use super::{Judge, Generator, pairs};
    #[test]
    fn generator_values() {
        let gen_a = Generator(16807, 65);
        let gen_b = Generator(48271, 8921);
        let a_results: Vec<_> = gen_a.take(5).collect();
        let b_results: Vec<_> = gen_b.take(5).collect();
        assert_eq!(a_results, vec![1092455, 1181022009, 245556042, 1744312007, 1352636452]);
        assert_eq!(b_results, vec![430625591, 1233683848, 1431495498, 137874439, 285222916]);
    }

    #[test]
    fn judge_values() {
        let gen_a = Generator(16807, 65);
        let gen_b = Generator(48271, 8921);
        let mut judge = Judge(gen_a, gen_b);
        let first  = judge.next().unwrap();
        let second = judge.next().unwrap();
        assert_eq!(vec![first.0, first.1], vec![43879, 54071]);
        assert_eq!(vec![second.0, second.1], vec![63289, 34184]);
    }

    #[test]
    fn pair_counting() {
        assert_eq!(pairs(40_000_000, 65, 8921), 588);
    }
}
