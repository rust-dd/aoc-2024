use std::{fs, path::Path, thread, time};

use regex::Regex;

pub fn solution() {
    let input = fs::read_to_string(Path::new("./inputs/day17.txt")).unwrap();
    let re = Regex::new(
        r"Register A:\s*(\d+)\s*Register B:\s*(\d+)\s*Register C:\s*(\d+)\s*Program:\s*([\d,]+)",
    )
    .unwrap();
    let caps = re.captures(&input).unwrap();
    let mut a: u32 = caps[1].parse().unwrap();
    let mut b: u32 = caps[2].parse().unwrap();
    let mut c: u32 = caps[3].parse().unwrap();
    let programs: Vec<u32> = caps[4]
        .split(',')
        .map(|x| x.trim().parse().unwrap())
        .collect();

    let mut result = Vec::new();
    let mut pointer: usize = 0;

    while pointer < programs.len() {
        let opcode = programs[pointer];
        let operand = programs[pointer + 1];
        let combo = combo(operand, a, b, c);
        println!(
            "opcode: {:?}, operand: {:?}, combo: {:?}",
            opcode, operand, combo
        );

        match opcode {
            // adv
            0 => {
                a = a / (u32::pow(2, combo));
                println!("a: {}", a);
            }

            // bxl
            1 => {
                b = b ^ operand;
                println!("b: {}", b);
            }

            // bst
            2 => {
                b = combo % 8;
                println!("b: {}", b);
            }

            // jnz
            3 => {
                if a != 0 {
                    pointer = (operand ) as usize;
                    continue;
                }
            }

            // bxc
            4 => {
                b = b ^ c;
                println!("b {}", b);
            }

            // out
            5 => {
                let output = combo % 8;
                println!("Output: {}", output);
                result.push(output);
            }

            // bdv
            6 => {
                b = a / (u32::pow(2, combo));
                println!("b: {}", b);
            }

            // cdv
            7 => {
                c = a / (u32::pow(2, combo));
                println!("c: {}", c);
            }

            _ => {
                println!("Unknown opcode: {}", opcode);
            }
        }
        pointer += 2;
    }

    // Join the result for output
    let joined = result
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join(",");

    println!("Result: {}", joined);
}

fn combo(operand: u32, a: u32, b: u32, c: u32) -> u32 {
    match operand {
        4 => a,
        5 => b,
        6 => c,
        _ => operand,
    }
}
