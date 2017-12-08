extern crate registers;

use registers::max;
use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let lines: Vec<String> = handle.lines().filter_map(|line| line.ok()).collect();
    println!("{:?}", max(&lines));
}
