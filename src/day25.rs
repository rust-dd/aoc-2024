use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
struct Block {
    origin: String,
    heights: Vec<i32>,
}

pub fn solution() {
    let file = File::open("./inputs/day25.txt").expect("Could not open input file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|line| line.expect("Could not read line")).collect();

    let blocks = parse_blocks(&lines);
    let processed_blocks: Vec<Block> = blocks
        .iter()
        .map(|block| {
            let origin = determine_block_origin(block);
            let heights = calculate_heights(block, &origin);
            Block { origin, heights }
        })
        .collect();

    let match_count = count_matches(&processed_blocks);
    println!("Result A: {}", match_count);
}

fn parse_blocks(lines: &[String]) -> Vec<Vec<String>> {
    let mut blocks = vec![];
    let mut current = vec![];

    for line in lines {
        if line.trim().is_empty() {
            if !current.is_empty() {
                blocks.push(current.clone());
                current.clear();
            }
        } else {
            current.push(line.clone());
            if current.len() == 7 {
                blocks.push(current.clone());
                current.clear();
            }
        }
    }

    if !current.is_empty() {
        blocks.push(current);
    }

    blocks
}

fn determine_block_origin(block: &[String]) -> String {
    let first_line_hashes = block[0].chars().filter(|&c| c == '#').count();
    let last_line_hashes = block[6].chars().filter(|&c| c == '#').count();

    if first_line_hashes > last_line_hashes {
        "Top".to_string()
    } else {
        "Bottom".to_string()
    }
}

fn calculate_heights(block: &[String], origin: &str) -> Vec<i32> {
    (0..block.iter().map(|line| line.len()).max().unwrap())
        .map(|col| {
            let mut height = 0;
            let rows = if origin == "Top" {
                (1..7).collect::<Vec<_>>()
            } else {
                (0..6).rev().collect::<Vec<_>>()
            };

            for row in rows {
                if block[row].chars().nth(col).unwrap_or('.') == '#' {
                    height += 1;
                } else {
                    break;
                }
            }

            height
        })
        .collect()
}

fn count_matches(blocks: &[Block]) -> i32 {
    let mut match_count = 0;

    for (i, block_a) in blocks.iter().enumerate() {
        for block_b in blocks.iter().skip(i + 1) {
            if block_a.origin != block_b.origin
                && block_a.heights.iter().zip(&block_b.heights).all(|(a, b)| a + b < 6)
            {
                match_count += 1;
            }
        }
    }

    match_count
}
