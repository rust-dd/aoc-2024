use ndarray::Array2;
use std::fs::File;
use std::io::BufRead;

pub fn solution() {
    let aoc_2024_a =
        File::open("./inputs/day04.txt").expect("Something went wrong reading the file");
    let lines = std::io::BufReader::new(aoc_2024_a)
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    // A.
    let rows = lines.len();
    let cols = lines[0].len();
    let data = lines.into_iter().flatten().collect::<Vec<char>>();
    let grid = Array2::from_shape_vec((rows, cols), data).unwrap();

    let target = "XMAS".chars().collect::<Vec<char>>();
    let rows = grid.nrows() as i32;
    let cols = grid.ncols() as i32;

    let dxdy: [(i32, i32); 8] = [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];

    let mut count = 0;
    for i in 0..rows {
        for j in 0..cols {
            for &(dx, dy) in &dxdy {
                let mut found = true;
                for k in 0..target.len() {
                    let x = i + dx * k as i32;
                    let y = j + dy * k as i32;
                    if x < 0
                        || y < 0
                        || x >= rows
                        || y >= cols
                        || grid[[x as usize, y as usize]] != target[k]
                    {
                        found = false;
                        break;
                    }
                }
                if found {
                    count += 1;
                }
            }
        }
    }

    println!("The number of XMAS is: {}", count);

    // B.
    let mas = ['M', 'A', 'S'];
    let sam = ['S', 'A', 'M'];
    let targets = [(&mas, &mas), (&mas, &sam), (&sam, &mas), (&sam, &sam)];
    let mut count = 0;

    for i in 0..rows {
        for j in 0..cols {
            if grid[[i as usize, j as usize]] == 'A' {
                let diag1 = [(i - 1, j - 1), (i, j), (i + 1, j + 1)];
                let diag2 = [(i - 1, j + 1), (i, j), (i + 1, j - 1)];

                let in_diag1 = diag1
                    .iter()
                    .all(|&(x, y)| x >= 0 && y >= 0 && x < rows && y < cols);
                let in_diag2 = diag2
                    .iter()
                    .all(|&(x, y)| x >= 0 && y >= 0 && x < rows && y < cols);

                if in_diag1 && in_diag2 {
                    let x_in_diag1 = diag1
                        .iter()
                        .map(|&(x, y)| grid[[x as usize, y as usize]])
                        .collect::<Vec<char>>();
                    let x_in_diag2 = diag2
                        .iter()
                        .map(|&(x, y)| grid[[x as usize, y as usize]])
                        .collect::<Vec<char>>();

                    for &(diag1_word, diag2_word) in &targets {
                        if x_in_diag1 == diag1_word && x_in_diag2 == diag2_word {
                            count += 1;
                        }
                    }
                }
            }
        }
    }

    println!("The number of MAS is: {}", count);
}
