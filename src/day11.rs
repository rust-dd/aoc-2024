use ahash::AHashMap;
use rayon::prelude::*;

pub fn solution() {
    let input = std::fs::read_to_string("./inputs/day11.txt").expect("Failed to read input file");

    let parse = |input: &str| {
        input
            .split_whitespace()
            .filter_map(|x| x.parse::<usize>().ok())
            .fold(AHashMap::new(), |mut acc, x| {
                *acc.entry(x).or_insert(0) += 1;
                acc
            })
    };

    let run = |stones: &mut AHashMap<usize, usize>, iter: &mut i32| {
        while *iter > 0 {
            let mut next_stones = AHashMap::new();

            for (stone, count) in stones.iter() {
                if *stone == 0 {
                    *next_stones.entry(1).or_insert(0) += count;
                } else if stone.to_string().len() % 2 == 0 {
                    let stone_str = stone.to_string();
                    let mid = stone_str.len() / 2;
                    let left = stone_str[..mid].parse::<usize>().unwrap();
                    let right = stone_str[mid..].parse::<usize>().unwrap();
                    *next_stones.entry(left).or_insert(0) += count;
                    *next_stones.entry(right).or_insert(0) += count;
                } else {
                    let new_stone = stone * 2024;
                    *next_stones.entry(new_stone).or_insert(0) += count;
                }
            }

            *stones = next_stones;
            *iter -= 1;
        }

        println!("Result : {}", stones.values().sum::<usize>());
    };

    // A.
    let mut stones = parse(&input);
    let mut iterations = 25;
    run(&mut stones, &mut iterations);

    // B.
    iterations = 75;
    let mut stones = parse(&input);
    run(&mut stones, &mut iterations);
}

pub fn solution_par() {
    let input = std::fs::read_to_string("./inputs/day11.txt").expect("Failed to read input file");
    let parse = |input: &str| {
        input
            .split_whitespace()
            .filter_map(|x| x.parse::<usize>().ok())
            .fold(AHashMap::new(), |mut acc, x| {
                *acc.entry(x).or_insert(0) += 1;
                acc
            })
    };

    let run = |stones: &mut AHashMap<usize, usize>, iter: &mut i32| {
        while *iter > 0 {
            let next_stones = stones
                .par_iter()
                .flat_map(|(&stone, &count)| {
                    if stone == 0 {
                        vec![(1, count)]
                    } else if stone.to_string().len() % 2 == 0 {
                        let stone_str = stone.to_string();
                        let mid = stone_str.len() / 2;
                        let left = stone_str[..mid].parse::<usize>().unwrap();
                        let right = stone_str[mid..].parse::<usize>().unwrap();
                        vec![(left, count), (right, count)]
                    } else {
                        let new_stone = stone * 2024;
                        vec![(new_stone, count)]
                    }
                })
                .fold(AHashMap::new, |mut acc, (stone, count)| {
                    *acc.entry(stone).or_insert(0) += count;
                    acc
                })
                .reduce_with(|mut acc, map| {
                    for (stone, count) in map {
                        *acc.entry(stone).or_insert(0) += count;
                    }
                    acc
                })
                .unwrap_or_default();

            *stones = next_stones;
            *iter -= 1;
        }

        println!("Result: {}", stones.values().sum::<usize>());
    };

    // A.
    let mut stones = parse(&input);
    let mut iterations = 25;
    run(&mut stones, &mut iterations);

    // B.
    iterations = 75;
    let mut stones = parse(&input);
    run(&mut stones, &mut iterations);
}
