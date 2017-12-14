extern crate diskdefrag;

use diskdefrag::groups;
use std::env;

fn main() {
    match env::args().last() {
        Some(input) => {
            println!("{:?}", groups(&input))
        },
        None => println!("No input"),
    }
}
