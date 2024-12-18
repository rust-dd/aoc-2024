use std::fs;
use regex::Regex;

pub fn solution() {
    let input = fs::read_to_string("./inputs/day17.txt").expect("Failed to read file");
    let re = Regex::new(r"Register A:\s*(\d+)\s*Register B:\s*(\d+)\s*Register C:\s*(\d+)\s*Program:\s*([\d,]+)").unwrap();
    let caps = re.captures(&input).expect("Failed to parse input");

    let a_init: u64 = caps[1].parse().unwrap();
    let b_init: u64 = caps[2].parse().unwrap();
    let c_init: u64 = caps[3].parse().unwrap();
    let programs: Vec<u64> = caps[4]
        .split(',')
        .map(|x| x.trim().parse().unwrap())
        .collect();

    // Part A
    let result_a = run_program(a_init, b_init, c_init, &programs);
    let result_a_joined = result_a.iter()
        .map(u64::to_string)
        .collect::<Vec<_>>()
        .join(",");
    println!("Part A Result: {}", result_a_joined);

     // Part B
    let result_b = find_part_b_initial_a(b_init, c_init, &programs).unwrap();
    println!("Part B Result: {}", result_b);
}

fn run_program(mut a: u64, mut b: u64, mut c: u64, programs: &[u64]) -> Vec<u64> {
    let mut result = Vec::new();
    let mut pointer = 0;

    while pointer < programs.len() {
        let opcode = programs[pointer];
        let operand = programs[pointer + 1];
        let combo = get_combo(operand, a, b, c);

        match opcode {
            0 => a /= 1 << combo,
            1 => b ^= operand,
            2 => b = combo % 8,
            3 => if a != 0 {
                pointer = operand as usize;
                continue;
            },
            4 => b ^= c,
            5 => result.push(combo % 8),
            6 => b = a / (1 << combo),
            7 => c = a / (1 << combo),
            _ => println!("Unknown opcode: {}", opcode),
        }
        pointer += 2;
    }
    result
}

fn get_combo(operand: u64, a: u64, b: u64, c: u64) -> u64 {
    match operand {
        4 => a,
        5 => b,
        6 => c,
        _ => operand,
    }
}


fn find_part_b_initial_a(b_init: u64, c_init: u64, programs: &[u64]) -> Option<u64> {
        let mut candidates = vec![0];
        while !candidates.is_empty() {
            let mut next: Vec<u64> = vec![];
            for base in candidates {
                for i in 0..=7 { 
                    let a: u64 = 8 * base + i;
                    let output = run_program(a, b_init, c_init, programs);
                    if output == programs {
                        return Some(a);
                    }
                    if programs.ends_with(&output) {
                        next.push(a);
                    }
                }
            }
            candidates = next;
        }
        None
}