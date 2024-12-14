use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

use petgraph::adj::NodeIndex;
use petgraph::algo;
use petgraph::prelude::UnGraphMap;
use petgraph::visit::IntoNodeReferences;

pub fn solution() {
    let aoc_2024_12 = File::open("./inputs/day12.txt").expect("Failed to open file");
    let grid: Vec<Vec<char>> = io::BufReader::new(aoc_2024_12)
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.chars().collect())
        .collect();

    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    let mut visited = HashSet::new();
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    let mut total_sum = 0;

    for r in 0..rows {
        for c in 0..cols {
            if visited.contains(&(r, c)) {
                continue;
            }

            let mut queue = vec![(r, c)];
            visited.insert((r, c));

            let mut group_size = 0;
            let mut perimeter = 0;

            while let Some((x, y)) = queue.pop() {
                let value = grid[x as usize][y as usize];
                let mut local_perimeter = 4;
                group_size += 1;

                for &(dx, dy) in &directions {
                    let nx = x + dx;
                    let ny = y + dy;

                    if nx >= 0 && nx < rows && ny >= 0 && ny < cols {
                        if grid[nx as usize][ny as usize] == value {
                            if visited.insert((nx, ny)) {
                                queue.push((nx, ny));
                            }
                            local_perimeter -= 1;
                        }
                    }
                }
                perimeter += local_perimeter;
            }
            total_sum += group_size * perimeter;
        }
    }
    println!("Result: {}", total_sum);

    // B.
    let mut graph = UnGraphMap::<(i32, i32), ()>::new();

    for y in 0..rows {
        for x in 0..cols {
            // Add node to the graph
            let node = graph.add_node((x, y));
            let c = grid[y as usize][x as usize];

            // Add edges to the graph
            for &(dx, dy) in &directions {
                let nx = x + dx;
                let ny = y + dy;

                // Check if the node is within the grid
                if nx >= 0 && nx < cols && ny >= 0 && ny < rows {
                    if grid[ny as usize][nx as usize] == c {
                        graph.add_edge(node, (nx, ny), ());
                    }
                }
            }
        }
    }

    let graph = algo::condensation(graph.into_graph::<NodeIndex>(), false);
    let sum = graph.node_references().map(|(_, nodes)| {
        let id = grid[nodes[0].1 as usize][nodes[0].0 as usize];
        let node_size = nodes.len();
        let perimeter = nodes
            .iter()
            .map(|n| {
                let mut count = 0;

                for i in 0..4 {
                    let (dx, dy) = directions[i];
                    let (dx1, dy1) = directions[(i + 1) % 4];

                    let first_neighbor_same_region = {
                        let nx = n.0 + dx;
                        let ny = n.1 + dy;
                        nx >= 0
                            && ny >= 0
                            && (ny as usize) < grid.len()
                            && (nx as usize) < grid[ny as usize].len()
                            && grid[ny as usize][nx as usize] == id
                    };

                    let second_neighbor_same_region = {
                        let nx = n.0 + dx1;
                        let ny = n.1 + dy1;
                        nx >= 0
                            && ny >= 0
                            && (ny as usize) < grid.len()
                            && (nx as usize) < grid[ny as usize].len()
                            && grid[ny as usize][nx as usize] == id
                    };

                    let diagonal_test = {
                        let nx = n.0 + dx + dx1;
                        let ny = n.1 + dy + dy1;
                        nx >= 0
                            && ny >= 0
                            && (ny as usize) < grid.len()
                            && (nx as usize) < grid[ny as usize].len()
                            && grid[ny as usize][nx as usize] != id
                    };

                    if first_neighbor_same_region && second_neighbor_same_region && diagonal_test {
                        count += 1;
                    } else if !first_neighbor_same_region && !second_neighbor_same_region {
                        count += 1;
                    }
                }

                count
            })
            .sum::<usize>();

        node_size * perimeter
    });

    println!("Result: {}", sum.sum::<usize>());
}
