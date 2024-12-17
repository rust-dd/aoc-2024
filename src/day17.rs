use std::fs;
use regex::Regex;

pub fn solution() {
    let input = fs::read_to_string("./inputs/day17.txt").expect("Failed to read file");
    let re = Regex::new(r"Register A:\s*(\d+)\s*Register B:\s*(\d+)\s*Register C:\s*(\d+)\s*Program:\s*([\d,]+)").unwrap();
    let caps = re.captures(&input).expect("Failed to parse input");

    let mut a: u32 = caps[1].parse().unwrap();
    let mut b: u32 = caps[2].parse().unwrap();
    let mut c: u32 = caps[3].parse().unwrap();
    let programs: Vec<u32> = caps[4]
        .split(',')
        .map(|x| x.trim().parse().unwrap())
        .collect();

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

    // Join result for output
    let output = result.iter()
        .map(u32::to_string)
        .collect::<Vec<_>>()
        .join(",");
    println!("Result: {}", output);
}

fn get_combo(operand: u32, a: u32, b: u32, c: u32) -> u32 {
    match operand {
        4 => a,
        5 => b,
        6 => c,
        _ => operand,
    }
}
