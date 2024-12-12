use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

pub fn solution() {
    let aoc_2024_12 = File::open("./inputs/day12.txt").expect("Failed to open file");
    let grid: Vec<Vec<char>> = io::BufReader::new(aoc_2024_12)
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.chars().collect())
        .collect();

    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    let mut visited = HashSet::new();
    let directions = [(-1, 0), (1, 0), (0, 1), (0, -1)];

    let mut total_sum = 0;

    for r in 0..rows {
        for c in 0..cols {
            if visited.contains(&(r, c)) {
                continue;
            }

            let mut queue = vec![(r, c)];
            visited.insert((r, c));

            let mut group_size = 0;
            let mut perimeter = 0;

            while let Some((x, y)) = queue.pop() {
                let value = grid[x as usize][y as usize];
                let mut local_perimeter = 4;
                group_size += 1;

                for &(dx, dy) in &directions {
                    let nx = x + dx;
                    let ny = y + dy;

                    if nx >= 0 && nx < rows && ny >= 0 && ny < cols {
                        if grid[nx as usize][ny as usize] == value {
                            if visited.insert((nx, ny)) {
                                queue.push((nx, ny));
                            }
                            local_perimeter -= 1;
                        }
                    }
                }
                perimeter += local_perimeter;
            }
            total_sum += group_size * perimeter;
        }
    }
    println!("Result: {}", total_sum);
}
