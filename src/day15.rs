use std::collections::HashSet;

const DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

struct Warehouse;

impl Warehouse {
    fn get_direction(&self, mv: char) -> Option<(isize, isize)> {
        match mv {
            '^' => Some(DIRS[0]),
            'v' => Some(DIRS[1]),
            '<' => Some(DIRS[2]),
            '>' => Some(DIRS[3]),
            _ => None,
        }
    }

    fn get_robot_pos(&self, grid: &Vec<Vec<char>>) -> Option<(usize, usize)> {
        for (i, row) in grid.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                if cell == '@' {
                    return Some((i, j));
                }
            }
        }
        None
    }

    fn moving(
        &self,
        mut grid: Vec<Vec<char>>,
        mut pos: (usize, usize),
        moves: Vec<char>,
        part: usize,
    ) -> Vec<Vec<char>> {
        for mv in moves {
            let (dy, dx) = match self.get_direction(mv) {
                Some(dir) => dir,
                None => {
                    continue;
                }
            };

            let ny = pos.0 as isize + dy;
            let nx = pos.1 as isize + dx;

            if ny < 0 || nx < 0 || ny >= grid.len() as isize || nx >= grid[0].len() as isize {
                continue;
            }

            let ny_usize = ny as usize;
            let nx_usize = nx as usize;

            match grid[ny_usize][nx_usize] {
                '.' => pos = (ny_usize, nx_usize),
                '#' => continue,
                _ => {
                    let (edges, adjs) = self.get_adjs_and_edges(&grid, pos, mv, part);
                    let mut blocked = 0;
                    for (ey, ex) in edges {
                        let (ey_new, ex_new) = (ey as isize + dy, ex as isize + dx);
                        if ey_new < 0
                            || ex_new < 0
                            || ey_new >= grid.len() as isize
                            || ex_new >= grid[0].len() as isize
                            || grid[ey_new as usize][ex_new as usize] == '#'
                        {
                            blocked += 1;
                        }
                    }

                    if blocked == 0 {
                        grid = self.update_grid(grid, adjs, mv);
                        pos = (ny_usize, nx_usize);
                    }
                }
            }
        }
        grid
    }

    fn get_adjs_and_edges(
        &self,
        grid: &Vec<Vec<char>>,
        pos: (usize, usize),
        move_dir: char,
        part: usize,
    ) -> (Vec<(usize, usize)>, HashSet<(usize, usize)>) {
        let (mut y, mut x) = pos;
        let (dy, dx) = self.get_direction(move_dir).unwrap();
        let mut adjs = HashSet::new();

        if part == 1 || "<>".contains(move_dir) {
            loop {
                let ny = y as isize + dy;
                let nx = x as isize + dx;

                if ny < 0 || nx < 0 || ny >= grid.len() as isize || nx >= grid[0].len() as isize {
                    return (vec![(y, x)], adjs);
                }

                let ny_usize = ny as usize;
                let nx_usize = nx as usize;

                if grid[ny_usize][nx_usize] == '.' || grid[ny_usize][nx_usize] == '#' {
                    return (vec![(y, x)], adjs);
                }

                y = ny_usize;
                x = nx_usize;
                adjs.insert((y, x));
            }
        } else {
            let mut edges = Vec::new();
            let mut queue = vec![(y, x)];
            while let Some((cy, cx)) = queue.pop() {
                if adjs.contains(&(cy, cx)) {
                    continue;
                }
                adjs.insert((cy, cx));

                let ny = cy as isize + dy;
                let nx = cx as isize + dx;

                if ny < 0 || nx < 0 || ny >= grid.len() as isize || nx >= grid[0].len() as isize {
                    continue;
                }

                let ny_usize = ny as usize;
                let nx_usize = nx as usize;

                match grid[ny_usize][nx_usize] {
                    '.' | '#' => edges.push((cy, cx)),
                    '[' => {
                        if nx + 1 < grid[0].len() as isize {
                            queue.push((ny_usize, nx_usize));
                            queue.push((ny_usize, nx_usize + 1));
                        }
                    }
                    ']' => {
                        if nx >= 1 {
                            queue.push((ny_usize, nx_usize));
                            queue.push((ny_usize, nx_usize - 1));
                        }
                    }
                    _ => (),
                }
            }
            (edges, adjs)
        }
    }

    fn update_grid(
        &self,
        mut grid: Vec<Vec<char>>,
        adjs: HashSet<(usize, usize)>,
        move_dir: char,
    ) -> Vec<Vec<char>> {
        let mut sorted_coords = adjs.into_iter().collect::<Vec<(usize, usize)>>();
        match move_dir {
            '^' => sorted_coords.sort_by_key(|&(y, _)| y),
            'v' => sorted_coords.sort_by_key(|&(y, _)| std::cmp::Reverse(y)),
            '<' => sorted_coords.sort_by_key(|&(_, x)| x),
            '>' => sorted_coords.sort_by_key(|&(_, x)| std::cmp::Reverse(x)),
            _ => (),
        }

        let (dy, dx) = self.get_direction(move_dir).unwrap();
        for (y, x) in sorted_coords {
            let ny = y as isize + dy;
            let nx = x as isize + dx;

            if ny < 0 || nx < 0 || ny >= grid.len() as isize || nx >= grid[0].len() as isize {
                continue;
            }

            let ny_usize = ny as usize;
            let nx_usize = nx as usize;

            grid[ny_usize][nx_usize] = grid[y][x];
            grid[y][x] = '.';
        }

        grid
    }

    fn get_coords_sum(&self, grid: &Vec<Vec<char>>, part: usize) -> usize {
        let box_char = if part == 2 { '[' } else { 'O' };
        grid.iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(|(x, &cell)| {
                        if cell == box_char {
                            Some(100 * y + x)
                        } else {
                            None
                        }
                    })
                    .sum::<usize>()
            })
            .sum()
    }

    fn resize_grid(&self, grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
        let mappings = vec![('#', "##"), ('O', "[]"), ('.', ".."), ('@', "@.")]
            .into_iter()
            .collect::<std::collections::HashMap<_, _>>();

        grid.iter()
            .map(|row| {
                row.iter()
                    .flat_map(|&c| mappings.get(&c).unwrap_or(&"").chars())
                    .collect()
            })
            .collect()
    }

    pub fn solve(&self, data: &str, part: usize) -> usize {
        let parts = data.split("\n\n").collect::<Vec<&str>>();
        if parts.len() != 2 {
            return 0;
        }

        let mut grid = parts[0].lines().map(|l| l.chars().collect()).collect();
        let valid_moves = vec!['^', 'v', '<', '>'];
        let moves = parts[1]
            .chars()
            .filter(|c| valid_moves.contains(c))
            .collect();

        if part == 2 {
            grid = self.resize_grid(&grid);
        }

        let pos = match self.get_robot_pos(&grid) {
            Some(p) => p,
            None => return 0,
        };
        grid[pos.0][pos.1] = '.';

        let grid = self.moving(grid, pos, moves, part);
        self.get_coords_sum(&grid, part)
    }
}

pub fn solution() {
    let data = std::fs::read_to_string("inputs/day15.txt").unwrap();

    let a = Warehouse.solve(&data, 1);
    println!("Part A: {}", a);

    // Part 2 is inspired by:
    // https://github.com/nitekat1124/advent-of-code-2024/blob/main/solutions/day15.py
    let b = Warehouse.solve(&data, 2);
    println!("Part B: {}", b);
}
