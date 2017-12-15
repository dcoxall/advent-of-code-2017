const MOD: u64 = 2147483647;
struct Generator(u64, u64, u64);
impl Iterator for Generator {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        self.2 = (self.2 * self.0) % MOD;
        while self.2 % self.1 != 0 {
            self.2 = (self.2 * self.0) % MOD;
        }
        Some(self.2)
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

pub fn pairs(length: usize, a: (u64, u64), b: (u64, u64)) -> u32 {
    let generator_a = Generator(16807, a.0, a.1);
    let generator_b = Generator(48271, b.0, b.1);
    let judge       = Judge(generator_a, generator_b);
    judge.take(length).fold(0, |acc, (a, b)| if a == b { acc + 1 } else { acc })
}

#[cfg(test)]
mod tests {
    use super::{Judge, Generator, pairs};
    #[test]
    fn generator_values() {
        let gen_a = Generator(16807, 4, 65);
        let gen_b = Generator(48271, 8, 8921);
        let a_results: Vec<_> = gen_a.take(5).collect();
        let b_results: Vec<_> = gen_b.take(5).collect();
        assert_eq!(a_results, vec![1352636452, 1992081072, 530830436, 1980017072, 740335192]);
        assert_eq!(b_results, vec![1233683848, 862516352, 1159784568, 1616057672, 412269392]);
    }

    #[test]
    fn judge_values() {
        let gen_a = Generator(16807, 4, 65);
        let gen_b = Generator(48271, 8, 8921);
        let mut judge = Judge(gen_a, gen_b);
        let first  = judge.next().unwrap();
        let second = judge.next().unwrap();
        assert_eq!(vec![first.0, first.1], vec![38948, 34184]);
        assert_eq!(vec![second.0, second.1], vec![48816, 62592]);
    }

    #[test]
    fn pair_counting() {
        assert_eq!(pairs(5_000_000, (4, 65), (8, 8921)), 309);
    }
}
