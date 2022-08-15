use lib::HashMapNvc;
use lib::NextValidCharacter;

use std::io::{self, BufRead};

fn main() {
    let mut words = Vec::new();
    let mut nvc = HashMapNvc::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            nvc.parse(&line);
            words.push(line);
        }
    }
}
