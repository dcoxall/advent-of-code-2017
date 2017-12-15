extern crate duelinggenerators;

use duelinggenerators::pairs;
use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let input: Vec<_> = handle.lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| line[24..].parse::<u64>().ok())
        .collect();
    println!("{:?}", pairs(40_000_000, input[0], input[1]));
}
