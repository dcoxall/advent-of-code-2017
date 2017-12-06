extern crate memoryrealloc;

use memoryrealloc::realloc;
use std::io;

fn main() {
    if let Some(line) = fetch_row() {
        let mut banks: Vec<u32> = line.split_whitespace()
            .filter_map(|s| s.parse::<u32>().ok())
            .collect();
        println!("{:?}", realloc(&mut banks));
    }
}

fn fetch_row() -> Option<String> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_)  => Some(String::from(input.trim())),
        Err(_) => None,
    }
}
