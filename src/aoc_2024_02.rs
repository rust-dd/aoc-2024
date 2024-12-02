use std::fs;

pub fn solution() {
    let aoc_2024_a =
        fs::read_to_string("./src/aoc_2024_02.txt").expect("Something went wrong reading the file");

    // A.
    let mut safe_reports_a = 0;
    for line in aoc_2024_a.lines() {
        let numbers = line
            .split_whitespace()
            .map(|u| u.parse::<i8>().unwrap())
            .collect::<Vec<i8>>();
        let ascending = |a: &i8, b: &i8| *a < *b && *b <= (*a + 3);
        let descending = |a: &i8, b: &i8| *a > *b && *b >= (*a - 3);

        if numbers.is_sorted_by(ascending) || numbers.is_sorted_by(descending) {
            safe_reports_a = safe_reports_a + 1;
        }
    }
    println!("Number of safe reports is: {}", safe_reports_a);

    // B.
  
}
