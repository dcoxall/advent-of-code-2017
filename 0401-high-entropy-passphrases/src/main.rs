extern crate highentropypass;

use highentropypass::is_valid;
use std::io;
use std::io::Result;

fn main() {
    let mut valid_count = 0;
    loop {
        match fetch_row() {
            Ok(passphrase)    => {
                if passphrase.is_empty() {
                    break;
                } else if is_valid(passphrase) {
                    valid_count += 1;
                }
            },
            Err(_) => break,
        }
    }
    println!("{:?}", valid_count);
}

fn fetch_row() -> Result<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).map(|_| String::from(input.trim()))
}
