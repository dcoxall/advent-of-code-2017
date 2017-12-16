extern crate permutationpromenade;

use permutationpromenade::dance;
use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut dancers = vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
        'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
    ];
    let lines: Vec<_> = handle.lines().filter_map(|line| line.ok()).collect();
    let moves: Vec<_> = lines.iter().flat_map(|line| line.split(',')).collect();
    dance(&moves, &mut dancers);
    let result: String = dancers.iter().collect();
    println!("{}", result);
}
