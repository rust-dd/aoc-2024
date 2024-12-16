use std::fs;
use std::path::Path;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Tile {
    Wall,
    Empty,
    Box,
    Robot,
}

pub fn solution() {
    let input = fs::read_to_string(Path::new("./inputs/day15.txt")).unwrap();
    let (map_str, moves_str) = input.split_once("\n\n").unwrap();

    let mut map = map_str
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => Tile::Wall,
                    '.' => Tile::Empty,
                    'O' => Tile::Box,
                    '@' => Tile::Robot,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect::<Vec<Vec<Tile>>>();

    let moves = moves_str
        .lines()
        .flat_map(|line| line.chars().filter(|c| !c.is_whitespace()))
        .collect::<Vec<char>>();

    let mut warehouse = Warehouse::new(&mut map);
    warehouse.simulate_moves(&moves);

    let gps_sum = warehouse.calculate_gps_sum();
    println!("Sum of GPS coordinates: {}", gps_sum);
}

struct Warehouse {
    grid: Vec<Vec<Tile>>,
    robot_pos: (usize, usize),
    rows: usize,
    cols: usize,
}

impl Warehouse {
    fn new(grid: &mut [Vec<Tile>]) -> Self {
        let rows = grid.len();
        let cols = if rows == 0 { 0 } else { grid[0].len() };
        let mut robot_pos = (0, 0);

        'outer: for (row_idx, row) in grid.iter().enumerate() {
            for (col_idx, &tile) in row.iter().enumerate() {
                if tile == Tile::Robot {
                    robot_pos = (row_idx, col_idx);
                    break 'outer;
                }
            }
        }

        Warehouse {
            grid: grid.to_vec(),
            robot_pos,
            rows,
            cols,
        }
    }

    fn simulate_moves(&mut self, moves: &[char]) {
        for &mv in moves {
            self.make_move(mv);
        }
    }

    fn make_move(&mut self, mv: char) {
        let (dr, dc) = match mv {
            '^' => (-1, 0),
            'v' => (1, 0),
            '<' => (0, -1),
            '>' => (0, 1),
            _ => unreachable!(),
        };

        let (r0, c0) = self.robot_pos;
        let new_r = r0 as isize + dr;
        let new_c = c0 as isize + dc;

        if !self.is_in_bounds((new_r, new_c)) {
            return;
        }
        let (new_r, new_c) = (new_r as usize, new_c as usize);

        match self.get_tile((new_r, new_c)) {
            Some(Tile::Empty) => {
                self.update_robot_position((new_r, new_c));
            }
            Some(Tile::Box) => {
                if self.try_push((new_r, new_c), (dr, dc)) {
                    self.update_robot_position((new_r, new_c));
                }
            }

            _ => {}
        }
    }

    fn try_push(&mut self, pos: (usize, usize), dir: (isize, isize)) -> bool {
        let (r, c) = pos;
        let (dr, dc) = dir;

        let next_r = r as isize + dr;
        let next_c = c as isize + dc;

        if !self.is_in_bounds((next_r, next_c)) {
            return false;
        }
        let (next_r, next_c) = (next_r as usize, next_c as usize);

        match self.get_tile((next_r, next_c)) {
            Some(Tile::Empty) => {
                self.set_tile((next_r, next_c), Tile::Box);
                self.set_tile(pos, Tile::Empty);
                true
            }
            Some(Tile::Box) => {
                if self.try_push((next_r, next_c), (dr, dc)) {
                    self.set_tile((next_r, next_c), Tile::Box);
                    self.set_tile(pos, Tile::Empty);
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    fn is_in_bounds(&self, pos: (isize, isize)) -> bool {
        let (r, c) = pos;
        if r < 0 || c < 0 {
            return false;
        }

        if (r as usize) >= self.rows || (c as usize) >= self.cols {
            return false;
        }

        true
    }

    fn update_robot_position(&mut self, new_pos: (usize, usize)) {
        self.set_tile(self.robot_pos, Tile::Empty);

        self.set_tile(new_pos, Tile::Robot);
        self.robot_pos = new_pos;
    }

    fn get_tile(&self, pos: (usize, usize)) -> Option<Tile> {
        let (r, c) = pos;
        if r < self.rows && c < self.cols {
            Some(self.grid[r][c])
        } else {
            None
        }
    }

    fn set_tile(&mut self, pos: (usize, usize), tile: Tile) {
        let (r, c) = pos;
        self.grid[r][c] = tile;
    }

    fn calculate_gps_sum(&self) -> usize {
        self.grid.iter().enumerate().fold(0, |sum, (row_idx, row)| {
            sum + row.iter().enumerate().fold(0, |row_sum, (col_idx, &tile)| {
                if tile == Tile::Box {
                    row_sum + (100 * row_idx) + col_idx
                } else {
                    row_sum
                }
            })
        })
    }
}
