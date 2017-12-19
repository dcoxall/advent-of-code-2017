extern crate tubeseries;

use tubeseries::steps;
use std::io;
use std::io::Read;

fn main() {
    let mut buff: Vec<u8> = Vec::new();
    io::stdin().read_to_end(&mut buff).expect("No input found");
    println!("{:?}", steps(&buff));
}
