extern crate duet;

use duet::recovered;
use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let lines: Vec<String> = handle.lines().filter_map(|line| line.ok()).collect();
    println!("{:?}", recovered(&lines));
}
