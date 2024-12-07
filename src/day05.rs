use std::fs;

use petgraph::{
    graph::{DiGraph, NodeIndex},
    visit::IntoNodeReferences,
    Direction,
};

pub fn solution() {
    let aoc_2024_04 =
        fs::read_to_string("./inputs/day05.txt").expect("Something went wrong reading the file");

    let mut parts = aoc_2024_04.split("\n\n");
    let page_ordering_rules_text = parts.next().unwrap();
    let page_ordering_rules: Vec<(i8, i8)> = page_ordering_rules_text
        .lines()
        .map(|line| {
            let mut numbers = line.split('|').map(|n| n.parse::<i8>().unwrap());
            (numbers.next().unwrap(), numbers.next().unwrap())
        })
        .collect();

    let pages_to_reproduce_text = parts.next().unwrap();
    let pages_to_reproduce: Vec<Vec<i8>> = pages_to_reproduce_text
        .lines()
        .map(|line| line.split(',').map(|n| n.parse::<i8>().unwrap()).collect())
        .collect();

    // A.
    let sum: i32 = pages_to_reproduce
        .iter()
        .filter(|page| {
            page_ordering_rules.iter().all(|&(left_rule, right_rule)| {
                let left_index = page.iter().position(|&r| r == left_rule);
                let right_index = page.iter().position(|&r| r == right_rule);

                match (left_index, right_index) {
                    (Some(left), Some(right)) => left <= right,
                    _ => true,
                }
            })
        })
        .map(|page| *(page.get(page.len() / 2).unwrap()) as i32)
        .sum();
    println!("Result: {}", sum);

    // B.
    let mut sum: i32 = 0;
    let mut failed_rows = Vec::new();
    for page in &pages_to_reproduce {
        for rule in &page_ordering_rules {
            let left_index = page.iter().position(|&r| r == rule.0);
            let right_index = page.iter().position(|&r| r == rule.1);

            if left_index.is_some() && right_index.is_some() {
                if left_index.unwrap() > right_index.unwrap() {
                    failed_rows.push(page);
                    break;
                }
            }
        }
    }
    for failed_row in failed_rows {
        let mut graph = DiGraph::<&i8, ()>::new();
        for number in failed_row {
            graph.add_node(number);
        }
        for first in failed_row {
            for second in failed_row {
                let rule = page_ordering_rules
                    .iter()
                    .position(|&r| r.0 == *first && r.1 == *second);
                if rule.is_some() {
                    let first_node = find_node_by_value(&graph, first);
                    let second_node = find_node_by_value(&graph, second);
                    graph.add_edge(first_node.unwrap(), second_node.unwrap(), ());
                }
            }
        }
        let mut no_inbound = Vec::new();
        for node in graph.node_indices() {
            if graph
                .neighbors_directed(node, Direction::Incoming)
                .next()
                .is_none()
            {
                no_inbound.push(node);
            }
        }

        let mut nodes: Vec<(NodeIndex, usize, usize)> = graph
            .node_indices()
            .map(|node| {
                let incoming_count = graph
                    .neighbors_directed(node, Direction::Incoming)
                    .count();
                let outgoing_count = graph
                    .neighbors_directed(node, Direction::Outgoing)
                    .count();
                (node, incoming_count, outgoing_count)
            })
            .collect();

        nodes.sort_by_key(|&(_, incoming_count, outgoing_count)| (incoming_count, outgoing_count));


        let middle_index = nodes.len() / 2;
        let middle_node = nodes[middle_index].0; 
        let middle_value = graph[middle_node];
        sum = sum + *middle_value as i32;
    }
    println!("Result: {}", sum);
}

fn find_node_by_value(graph: &DiGraph<&i8, ()>, value: &i8) -> Option<NodeIndex> {
    for (node_index, node_value) in graph.node_references() {
        if *node_value == value {
            return Some(node_index);
        }
    }
    None
}
