extern crate knothash;

use knothash::digest;
use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let line = handle.lines().next().unwrap().unwrap();
    println!("{:?}", digest(&line));
}
