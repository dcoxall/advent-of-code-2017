use std::collections::HashMap;
use std::collections::VecDeque;

type Registers = HashMap<char, isize>;

enum Value {
    Register(char),
    Literal(isize),
}

impl Value {
    fn resolve(&self, registers: &mut Registers) -> isize {
        match self {
            &Value::Register(c) => registers.entry(c).or_insert(0).clone(),
            &Value::Literal(n)  => n,
        }
    }

    fn set(&self, registers: &mut Registers, value: isize) {
        if let &Value::Register(c) = self {
            let entry = registers.entry(c).or_insert(0);
            *entry = value;
        }
    }
}

impl <'a> From<&'a str> for Value {
    fn from(input: &'a str) -> Value {
        use Value::*;
        if let Ok(n) = input.parse::<isize>() {
            Literal(n)
        } else {
            Register(input.chars().next().unwrap())
        }
    }
}

enum Instruction {
    Snd(Value),
    Rcv(Value),
    Set(Value, Value),
    Add(Value, Value),
    Mul(Value, Value),
    Mod(Value, Value),
    Jgz(Value, Value),
}

impl <'a> From<&'a String> for Instruction {
    fn from(input: &'a String) -> Instruction {
        use Instruction::*;
        let parts: Vec<_> = input.split_whitespace().collect();
        match parts[0] {
            "snd" => Snd(Value::from(parts[1])),
            "rcv" => Rcv(Value::from(parts[1])),
            "set" => Set(Value::from(parts[1]), Value::from(parts[2])),
            "add" => Add(Value::from(parts[1]), Value::from(parts[2])),
            "mul" => Mul(Value::from(parts[1]), Value::from(parts[2])),
            "mod" => Mod(Value::from(parts[1]), Value::from(parts[2])),
            "jgz" => Jgz(Value::from(parts[1]), Value::from(parts[2])),
            _     => panic!("Unrecognised instruction"),
        }
    }
}

pub fn recovered(input: &[String]) -> isize {
    use Instruction::*;

    let mut registers: Registers = HashMap::new();
    let mut sounds: VecDeque<isize> = VecDeque::new();
    let mut i: isize = 0;
    let instructions: Vec<_> = input.iter().map(Instruction::from).collect();

    while let Some(instruction) = instructions.get(i as usize) {
        match instruction {
            &Snd(ref a)        => sounds.push_back(a.resolve(&mut registers)),
            &Set(ref a, ref b) => {
                let b_val = { b.resolve(&mut registers).clone() };
                a.set(&mut registers, b_val);
            },
            &Add(ref a, ref b) => {
                let a_val = { a.resolve(&mut registers) };
                let b_val = { b.resolve(&mut registers) };
                a.set(&mut registers, a_val + b_val);
            },
            &Mul(ref a, ref b) => {
                let a_val = { a.resolve(&mut registers) };
                let b_val = { b.resolve(&mut registers) };
                a.set(&mut registers, a_val * b_val);
            },
            &Mod(ref a, ref b) => {
                let a_val = { a.resolve(&mut registers) };
                let b_val = { b.resolve(&mut registers) };
                a.set(&mut registers, a_val % b_val);
            },
            &Jgz(ref a, ref b) => {
                if a.resolve(&mut registers) > 0 {
                    i += b.resolve(&mut registers);
                    continue;
                }
            },
            &Rcv(ref a)   => {
                if a.resolve(&mut registers) != 0 {
                    return sounds.back().unwrap().clone();
                }
            },
        }

        i += 1;
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = vec![
            String::from("set a 1"),
            String::from("add a 2"),
            String::from("mul a a"),
            String::from("mod a 5"),
            String::from("snd a"),
            String::from("set a 0"),
            String::from("rcv a"),
            String::from("jgz a -1"),
            String::from("set a 1"),
            String::from("jgz a -2"),
        ];

        assert_eq!(recovered(&input), 4);
    }
}
