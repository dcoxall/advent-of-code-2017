extern crate stream;

use stream::garbage;
use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let lines: String = handle.lines().filter_map(|line| line.ok()).collect();
    println!("{:?}", garbage(&lines));
}
