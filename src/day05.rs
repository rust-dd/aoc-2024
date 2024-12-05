use std::fs;

pub fn solution() {
    let aoc_2024_04 =
        fs::read_to_string("./inputs/day05.txt").expect("Something went wrong reading the file");

    let mut parts = aoc_2024_04.split("\n\n");
    let page_ordering_rules_text = parts.next().unwrap();
    let page_ordering_rules: Vec<(i8, i8)> = page_ordering_rules_text
        .lines()
        .map(|line| {
            let mut numbers = line.split('|').map(|n| n.parse::<i8>().unwrap());
            (numbers.next().unwrap(), numbers.next().unwrap())
        })
        .collect();

    let pages_to_reproduce_text = parts.next().unwrap();
    let pages_to_reproduce: Vec<Vec<i8>> = pages_to_reproduce_text
        .lines()
        .map(|line| line.split(',').map(|n| n.parse::<i8>().unwrap()).collect())
        .collect();

    // A.
    let sum: i32 = pages_to_reproduce
        .iter()
        .filter(|page| {
            page_ordering_rules.iter().all(|&(left_rule, right_rule)| {
                let left_index = page.iter().position(|&r| r == left_rule);
                let right_index = page.iter().position(|&r| r == right_rule);

                match (left_index, right_index) {
                    (Some(left), Some(right)) => left <= right,
                    _ => true,
                }
            })
        })
        .map(|page| *(page.get(page.len() / 2).unwrap()) as i32)
        .sum();
    println!("Result: {}", sum);

    // B.

    println!("Result: {}", sum);

}
