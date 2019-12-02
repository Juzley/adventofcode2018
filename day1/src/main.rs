use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input").expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut counts: HashSet<i64> = HashSet::new();
    let mut current = 0;
    counts.insert(current);

    let mut freqs: Vec<i64> = Vec::new();
    for line in reader.lines() {
        let freq = line.unwrap().parse::<i64>().unwrap();
        freqs.push(freq);
    }

    loop {
        for freq in &freqs {
            current += freq;

            if counts.contains(&current) {
                println!("Repeated frequency: {}", current);
                return;
            }

            counts.insert(current);
        }
    }
}
