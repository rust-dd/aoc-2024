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

    for (y, row) in map.iter_mut().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == '^' {
                p = (x, y);
                row[x] = '.';
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

    println!("Result part A: {}", visited.len());

    // B.
    let input = fs::read_to_string("./inputs/day06.txt").expect("Failed to read input file");
    let mut map = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let mut looping_positions = HashSet::<(usize, usize)>::new();
    let mut p = (0, 0);
    let d = (0, -1);

    for (y, row) in map.iter_mut().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == '^' {
                p = (x, y);
                row[x] = '.';
                break;
            }
        }
    }

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == '.' && (x, y) != p {
                map[y][x] = '#';

                if simulate_guard(&mut map, p, d, &dirs).is_some() {
                    looping_positions.insert((x, y));
                }

                map[y][x] = '.';
            }
        }
    }

    println!("Part B Result: {}", looping_positions.len());
}

fn simulate_guard(
    map: &Vec<Vec<char>>,
    start_pos: (usize, usize),
    start_dir: (isize, isize),
    dirs: &[(isize, isize)],
) -> Option<HashSet<(usize, usize)>> {
    let mut visited = HashSet::<(usize, usize)>::new();
    let mut states = HashSet::<((usize, usize), (isize, isize))>::new();
    let mut p = start_pos;
    let mut d = start_dir;

    loop {
        let (x, y) = p;
        let (dx, dy) = d;
        let next = ((x as isize + dx) as usize, (y as isize + dy) as usize);

        if next.1 >= map.len() || next.0 >= map[0].len() {
            return None;
        }

        match map[next.1][next.0] {
            '#' => {
                d = dirs[(dirs.iter().position(|&_d| _d == d).unwrap() + 1) % dirs.len()];
            }
            '.' => {
                p = next;
                if !states.insert((p, d)) {
                    return Some(visited);
                }
                visited.insert(p);
            }
            _ => {
                return None;
            }
        }
    }
}
