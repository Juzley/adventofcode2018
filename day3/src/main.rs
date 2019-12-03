use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Rect {
    id: u64,
    left: u64,
    top: u64,
    width: u64,
    height: u64,
}

fn parse_rect(re: &Regex, rect: &str) -> Rect {
    let caps = re.captures(rect).expect("No regex match");
    let parse = |key: &str| caps[key].parse::<u64>().unwrap();
    return Rect {
        id: parse("id"),
        left: parse("left"),
        top: parse("top"),
        width: parse("width"),
        height: parse("height"),
    };
}

fn read_rects() -> Vec<Rect> {
    let file = File::open("input").expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut rects = Vec::new();
    let re = Regex::new(r"#(?P<id>\d+) @ (?P<left>\d+),(?P<top>\d+): (?P<width>\d+)x(?P<height>\d+)").unwrap();
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let line = line.trim();
        let rect = parse_rect(&re, &line);
        rects.push(rect);
    }

    return rects;
}

fn main() {
    let mut covered: HashMap<(u64, u64), Vec<u64>> = HashMap::new();
    let mut unique_ids: HashSet<u64> = HashSet::new();
    for rect in read_rects() {
        unique_ids.insert(rect.id);
        for x in 0..rect.width {
            for y in 0..rect.height {
                let x = rect.left + x + 1;
                let y = rect.top + y + 1;
                let coord = (x, y);

                if let Some(ids) = covered.get_mut(&coord) {
                    ids.push(rect.id);
                } else {
                    covered.insert(coord, vec![rect.id]);
                }
            }
        }
    }

    for (_, ids) in covered {
        if ids.len() > 1 {
            for id in ids {
                unique_ids.remove(&id);
            }
        }
    }

    let result = unique_ids.iter().next().expect("No unique ids");
    println!("Unique id: {}", result);
}
