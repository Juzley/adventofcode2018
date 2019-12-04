use chrono::{NaiveDateTime, Timelike};
use regex::{Captures, Regex};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

enum LogEvent {
    UNKNOWN,
    START,
    SLEEP,
    WAKE,
}

struct LogEntry {
    timestamp: NaiveDateTime,
    event: LogEvent,
    id: Option<u32>,
}

struct GuardInfo {
    id: u32,
    sleeps: Vec<(u32, u32)>,
}

// Returns a sorted list of log events.
fn parse_log() -> Vec<LogEntry> {
    let file = File::open("input").expect("Failed to open file");
    let reader = BufReader::new(file);

    let guard_re = Regex::new(r"\[(?P<timestamp>.*)\] Guard #(?P<id>\d+) .*").unwrap();
    let sleep_re = Regex::new(r"\[(?P<timestamp>.*)\] (?P<event>.*)").unwrap();

    let mut events = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let line = line.trim();

        let parse_timestamp = |caps: &Captures| {
            return NaiveDateTime::parse_from_str(&caps["timestamp"], "%Y-%m-%d %H:%M")
                .expect("Failed to parse timestamp");
        };

        if let Some(caps) = guard_re.captures(line) {
            // Changing of the guard
            events.push(LogEntry {
                event: LogEvent::START,
                timestamp: parse_timestamp(&caps),
                id: Some(caps["id"].parse::<u32>().unwrap()),
            });
        } else if let Some(caps) = sleep_re.captures(line) {
            // A guard sleeping/waking up.
            let event = match &caps["event"] {
                "wakes up" => LogEvent::WAKE,
                "falls asleep" => LogEvent::SLEEP,
                _ => LogEvent::UNKNOWN,
            };
            events.push(LogEntry {
                event: event,
                timestamp: parse_timestamp(&caps),
                id: None,
            });
        }
    }

    events.sort_by(|e1, e2| e1.timestamp.cmp(&e2.timestamp));
    return events;
}

fn process_log(events: &[LogEntry]) -> Vec<GuardInfo> {
    let mut guard_info = HashMap::new();
    let mut current_guard = 0;
    let mut sleep_time = 0;
    for e in events {
        match e.event {
            LogEvent::START => {
                current_guard = e.id.expect("Start event with no id");
                if !guard_info.contains_key(&current_guard) {
                    guard_info.insert(current_guard, GuardInfo {
                        id: current_guard,
                        sleeps: Vec::new(),
                    });
                }
            },
            LogEvent::SLEEP => {
                sleep_time = e.timestamp.minute();
            },
            LogEvent::WAKE => {
                let info = guard_info.get_mut(&current_guard).unwrap();                
                info.sleeps.push((sleep_time, e.timestamp.minute()));
            },
            _ => (),
        }
    }

    return guard_info.into_iter().map(|(_, g)| g).collect();
}

fn total_sleep(guard: &GuardInfo) -> u32 {
    return guard.sleeps.iter().map(|(start, end)| end - start).sum();
}

fn sleepiest_minute(guard: &GuardInfo) -> u32 {
    let mut minutes: [u32; 60] = [0; 60];
    for (start, end) in &guard.sleeps {
        for minute in *start..*end {
            minutes[minute as usize] += 1;
        }
    }

    let mut max_sleeps = 0;
    let mut max_minute = 0;
    for m in 0..minutes.len() {
        if minutes[m] > max_sleeps {
            max_minute = m as u32;
            max_sleeps = minutes[m];
        }
    }
    
    return max_minute;
}

// Find the minute where a single guard has been alseep the most times,
// the id of that guard, and how many times they had slept in that
// minute for.
fn find_best_minute(guards: &[GuardInfo]) -> (u32, u32, u32) {
    // Ideally would use an array of hashmaps here, but it's awkward to
    // initialize, so just fill a vector with 60 items.
    let mut minutes: Vec::<HashMap<u32, u32>> = Vec::new();
    for _i in 0..60 {
        minutes.push(HashMap::new());
    }

    // For each minute, build a map from the guard ids that have been asleep
    // on that minute to the number of times they have been asleep
    // on that minute.
    for guard in guards {
        for (start, end) in &guard.sleeps {
            for minute in *start..*end {
                let map = &mut minutes[minute as usize];
                if let Some(count) = map.get_mut(&guard.id) {
                    *count += 1;
                } else {
                    map.insert(guard.id, 1);
                }
            }
        }
    }

    let minutes_vec: Vec<(u32, u32, u32)> = minutes
        .into_iter()
        .enumerate()
        .map(|(min, map)| {
            // Find the guard who has been asleep during this minute the most times.
            let mut max_id = 0;
            let mut max_count = 0;
            for (id, count) in map.iter() {
                if *count > max_count {
                    max_id = *id;
                    max_count = *count;
                }
            }
            (min as u32, max_id, max_count)
        })
        .collect();

    // Find the minute with the highest sleep count by a single guard.
    let top = minutes_vec.iter().max_by(|(_, _, count1), (_, _, count2)| count1.cmp(count2)).unwrap();

    return top.clone();
}

fn main() {
    let events = parse_log();
    let mut guards = process_log(&events);

    // Part 1: find the guard who is asleep the most.
    guards.sort_by(|g1, g2| total_sleep(g2).cmp(&total_sleep(g1)));
    let sleepiest_guard = guards.first().unwrap();
    let sleepiest_minute = sleepiest_minute(sleepiest_guard);
    println!("Sleepiest guard: {}, Sleepiest minute: 00:{:2}, Answer: {}",
        sleepiest_guard.id, sleepiest_minute, sleepiest_guard.id * sleepiest_minute);

    // Part 2: Find the guard who sleeps the most for a single minute.
    let (best_minute, sleepiest_guard, sleep_count) = find_best_minute(&guards);
    println!("Best minute: {}, Sleepiest guard: {}, Sleep Count: {}, Answer: {}",
        best_minute, sleepiest_guard, sleep_count, best_minute * sleepiest_guard);
}
