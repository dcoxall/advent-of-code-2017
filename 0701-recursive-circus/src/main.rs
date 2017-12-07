extern crate recursivecircus;
extern crate regex;

use regex::Regex;
use recursivecircus::bottom;
use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let re = Regex::new(r"^(?P<name>\w+) \((?P<weight>\d+)\)(?: -> )?(?P<children>.*?)?$").unwrap();
    let input: Vec<(String, u32, Vec<String>)> = handle.lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            let caps = re.captures(line.as_str()).unwrap();
            let weight: u32 = caps["weight"].parse().expect("This should be a number");
            if let Some(children) = caps.name("children") {
                let children: Vec<String> = children.as_str().split(",").map(|name| name.trim().to_string()).collect();
                (caps["name"].to_string(), weight, children)
            } else {
                (caps["name"].to_string(), weight, vec![])
            }
        }).collect();

    println!("{}", bottom(&input));
}
