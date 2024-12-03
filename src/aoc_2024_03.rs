use std::fs;

use regex::Regex;

pub fn solution() {
    let aoc_2024_a =
        fs::read_to_string("./src/aoc_2024_03.txt").expect("Something went wrong reading the file");

    // A.
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let caps = re.captures_iter(&aoc_2024_a);
    
    let mut sum = 0;
    for cap in caps {
        println!("{} - {}", cap.get(1).unwrap().as_str(), cap.get(2).unwrap().as_str());
        let first_value = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let second_value = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
        sum = sum + (first_value * second_value);
    }
    println!("Result: {}", sum);

}
