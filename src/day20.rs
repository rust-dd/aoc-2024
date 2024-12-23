use std::{
    collections::{HashMap, VecDeque},
    fs,
    io::{self, BufRead},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Pos {
    r: usize,
    c: usize,
}

pub fn solution() {
    let file = fs::File::open("./inputs/day20.txt").expect("Failed to open file");
    let reader = io::BufReader::new(file);

    let map: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Could not parse line"))
        .collect();

    let mut start = Pos { r: 0, c: 0 };
    let mut end = Pos { r: 0, c: 0 };
    let rows = map.len();
    let cols = map[0].len();

    for r in 0..rows {
        for c in 0..cols {
            let ch = map[r].chars().nth(c).unwrap();
            if ch == 'S' {
                start = Pos { r, c };
            } else if ch == 'E' {
                end = Pos { r, c };
            }
        }
    }

    let dist_from_start = bfs_no_cheat_from(&map, start);
    let dist_from_end = bfs_no_cheat_from(&map, end);
    let dist_no_cheat = *dist_from_start.get(&end).unwrap();

    let count_2 = count_cheats_saving_100(
        &map,
        &dist_from_start,
        &dist_from_end,
        dist_no_cheat,
        2
    );
    println!("Result A: {}", count_2);

    let count_20 = count_cheats_saving_100(
        &map,
        &dist_from_start,
        &dist_from_end,
        dist_no_cheat,
        20
    );
    println!("Result B: {}", count_20);
}

fn bfs_no_cheat_from(map: &[String], start: Pos) -> HashMap<Pos, usize> {
    let rows = map.len();
    let cols = map[0].len();

    let mut dist_map = HashMap::<Pos, usize>::new();
    let mut visited = vec![vec![false; cols]; rows];

    visited[start.r][start.c] = true;
    dist_map.insert(start, 0);

    let mut queue = VecDeque::new();
    queue.push_back(start);

    let directions = [(1,0), (-1,0), (0,1), (0,-1)];

    while let Some(cur) = queue.pop_front() {
        let current_dist = dist_map[&cur];
        for (dr, dc) in directions {
            let nr = cur.r as isize + dr;
            let nc = cur.c as isize + dc;
            if nr < 0 || nc < 0 {
                continue;
            }
            let nr = nr as usize;
            let nc = nc as usize;
            if nr < rows && nc < cols && !visited[nr][nc] {
                let ch = map[nr].chars().nth(nc).unwrap();
                if ch != '#' {
                    visited[nr][nc] = true;
                    let nxt = Pos { r: nr, c: nc };
                    dist_map.insert(nxt, current_dist + 1);
                    queue.push_back(nxt);
                }
            }
        }
    }

    dist_map
}

fn count_cheats_saving_100(
    map: &[String],
    dist_s: &HashMap<Pos, usize>,
    dist_e: &HashMap<Pos, usize>,
    dist_no_cheat: usize,
    max_steps: usize,
) -> usize {
    let mut cheat_best_dist = HashMap::<(Pos, Pos), usize>::new();

    let is_track = |r: usize, c: usize| {
        let ch = map[r].chars().nth(c).unwrap();
        ch != '#'
    };

    for (&x, &dist_sx) in dist_s.iter() {
        if !is_track(x.r, x.c) {
            continue;
        }

        let mut visited_steps = vec![vec![None; map[0].len()]; map.len()];
        visited_steps[x.r][x.c] = Some(0);
        let mut queue = VecDeque::new();
        queue.push_back(x);

        while let Some(cur) = queue.pop_front() {
            let steps_so_far = visited_steps[cur.r][cur.c].unwrap();
            if steps_so_far >= max_steps {
                continue;
            }

            for (dr, dc) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let nr = cur.r as isize + dr;
                let nc = cur.c as isize + dc;
                if nr < 0 || nc < 0 {
                    continue;
                }
                let nr = nr as usize;
                let nc = nc as usize;
                if nr >= map.len() || nc >= map[0].len() {
                    continue;
                }
                // ignoring walls => always can go
                if visited_steps[nr][nc].is_none() {
                    visited_steps[nr][nc] = Some(steps_so_far + 1);
                    queue.push_back(Pos { r: nr, c: nc });
                }
            }
        }

        // Evaluate each reachable y for 1 <= steps <= max_steps
        for r in 0..map.len() {
            for c in 0..map[0].len() {
                if let Some(steps) = visited_steps[r][c] {
                    if steps >= 1 && steps <= max_steps && is_track(r, c) {
                        let y = Pos { r, c };
                        if let Some(&dist_ey) = dist_e.get(&y) {
                            let candidate_dist = dist_sx + steps + dist_ey;
                            let entry = cheat_best_dist.entry((x, y)).or_insert(candidate_dist);
                            if candidate_dist < *entry {
                                *entry = candidate_dist;
                            }
                        }
                    }
                }
            }
        }
    }

    let mut count_ge_100 = 0;
    for &cheat_dist in cheat_best_dist.values() {
        let saving = dist_no_cheat.saturating_sub(cheat_dist);
        if saving >= 100 {
            count_ge_100 += 1;
        }
    }
    count_ge_100
}
