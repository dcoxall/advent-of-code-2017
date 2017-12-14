extern crate diskdefrag;

use diskdefrag::used;
use std::env;

fn main() {
    match env::args().last() {
        Some(input) => {
            println!("{:?}", used(&input))
        },
        None => println!("No input"),
    }
}
