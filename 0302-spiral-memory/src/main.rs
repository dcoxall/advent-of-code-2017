extern crate spiralmemory;

use spiralmemory::larger_than;
use std::env;

fn main() {
    match env::args().last() {
        Some(input) => {
            let input = input.parse::<u32>().expect("Must be a number");
            println!("{:?}", larger_than(input))
        },
        None => println!("No input"),
    }
}
