use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

use petgraph::{
    graph::{Graph, NodeIndex},
    Undirected,
};

pub fn solution() {
    let file = File::open("./inputs/day23.txt").unwrap();
    let reader = BufReader::new(file);

    let mut graph = Graph::<String, (), Undirected>::new_undirected();
    let mut nodes: HashMap<String, NodeIndex> = HashMap::new();

    for line_result in reader.lines() {
        let line = line_result.unwrap();
        let parts: Vec<String> = line.split('-').map(|s| s.to_string()).collect();
        let computer1 = &parts[0];
        let computer2 = &parts[1];

        let node1 = *nodes
            .entry(computer1.to_string())
            .or_insert_with(|| graph.add_node(computer1.to_string()));
        let node2 = *nodes
            .entry(computer2.to_string())
            .or_insert_with(|| graph.add_node(computer2.to_string()));

        graph.add_edge(node1, node2, ());
    }

    // A
    let mut three_node_sets: HashSet<Vec<String>> = HashSet::new();
    for u in graph.node_indices() {
        let neighbors: Vec<_> = graph.neighbors(u).collect();
        for i in 0..neighbors.len() {
            for j in (i + 1)..neighbors.len() {
                let n1 = neighbors[i];
                let n2 = neighbors[j];

                if graph.contains_edge(n1, n2) {
                    let mut tri = vec![graph[u].clone(), graph[n1].clone(), graph[n2].clone()];
                    tri.sort();
                    three_node_sets.insert(tri);
                }
            }
        }
    }

    let set_has_node_start_with_t = three_node_sets
        .iter()
        .filter(|triangle| triangle.iter().any(|label| label.starts_with('t')))
        .count();
    println!("Result A: {}", set_has_node_start_with_t);

    // B
    let mut largest_set: HashSet<NodeIndex> = HashSet::new();
    for current_node in graph.node_indices() {
        let mut current_set = HashSet::new();
        current_set.insert(current_node);
        let mut neighbours: Vec<NodeIndex> = graph.neighbors(current_node).collect();

        while let Some(neighbour) = neighbours.pop() {
            let is_fully_connected = current_set
                .iter()
                .all(|&current| graph.contains_edge(neighbour, current) || current == neighbour);

            if is_fully_connected {
                current_set.insert(neighbour);
                for neighbour_of_neighbour in graph.neighbors(neighbour) {
                    if !current_set.contains(&neighbour_of_neighbour)
                        && !neighbours.contains(&neighbour_of_neighbour)
                    {
                        neighbours.push(neighbour_of_neighbour);
                    }
                }
            }
        }
        if current_set.len() > largest_set.len() {
            largest_set = current_set;
        }
    }

    let mut labels: Vec<String> = largest_set.into_iter().map(|i| graph[i].clone()).collect();
    labels.sort();

    println!("Result B: {}", labels.join(","));
}
