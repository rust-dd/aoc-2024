use std::collections::VecDeque;
use std::fs;
use std::io::{self, BufRead};

const GRID_SIZE: usize = 71;
const NUM_ROWS: usize = 1024;

pub fn solution() {
    let file = fs::File::open("./inputs/day18.txt").expect("Failed to open file");
    let reader = io::BufReader::new(file);

    let mut grid = [['.'; GRID_SIZE]; GRID_SIZE];

    for line in reader.lines().take(NUM_ROWS) {
        let line_content = line.unwrap();
        let (y, x) = line_content.trim().split_once(',').unwrap();
        let (x, y): (usize, usize) = (x.parse().unwrap(), y.parse().unwrap());
        if x < GRID_SIZE && y < GRID_SIZE {
            grid[x][y] = '#';
        }
    }

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut queue = VecDeque::new();
    let mut visited = [[false; GRID_SIZE]; GRID_SIZE];

    queue.push_back((0, 0, vec![(0, 0)]));
    visited[0][0] = true;

    while let Some((x, y, path)) = queue.pop_front() {
        if (x, y) == (GRID_SIZE - 1, GRID_SIZE - 1) {
            println!("Result A: {}", path.len() - 1);
            break;
        }

        for &(dx, dy) in &directions {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0 && nx < GRID_SIZE as isize && ny >= 0 && ny < GRID_SIZE as isize {
                let (nx, ny) = (nx as usize, ny as usize);
                if !visited[nx][ny] && grid[nx][ny] == '.' {
                    let mut new_path = path.clone();
                    new_path.push((nx, ny));
                    queue.push_back((nx, ny, new_path));
                    visited[nx][ny] = true;
                }
            }
        }
    }
}
