use std::fs;

pub fn solution() {
    let aoc_2024_a =
        fs::read_to_string("./inputs/day02.txt").expect("Something went wrong reading the file");

    // A.
    let mut safe_reports_a = 0;
    let mut safe_reports = Vec::<usize>::new();
    for (idx, line) in aoc_2024_a.lines().enumerate() {
        let numbers = line
            .split_whitespace()
            .map(|u| u.parse::<i8>().unwrap())
            .collect::<Vec<i8>>();
        let ascending = |a: &i8, b: &i8| *a < *b && *b <= (*a + 3);
        let descending = |a: &i8, b: &i8| *a > *b && *b >= (*a - 3);

        if numbers.is_sorted_by(ascending) || numbers.is_sorted_by(descending) {
            safe_reports_a = safe_reports_a + 1;

            // collect the indexes of the unsafe reports
            safe_reports.push(idx);
        }
    }
    println!("Number of safe reports is: {}", safe_reports_a);

    // B.
    let asc = |levels: &[i8]| {
        levels
            .windows(2)
            .all(|w| w[1] > w[0] && (1..=3).contains(&(w[1] - w[0])))
    };
    let desc = |levels: &[i8]| {
        levels
            .windows(2)
            .all(|w| w[1] < w[0] && (1..=3).contains(&(w[0] - w[1])))
    };

    let is_safe = |levels: &[i8]| -> bool {
        (asc(levels) || desc(levels))
            || (0..levels.len()).any(|i| {
                let mut levels = levels.to_vec();
                levels.remove(i);
                asc(&levels) || desc(&levels)
            })
    };

    let levels = aoc_2024_a
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|u| u.parse::<i8>().unwrap())
                .collect::<Vec<i8>>()
        })
        .collect::<Vec<Vec<i8>>>();

    let count = levels.into_iter().filter(|l| is_safe(l)).count();
    println!("Number of safe reports is: {}", count);
}
