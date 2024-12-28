use std::collections::VecDeque;
use std::fs;
use std::io::{self, BufRead};

const GRID_SIZE: usize = 71;
const NUM_ROWS: usize = 1024;

type Point = (usize, usize);

fn bfs(grid: &[[char; GRID_SIZE]; GRID_SIZE]) -> (bool, usize) {
    let directions: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut queue = VecDeque::new();
    let mut visited = [[false; GRID_SIZE]; GRID_SIZE];
    let mut path_length = 0;

    queue.push_back((0, 0, vec![(0,0)]));
    visited[0][0] = true;

     while let Some((x, y, path)) = queue.pop_front() {
        if (x, y) == (GRID_SIZE - 1, GRID_SIZE - 1) {
             path_length = path.len() -1;
           return (true, path_length);
        }

        for &(dx, dy) in &directions {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0 && nx < GRID_SIZE as isize && ny >= 0 && ny < GRID_SIZE as isize {
                let (nx, ny) = (nx as usize, ny as usize);
                if !visited[nx][ny] && grid[nx][ny] == '.' {
                    let mut new_path = path.clone();
                    new_path.push((nx,ny));
                    queue.push_back((nx, ny, new_path));
                    visited[nx][ny] = true;
                }
            }
        }
    }
   (false, path_length)
}

pub fn solution() {
    let file = fs::File::open("./inputs/day18.txt").expect("Failed to open file");
    let reader = io::BufReader::new(file);
    let mut lines_iter = reader.lines().map(Result::unwrap);
    let mut grid = [['.'; GRID_SIZE]; GRID_SIZE];

    // Initial grid setup
    for line_content in lines_iter.by_ref().take(NUM_ROWS) {
        let (y_str, x_str) = line_content.trim().split_once(',').unwrap();
        let (x, y): (usize, usize) = (x_str.parse().unwrap(), y_str.parse().unwrap());
        if x < GRID_SIZE && y < GRID_SIZE {
            grid[x][y] = '#';
        }
    }

    // Part A
    let (path_found, path_length) = bfs(&grid);
    if path_found {
        println!("Result A: {}", path_length);
    }


    // Part B
    for line_content in lines_iter {
        let (y_str, x_str) = line_content.trim().split_once(',').unwrap();
        let (x, y): (usize, usize) = (x_str.parse().unwrap(), y_str.parse().unwrap());
        
        if x < GRID_SIZE && y < GRID_SIZE {
            grid[x][y] = '#';
        }

        let (path_found, _) = bfs(&grid);
        if !path_found {
            println!("Result B: {},{}", y, x);
            break;
        }
    }
}