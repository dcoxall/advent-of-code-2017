extern crate hexed;

use hexed::distance;
use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let line = handle.lines().next().unwrap().unwrap();
    println!("{:?}", distance(&line));
}
