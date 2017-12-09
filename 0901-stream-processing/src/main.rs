extern crate stream;

use stream::score;
use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let lines: String = handle.lines().filter_map(|line| line.ok()).collect();
    println!("{:?}", score(&lines));
}
