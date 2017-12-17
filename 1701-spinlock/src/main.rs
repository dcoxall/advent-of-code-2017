extern crate spinlock;
use spinlock::{CircularBuffer, fill_buffer};
use std::env;

fn main() {
    if let Some(n) = env::args().last() {
        let mut buffer = CircularBuffer::new(n.parse().unwrap());
        fill_buffer(&mut buffer, 2017);
        println!("{:?}", buffer.peek());
    }
}
