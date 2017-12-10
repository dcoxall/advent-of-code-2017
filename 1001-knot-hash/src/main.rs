extern crate knothash;

use knothash::confirm;
use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let line = handle.lines().next().unwrap().unwrap();
    let lengths: Vec<_> = line.split(',')
        .filter_map(|val| val.parse().ok())
        .collect();
    let mut elements: Vec<_> = (0..256).collect();
    println!("{:?}", confirm(&mut elements, &lengths));
}
