extern crate corruptionchecksum;

use corruptionchecksum::corruption_checksum;
use std::io;
use std::io::Error;

fn main() {
    let mut input: Vec<Vec<u32>> = Vec::new();
    loop {
        match fetch_row() {
            Ok(row)    => {
                if row.is_empty() {
                    break;
                }
                input.push(row)
            },
            Err(_) => break,
        }
    }
    println!("{:?}", corruption_checksum(input));
}

fn fetch_row() -> Result<Vec<u32>, Error> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).map(|_| {
        input.split_whitespace()
            .map(|num| num.parse::<u32>().expect("Should only be a number"))
            .collect()
    })
}
