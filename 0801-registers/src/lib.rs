#[cfg(test)]
mod tests {
    use super::max;

    #[test]
    fn it_works() {
        let input = vec![
            String::from("b inc 5 if a > 1"),
            String::from("a inc 1 if b < 5"),
            String::from("c dec -10 if a >= 1"),
            String::from("c inc -20 if c == 10"),
        ];
        assert_eq!(1, max(&input));
    }
}

use std::collections::HashMap;

#[derive(Debug)]
enum Operation {
    Increment(i32),
    Decrement(i32),
}

impl Operation {
    fn execute(&self, target: &mut i32) -> () {
        match self {
            &Operation::Increment(n) => *target += n,
            &Operation::Decrement(n) => *target -= n,
        }
    }
}

#[derive(Debug)]
enum Comparison {
    Gt(i32),  // >
    Gte(i32), // >=
    Lt(i32),  // <
    Lte(i32), // <=
    Eql(i32), // ==
    Neq(i32), // !=
}

impl Comparison {
    fn parse(str: &str, num: i32) -> Option<Comparison> {
        match str {
            ">"  => Some(Comparison::Gt(num)),
            ">=" => Some(Comparison::Gte(num)),
            "<"  => Some(Comparison::Lt(num)),
            "<=" => Some(Comparison::Lte(num)),
            "==" => Some(Comparison::Eql(num)),
            "!=" => Some(Comparison::Neq(num)),
            _    => None,
        }
    }

    fn comp(&self, target: &i32) -> bool {
        match self {
            &Comparison::Gt(num)  => *target > num,
            &Comparison::Gte(num) => *target >= num,
            &Comparison::Lt(num)  => *target < num,
            &Comparison::Lte(num) => *target <= num,
            &Comparison::Eql(num) => *target == num,
            &Comparison::Neq(num) => *target != num,
        }
    }
}

#[derive(Debug)]
struct Condition {
    target: String,
    operator: Comparison,
}

impl Condition {
    fn evaluate(&self, registers: &mut Registers) -> bool {
        let target = registers.entry(self.target.clone()).or_insert(0);
        self.operator.comp(target)
    }
}

#[derive(Debug)]
struct Instruction {
    target: String,
    operation: Operation,
    condition: Condition,
}

type Registers = HashMap<String, i32>;

impl Instruction {
    fn execute(&self, registers: &mut Registers) -> () {
        if self.condition.evaluate(registers) {
            let target = registers.entry(self.target.clone()).or_insert(0);
            self.operation.execute(target);
        }
    }
}


fn parse_condition(line: &str) -> Option<Condition> {
    let mut parts = line.split_whitespace();
    let target = parts.next().map(|s| s.to_string());
    let operator = parts.next();
    let right = parts.next().iter().filter_map(|s| s.parse::<i32>().ok()).next();

    match (target, operator, right) {
        (Some(target), Some(operator), Some(right)) => {
            Comparison::parse(operator, right).map(|operator| {
                Condition{target, operator}
            })
        },
        _ => None,
    }
}

fn parse_instruction(line: &str) -> Option<Instruction> {
    let mut parts = line.split("if");
    let first = parts.next();
    let mut first_part = first.iter().flat_map(|part| part.split_whitespace());

    let target = first_part.next().map(|part| part.to_string());
    let operation = first_part.next();
    let right = match first_part.next() {
        Some(part) => part.parse::<i32>().ok(),
        None       => None,
    };

    let operation = match (operation, right) {
        (Some("inc"), Some(right)) => Some(Operation::Increment(right)),
        (Some("dec"), Some(right)) => Some(Operation::Decrement(right)),
        _           => None,
    };

    let condition = match parts.next() {
        Some(last_part) => parse_condition(last_part),
        None => None,
    };

    match (target, operation, condition) {
        (Some(target), Some(operation), Some(condition)) => {
            Some(Instruction{target, operation, condition})
        },
        _ => None
    }
}

pub fn max(input: &[String]) -> i32 {
    let mut registers = Registers::new();

    input.into_iter()
        .filter_map(|str| parse_instruction(str))
        .for_each(|instruction| instruction.execute(&mut registers));

    registers.into_iter().map(|(_, n)| n).max().unwrap()
}
