use ndarray::Array2;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;

pub fn solution() {
    let aoc_2024_08 =
        File::open("./inputs/day08.txt").expect("Something went wrong reading the file");
    let lines = std::io::BufReader::new(aoc_2024_08)
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let rows = lines.len();
    let cols = lines[0].len();
    let data = lines.into_iter().flatten().collect::<Vec<char>>();

    let grid = Array2::from_shape_vec((rows, cols), data).unwrap();
    let mut positions_by_key: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let rows = grid.nrows() as i32;
    let cols = grid.ncols() as i32;

    let mut sum = 0;
    for i in 0..rows {
        for j in 0..cols {
            let key = grid[[i as usize, j as usize]];
            if grid[[i as usize, j as usize]] != '.' {
                positions_by_key
                    .entry(key)
                    .and_modify(|vec| vec.push((i, j)))
                    .or_insert_with(|| vec![(i, j)]);
            }
        }
    }

    // A.
    let mut grid_a = grid.clone();
    for entry in &positions_by_key {
        let values = entry.1;

        for i in 0..values.len() {
            let first = values.get(i).unwrap();
            for j in 0..values.len() {
                if i == j {
                    continue;
                }
                let second = values.get(j).unwrap();

                let x_diff = second.0 - first.0;
                let y_diff = second.1 - first.1;
                let new_x = second.0 + x_diff;
                let new_y = second.1 + y_diff;

                if new_x < rows && new_y < cols && new_x >= 0 && new_y >= 0 {
                    let cell = grid_a[(new_x as usize, new_y as usize)];
                    if cell != '#' {
                        sum += 1;
                        grid_a[(new_x as usize, new_y as usize)] = '#';
                    }
                }
            }
        }
    }
    println!("Result: {}", sum);

    // B
    let mut grid_b = grid.clone();
    for entry in &positions_by_key {
        let values = entry.1;

        for i in 0..values.len() {
            let first = values.get(i).unwrap();
            for j in 0..values.len() {
                if i == j {
                    continue;
                }
                let second = values.get(j).unwrap();

                let x_diff = second.0 - first.0;
                let y_diff = second.1 - first.1;
                let mut new_x = second.0 + x_diff;
                let mut new_y = second.1 + y_diff;

                while (0..rows).contains(&new_x) && (0..cols).contains(&new_y) {
                    let cell = grid_b[(new_x as usize, new_y as usize)];
                    if cell != '#' {
                        grid_b[(new_x as usize, new_y as usize)] = '#';
                    }
                    new_x = new_x + x_diff;
                    new_y = new_y + y_diff;
                }
            }
        }
    }

    let mut sum = 0;
    for i in 0..rows {
        for j in 0..cols {
            if grid_b[[i as usize, j as usize]] != '.' {
                sum += 1;
            }
        }
    }
    println!("Result: {}", sum)
}
