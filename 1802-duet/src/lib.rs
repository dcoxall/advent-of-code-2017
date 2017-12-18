use std::collections::HashMap;
use std::time::Duration;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

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

pub fn execute(input: &[String]) -> isize {
    use Instruction::*;

    let mut thread_handles: Vec<_> = Vec::with_capacity(2);

    // This channel is used by the main thread to extract information
    let (master_sender, master_receiver): (Sender<(usize, isize)>, Receiver<(usize, isize)>) = mpsc::channel();

    // We want 2 senders and 2 recievers
    let channels: Vec<(Sender<isize>, Arc<Mutex<Receiver<isize>>>)> = (0..2).map(|_| {
        let (sender, receiver): (Sender<isize>, Receiver<isize>) = mpsc::channel();
        (sender, Arc::new(Mutex::new(receiver)))
    }).collect();

    for prog_id in 0..2 {
        let master_sender = master_sender.clone();
        let send_channel = channels[prog_id].0.clone();
        let receive_channel = Arc::clone(&channels[(prog_id + 1) % 2].1);
        let instructions: Vec<_> = input.iter().map(Instruction::from).collect();
        let handle = thread::spawn(move || {
            let mut registers: Registers = HashMap::new();
            registers.insert('p', prog_id as isize);

            let mut i: isize = 0;
            while let Some(instruction) = instructions.get(i as usize) {
                match instruction {
                    &Snd(ref a) => {
                        let val = a.resolve(&mut registers);
                        send_channel.send(val).unwrap();
                        master_sender.send((prog_id, val)).unwrap();
                    },
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
                    &Rcv(ref a) => {
                        let rec = receive_channel.lock().unwrap();
                        match rec.recv_timeout(Duration::from_secs(1)) {
                            Ok(n)  => a.set(&mut registers, n),
                            Err(_) => {
                                // move the address out of the list
                                i = instructions.len() as isize;
                                continue;
                            },
                        };
                    },
                }

                i += 1;
            }
        });
        thread_handles.push(handle);
    }

    for handle in thread_handles {
        handle.join().unwrap();
    }

    master_receiver.try_iter().filter(|&(id, _)| id == 1).count() as isize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = vec![
            String::from("snd 1"),
            String::from("snd 2"),
            String::from("snd p"),
            String::from("rcv a"),
            String::from("rcv b"),
            String::from("rcv c"),
            String::from("rcv d"),
        ];

        assert_eq!(execute(&input), 3);
    }
}
