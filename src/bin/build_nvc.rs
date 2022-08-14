use lib::Nvc;
use std::io::{self, BufRead};

fn main() {
    let mut words = Vec::new();
    let mut nvc = Nvc::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            nvc.insert(&line);
            words.push(line);
        }
    }
}
