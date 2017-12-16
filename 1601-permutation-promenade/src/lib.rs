pub fn dance(moves: &[&str], dancers: &mut [char]) -> () {
    for m in moves {
        let m = Move::from(*m);
        m.execute(dancers);
    }
}

#[derive(Debug, PartialEq)]
enum Move {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

impl Move {
    fn execute(&self, dancers: &mut [char]) -> () {
        use Move::*;
        match self {
            &Spin(n) => {
                let length = dancers.len();
                for a in 0..n {
                    for b in 0..length-n {
                        let n = (length - (n + b)) + a;
                        dancers.swap(n, n - 1);
                    }
                }
            },
            &Exchange(a, b) => dancers.swap(a, b),
            &Partner(ref a, ref b) => {
                let a = dancers.iter().position(|c| c == a).unwrap();
                let b = dancers.iter().position(|c| c == b).unwrap();
                dancers.swap(a, b);
            },
        }
    }
}

impl <'a> From<&'a str> for Move {
    fn from(val: &'a str) -> Move {
        let mut chars = val.chars();
        let id = chars.next().unwrap();
        let chars: String = chars.collect();
        let chars: Vec<_> = chars.split('/').collect();
        match id {
            's' => Move::Spin(chars[0].parse().unwrap()),
            'x' => Move::Exchange(chars[0].parse().unwrap(), chars[1].parse().unwrap()),
            'p' => Move::Partner(chars[0].parse().unwrap(), chars[1].parse().unwrap()),
            _   => panic!("This makes no sense"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_moves() {
        assert_eq!(Move::from("s5"),    Move::Spin(5));
        assert_eq!(Move::from("s2"),    Move::Spin(2));
        assert_eq!(Move::from("x3/15"), Move::Exchange(3, 15));
        assert_eq!(Move::from("x9/10"), Move::Exchange(9, 10));
        assert_eq!(Move::from("pe/b"),  Move::Partner('e', 'b'));
        assert_eq!(Move::from("pa/p"),  Move::Partner('a', 'p'));
    }

    #[test]
    fn executing_spin() {
        let mut dancers = vec!['a', 'b', 'c', 'd', 'e'];
        let movement = Move::Spin(2);
        movement.execute(&mut dancers);
        assert_eq!(dancers, vec!['d', 'e', 'a', 'b', 'c']);
        let movement = Move::Spin(4);
        movement.execute(&mut dancers);
        assert_eq!(dancers, vec!['e', 'a', 'b', 'c', 'd']);
    }

    #[test]
    fn executing_exchange() {
        let mut dancers = vec!['a', 'b', 'c', 'd', 'e'];
        let movement = Move::Exchange(2, 4);
        movement.execute(&mut dancers);
        assert_eq!(dancers, vec!['a', 'b', 'e', 'd', 'c']);
        let movement = Move::Exchange(0, 1);
        movement.execute(&mut dancers);
        assert_eq!(dancers, vec!['b', 'a', 'e', 'd', 'c']);
    }

    #[test]
    fn executing_partner() {
        let mut dancers = vec!['a', 'b', 'c', 'd', 'e'];
        let movement = Move::Partner('d', 'a');
        movement.execute(&mut dancers);
        assert_eq!(dancers, vec!['d', 'b', 'c', 'a', 'e']);
        let movement = Move::Partner('b', 'e');
        movement.execute(&mut dancers);
        assert_eq!(dancers, vec!['d', 'e', 'c', 'a', 'b']);
    }

    #[test]
    fn executing_dance() {
        let mut dancers = vec!['a', 'b', 'c', 'd', 'e'];
        let moves = vec!["s1", "x3/4", "pe/b"];
        dance(&moves, &mut dancers);
        assert_eq!(dancers, vec!['b', 'a', 'e', 'd', 'c']);
    }
}
