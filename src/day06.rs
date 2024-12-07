use std::collections::HashSet;
use std::fs;

pub fn solution() {
    let input = fs::read_to_string("./inputs/day06.txt").expect("Failed to read input file");
    let mut map = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    // A.
    let mut p = (0, 0);
    let mut d = (0, -1);
    let dirs = [(0, -1), (1, 0), (0, 1), (-1, 0)];

    for (y, row) in map.clone().iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == '^' {
                p = (x, y);
                map[y][x] = '.';
                break;
            }
        }
    }

    let mut visited = HashSet::<(usize, usize)>::new();
    visited.insert(p);

    loop {
        let (x, y) = p;
        let (dx, dy) = d;
        let next_position = ((x as isize + dx) as usize, (y as isize + dy) as usize);

        if next_position.1 >= map.len() || next_position.0 >= map[0].len() {
            break;
        }

        match map[next_position.1][next_position.0] {
            '#' => {
                d = dirs[(dirs.iter().position(|&_d| _d == d).unwrap() + 1) % dirs.len()];
            }
            '.' => {
                p = next_position;
                visited.insert(p);
            }
            _ => {
                break;
            }
        }
    }

    println!("Result: {}", visited.len());
}
