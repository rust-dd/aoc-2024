use std::fs;

use regex::Regex;

pub fn solution() {
    let aoc_2024_03 =
        fs::read_to_string("./src/aoc_2024_03.txt").expect("Something went wrong reading the file");

    // A.
    let matching_regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let caps = matching_regex.captures_iter(&aoc_2024_03);
    
    let mut sum = 0;
    for cap in caps {
        let first_value = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let second_value = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
        sum = sum + (first_value * second_value);
    }
    println!("Result: {}", sum);

    // B.
    let text_without_new_lines = aoc_2024_03.replace("\n", "to");
    let regex_do_not_do = Regex::new(r"don't.*?(do\(\)|$)").unwrap();
    let text_without_do_not_do = regex_do_not_do.replace_all(&text_without_new_lines, "");
    let matching_regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let caps = matching_regex.captures_iter(&text_without_do_not_do);
    let mut sum = 0;
    for cap in caps {
        let first_value = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let second_value = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
        sum = sum + (first_value * second_value);
    }

    println!("Result: {}", sum);

}
