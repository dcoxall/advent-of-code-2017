extern crate spiralmemory;

use spiralmemory::distance;
use std::env;

fn main() {
    match env::args().last() {
        Some(input) => {
            let input = input.parse::<u32>().expect("Must be a number");
            println!("{:?}", distance(input))
        },
        None => println!("No input"),
    }
}
