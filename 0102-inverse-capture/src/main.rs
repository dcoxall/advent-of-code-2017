extern crate inversecapture;

use inversecapture::inverse_capture;
use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    {
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        handle.read_to_string(&mut input);
    }
    let input: Vec<u32> = input.chars().filter_map(|c| c.to_digit(10)).collect();
    println!("{:?}", inverse_capture(input));
}
