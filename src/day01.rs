use std::fs;
use std::str::FromStr;

pub fn solution() {
    // A.
    let aoc_2024_01 =
        fs::read_to_string("./inputs/day01.txt").expect("Something went wrong reading the file");
    let mut a = Vec::new();
    let mut b = Vec::new();

    for line in aoc_2024_01.lines() {
        // split by tab
        let mut parts = line.split_whitespace();
        a.push(<i32>::from_str(parts.next().unwrap()).unwrap());
        b.push(<i32>::from_str(parts.next().unwrap()).unwrap());
    }

    // unstable sorting is faster, and we can use this because the inputs are integers
    a.sort_unstable();
    b.sort_unstable();

    // calculate the difference between the two arrays
    let diff = a
        .iter()
        .zip(&b)
        .map(|(a, b)| (a - b).abs())
        .collect::<Vec<i32>>()
        .iter()
        .sum::<i32>();
    println!("The difference is: {}", diff);

    // B.
    let sum = a
        .iter()
        .map(|x| x * b.iter().filter(|&y| x == y).count() as i32)
        .sum::<i32>();

    println!("The sum of duplicates is: {}", sum);
}
