extern crate digitalplumber;

use digitalplumber::groups;
use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let input: Vec<_> = handle.lines()
        .filter_map(|line| line.ok())
        .collect();
    println!("{:?}", groups(&input));
}
