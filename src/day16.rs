use pathfinding::prelude::dijkstra_all;
use std::{collections::HashMap, fs, path::Path};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct State {
    row: usize,
    col: usize,
    dir: usize,
}

pub fn solution() {
    let input = fs::read_to_string(Path::new("./inputs/day16.txt")).unwrap();
    let maze = input.lines().collect::<Vec<&str>>();
    let rows = maze.len();
    let cols = if rows > 0 { maze[0].len() } else { 0 };
    let maze_grid = maze
        .iter()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    // A.
    let (mut start_row, mut start_col) = (0, 0);
    let (mut end_row, mut end_col) = (0, 0);
    for r in 0..rows {
        for c in 0..cols {
            match maze_grid[r][c] {
                'S' => {
                    start_row = r;
                    start_col = c;
                }
                'E' => {
                    end_row = r;
                    end_col = c;
                }
                _ => {}
            }
        }
    }

    let forward_successors = |&State { row, col, dir }: &State| -> Vec<(State, u64)> {
        let mut next = Vec::with_capacity(3);

        next.push((
            State {
                row,
                col,
                dir: (dir + 3) % 4,
            },
            1000,
        ));

        next.push((
            State {
                row,
                col,
                dir: (dir + 1) % 4,
            },
            1000,
        ));

        let (dr, dc) = match dir {
            0 => (0, 1),
            1 => (1, 0),
            2 => (0, -1),
            3 => (-1, 0),
            _ => unreachable!(),
        };

        let nr = row as isize + dr;
        let nc = col as isize + dc;
        if nr >= 0 && nr < rows as isize && nc >= 0 && nc < cols as isize {
            if maze_grid[nr as usize][nc as usize] != '#' {
                next.push((
                    State {
                        row: nr as usize,
                        col: nc as usize,
                        dir,
                    },
                    1,
                ));
            }
        }
        next
    };

    let start_state = State {
        row: start_row,
        col: start_col,
        dir: 0,
    };
    let dist_s_map: HashMap<State, (State, u64)> = dijkstra_all(&start_state, forward_successors);

    let mut best_score = u64::MAX;
    for d in 0..4 {
        let e_state = State {
            row: end_row,
            col: end_col,
            dir: d,
        };
        if let Some((_, cost)) = dist_s_map.get(&e_state) {
            best_score = best_score.min(*cost);
        }
    }
    println!("Result: {}", best_score);

    // B.
    let reverse_successors = |&State { row, col, dir }: &State| -> Vec<(State, u64)> {
        let mut neighbors = Vec::with_capacity(3);

        neighbors.push((
            State {
                row,
                col,
                dir: (dir + 1) % 4,
            },
            1000,
        ));

        neighbors.push((
            State {
                row,
                col,
                dir: (dir + 3) % 4,
            },
            1000,
        ));

        let (dr, dc) = match dir {
            0 => (0, -1),
            1 => (-1, 0),
            2 => (0, 1),
            3 => (1, 0),
            _ => unreachable!(),
        };

        let rr = row as isize + dr;
        let cc = col as isize + dc;

        if rr >= 0 && rr < rows as isize && cc >= 0 && cc < cols as isize {
            if maze_grid[rr as usize][cc as usize] != '#' {
                neighbors.push((
                    State {
                        row: rr as usize,
                        col: cc as usize,
                        dir,
                    },
                    1,
                ));
            }
        }
        neighbors
    };

    let mut dist_e_map = HashMap::<State, (State, u64)>::new();
    for d in 0..4 {
        let e_state = State {
            row: end_row,
            col: end_col,
            dir: d,
        };
        let partial_map = dijkstra_all(&e_state, reverse_successors);

        for (st, (_, cost)) in partial_map {
            dist_e_map
                .entry(st)
                .and_modify(|existing| {
                    if cost < existing.1 {
                        existing.1 = cost;
                    }
                })
                .or_insert((e_state, cost));
        }
    }

    let mut on_best_path = vec![vec![false; cols]; rows];
    for (&st, &(_, cost_s)) in &dist_s_map {
        if let Some((_, cost_e)) = dist_e_map.get(&st) {
            if cost_s + cost_e == best_score {
                on_best_path[st.row][st.col] = true;
            }
        }
    }

    on_best_path[start_row][start_col] = true;
    on_best_path[end_row][end_col] = true;

    let count = on_best_path
        .iter()
        .flat_map(|row| row.iter())
        .filter(|&&b| b)
        .count();

    println!("Result: {}", count);
}
