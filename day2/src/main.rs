use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn count_repeats(word: &String) -> (u64, u64) {
    let mut counts: HashMap<char, u64> = HashMap::new();

    for c in word.chars() {
        let count = counts.entry(c).or_insert(0);
        *count += 1;
    }

    let result = counts.iter().fold((0, 0), |acc, (_, count)| {
        let two = if acc.0 > 0 || *count == 2 { 1 } else { 0 };
        let three = if acc.1 > 0 || *count == 3 { 1 } else { 0 };
        (two, three)
    });
    return result;
}

fn get_words() -> Vec<String> {
    let file = File::open("input").expect("Failed to open file");
    let reader = BufReader::new(file);

    return reader
        .lines().map(|l| l.expect("Failed to get line"))
        .collect();
}

fn find_near(word: &String, words: &[String]) -> Vec<String> {
    return words.iter()
        .map(|candidate| {
            candidate
                .chars()
                .zip(word.chars())
                .filter(|(c1, c2)| c1 == c2)
                .map(|(c, _)| c)
                .collect()
        })
        .filter(|w: &String| w.len() == word.len() - 1)
        .collect();
}

fn find_all_near(words: &Vec<String>) -> Vec<String> {
    let mut results: Vec<String> = Vec::new();

    // Brute force, just compare every word with every other word.
    for i in 0..words.len() {
        results.extend(find_near(&words[i], &words[i+1..]));
    }

    return results;
}

fn main() {
    let words = get_words();

    // Calc checksum
    let result = words
        .iter()
        .map(|s| count_repeats(s))
        .fold((0, 0), |acc, (twos, threes)| (acc.0 + twos, acc.1 + threes));

    println!("Checksum {}", result.0 * result.1);

    // Find correct words.
    let results = find_all_near(&words);
    for r in results {
        println!("Candidate: {}", r);
    }
}
