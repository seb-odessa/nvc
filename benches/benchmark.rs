use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lib::Nvc;
use std::collections::HashSet;
use std::io::{self, BufRead};

use std::fs::File;
use std::io::BufReader;

fn read_lines(filename: &str) -> io::Result<Vec<String>> {
    let mut lines = Vec::new();
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        if let Ok(line) = line {
            lines.push(line);
        }
    }
    return Ok(lines);
}

fn build_nvc(lines: &Vec<String>) -> Nvc {
    let mut nvc = Nvc::new();
    for line in lines {
        nvc.insert(&line);
    }
    return nvc;
}

fn build_filtered((lines, allowed): (&Vec<String>, &HashSet<char>)) -> Nvc {
    let mut nvc = Nvc::new();
    for line in lines {
        nvc.insert_filtered(&line, &allowed);
    }
    return nvc;
}

fn criterion_benchmark(c: &mut Criterion) {
    let lines = read_lines("./data/titles.txt").unwrap();
    let alphabet = vec![
        'й', 'ц', 'у', 'к', 'е', 'ё', 'н', 'г', 'ш', 'щ', 'з', 'х', 'ъ', 'ф', 'ы', 'в', 'а', 'п',
        'р', 'о', 'л', 'д', 'ж', 'э', 'я', 'ч', 'с', 'м', 'и', 'т', 'ь', 'б', 'ю', '.', ',', '!',
        '?', ';', ':',
    ];
    let mut allowed = alphabet.iter().cloned().collect::<HashSet<char>>();
    for ch in &alphabet {
        if ch.is_alphabetic() {
            for up in ch.to_uppercase().to_string().chars() {
                allowed.insert(up);
            }
        }
    }

    let mut russians = Vec::new();
    for line in &lines {
        let chars = line.chars();
        // if chars.all(|c| allowed.contains(&c)) {
        let nvc = chars.collect::<Vec<char>>();
        russians.push(nvc);
        // }
    }

    c.bench_function("Build NVC", |b| b.iter(|| build_nvc(black_box(&lines))));
    c.bench_function("Build NVC Filtered", |b| {
        b.iter(|| build_filtered(black_box((&lines, &allowed))))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
