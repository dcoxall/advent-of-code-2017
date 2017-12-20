extern crate particleswarm;

use particleswarm::remaining;
use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let lines: Vec<String> = handle.lines().filter_map(|line| line.ok()).collect();
    println!("{:?}", remaining(&lines));
}
