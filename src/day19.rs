use std::fs;
use std::io::{self, BufRead};
use std::collections::HashMap;

fn count_ways(onsen_part: &str, patterns: &[String], memo: &mut HashMap<String, u64>) -> u64 {
    if onsen_part.is_empty() {
        return 1;
    }

    if let Some(&count) = memo.get(onsen_part) {
        return count;
    }

    let mut total = 0;
    for pattern in patterns {
        if onsen_part.starts_with(pattern) {
            let remainder = &onsen_part[pattern.len()..];
            total += count_ways(remainder, patterns, memo);
        }
    }
    memo.insert(onsen_part.to_string(), total);
    total
}

pub fn solution() {
    let file = fs::File::open("./inputs/day19.txt").expect("Failed to open file");
    let reader = io::BufReader::new(file);
    let mut lines_iter = reader.lines().map(|line| line.expect("Failed to read line"));
    let towel_pattern_line = lines_iter.next().expect("No towel patterns found");
    let towel_patterns: Vec<String> = towel_pattern_line
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    // blank line
    lines_iter.next();

    let onsens: Vec<String> = lines_iter.collect();

    let mut total_match_count = 0u64;
    let mut onsen_count_that_can_match = 0u64;

    for onsen in &onsens {
        let mut memo: HashMap<String, u64> = HashMap::new();
        let ways = count_ways(onsen, &towel_patterns, &mut memo);
        if ways > 0 {
            onsen_count_that_can_match += 1;
            total_match_count += ways;
        }
    }

    println!("Result A: {}", onsen_count_that_can_match);
    println!("Result B: {}", total_match_count);
}