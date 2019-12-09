use std::fs::File;
use std::io::{BufRead, BufReader};

fn to_lower(c: char) -> char {
    return c.to_lowercase().next().unwrap();
}

fn get_input(filename: &str) -> String {
    let file = File::open(filename).expect("Failed to open file");
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    reader.read_line(&mut line).expect("Failed to read line");
    return String::from(line.trim());
}

fn would_react(pair: &str) -> bool {
    let mut iter = pair.chars();
    let c1 = iter.next().unwrap();
    let c2 = iter.next().unwrap();

    if c1.to_lowercase().to_string() != c2.to_lowercase().to_string() {
        return false;
    }

    if (c1.is_uppercase() && c2.is_uppercase()) || (c1.is_lowercase() && c2.is_lowercase()) {
        return false;
    }

    return true;
}

fn react(chain: &str) -> String {
    let mut cur = String::from(chain);
    let mut nex = String::new();
    loop {
        let mut i = 0;
        while i < cur.len() {
            if i == cur.len() - 1 {
                // Reached the end of the string. We've already checked that this
                // char should be in, so just add it and finish.
                let c = cur[i..i + 1].chars().next().unwrap();
                nex.push(c);
                break;
            } else {
                let slice = &cur[i..=i + 1];
                if would_react(slice) {
                    // Found a reaction, skip the two reacting chars.
                    i += 2;
                    continue;
                } else {
                    let c = slice.chars().next().unwrap();
                    nex.push(c);
                    i += 1;
                }
            }
        }

        if nex.len() == cur.len() {
            break;
        }

        cur = nex.clone();
        nex.clear();
    }

    return cur;
}

fn filter_type(chain: &str, filter_char: char) -> String {
    return chain
        .chars()
        .filter(|c| to_lower(*c) != to_lower(filter_char))
        .collect();
}

fn main() {
    let chain = get_input("input");

    let result = (b'a'..=b'z')
        .map(char::from)
        .map(|c: char| {
            let new_chain = filter_type(chain.as_ref(), c);
            let final_chain = react(&new_chain);
            return final_chain.len();
        })
        .min()
        .unwrap();

    println!("Final chain length: {}", result);
}
