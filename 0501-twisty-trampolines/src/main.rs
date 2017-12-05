extern crate twistytrampoline;

use twistytrampoline::steps;
use std::io;

fn main() {
    let mut jumps: Vec<i32> = Vec::new();
    loop {
        match fetch_row() {
            Some(n) => jumps.push(n),
            None    => break,
        }
    }
    println!("{:?}", steps(&mut jumps));
}

fn fetch_row() -> Option<i32> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_)  => input.trim().parse().ok(),
        Err(_) => None,
    }
}
