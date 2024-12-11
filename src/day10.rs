use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

pub fn solution() {
    let aoc_2024_10 =
        File::open("./inputs/day10.txt").expect("Something went wrong reading the file");
    let grid = io::BufReader::new(aoc_2024_10)
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).map(|d| d as usize).unwrap_or(99))
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    let zero_positions: Vec<(i32, i32)> = (0..rows)
        .flat_map(|i| (0..cols).map(move |j| (i, j)))
        .filter(|&(i, j)| grid[i as usize][j as usize] == 0)
        .collect();

    let mut sum_of_scores = 0;
    let mut sum_of_ratings = 0;
    for (i, j) in zero_positions {
        let mut queue = Vec::new();
        queue.push((0, (i, j)));
        let mut scores: HashSet<(i32, i32)> = HashSet::new();
        let directions = [(-1, 0), (1, 0), (0, 1), (0, -1)];
        while let Some((actual_value, (x, y))) = queue.pop() {
            let next_value = actual_value + 1;

            for &(dx, dy) in &directions {
                let next_x = x + dx;
                let next_y = y + dy;

                if let Some(actual_value) = get_value(&grid, rows, cols, next_x, next_y) {
                    if actual_value == next_value {
                        if actual_value == 9 {
                            scores.insert((next_x, next_y));
                            sum_of_ratings += 1;
                        } else {
                            queue.push((next_value, (next_x, next_y)));
                        }
                    }
                }
            }
        }
        sum_of_scores += scores.len();
    }

    println!("matches a {}", sum_of_scores);
    println!("matches b {}", sum_of_ratings);
}

fn get_value(grid: &Vec<Vec<usize>>, rows: i32, cols: i32, r: i32, c: i32) -> Option<usize> {
    if r >= 0 && r < rows && c >= 0 && c < cols {
        Some(grid[r as usize][c as usize])
    } else {
        None
    }
}
