use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Machine {
    x_a: usize,
    y_a: usize,
    x_b: usize,
    y_b: usize,
    x_prize: usize,
    y_prize: usize,
}

fn read_input(path: &str) -> std::io::Result<Vec<Machine>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let re_button_a = Regex::new(r"Button A:\s*X\+(\d+),\s*Y\+(\d+)").unwrap();
    let re_button_b = Regex::new(r"Button B:\s*X\+(\d+),\s*Y\+(\d+)").unwrap();
    let re_prize = Regex::new(r"Prize:\s*X=(\d+),\s*Y=(\d+)").unwrap();

    let mut machines = Vec::new();
    let mut lines = reader.lines().filter_map(|l| l.ok());

    while let Some(line_a) = lines.next() {
        if line_a.trim().is_empty() {
            continue;
        }
        let line_b = match lines.next() {
            Some(line) => line,
            None => break,
        };
        let line_p = match lines.next() {
            Some(line) => line,
            None => break,
        };

        let caps_a = re_button_a.captures(&line_a).unwrap();
        let caps_b = re_button_b.captures(&line_b).unwrap();
        let caps_p = re_prize.captures(&line_p).unwrap();

        let x_a = caps_a[1].parse::<usize>().unwrap();
        let y_a = caps_a[2].parse::<usize>().unwrap();
        let x_b = caps_b[1].parse::<usize>().unwrap();
        let y_b = caps_b[2].parse::<usize>().unwrap();
        let x_prize = caps_p[1].parse::<usize>().unwrap();
        let y_prize = caps_p[2].parse::<usize>().unwrap();

        machines.push(Machine {
            x_a,
            y_a,
            x_b,
            y_b,
            x_prize,
            y_prize,
        });

        let _ = lines.next();
    }

    Ok(machines)
}

pub fn solution() {
    let machines = read_input("./inputs/day13.txt").unwrap();

    // A.
    // a * (XA, YA) + b * (XB, YB) = (X_prize, Y_prize)
    let mut total = 0;
    for m in &machines {
        let mut min_cost = None;
        for a in 0..=100 {
            for b in 0..=100 {
                let x = a * m.x_a + b * m.x_b;
                let y = a * m.y_a + b * m.y_b;
                if x == m.x_prize && y == m.y_prize {
                    let cost = 3 * a + b;
                    match min_cost {
                        None => min_cost = Some(cost),
                        Some(prev) if cost < prev => min_cost = Some(cost),
                        _ => {}
                    }
                }
            }
        }

        if let Some(c) = min_cost {
            total += c;
        }
    }

    println!("Total cost: {}", total);

    // B.
    // We add the offset (10,000,000,000,000) to each prize coordinate.
    // Instead of brute-forcing values of `a` and `b`, we use a direct formula approach to calculate them:
    //
    // a = (x_b * y_prize - y_b * x_prize) / (x_a * y_b - y_a * x_b)
    // b = (x_prize - x_a * a) / x_b
    //
    // Here:
    // - `x_a`, `y_a` are the x and y increments when Button A is pressed.
    // - `x_b`, `y_b` are the x and y increments when Button B is pressed.
    // - `(x_prize, y_prize)` is the target coordinate (after adding the offset in Part B).
    //
    // Let D = (x_a * y_b - y_a * x_b), which is the determinant of the 2D linear system:
    //     a * (x_a, y_a) + b * (x_b, y_b) = (x_prize, y_prize)
    //
    // If D ≠ 0, the vectors (x_a, y_a) and (x_b, y_b) are not collinear, so the system has a unique solution in real numbers.
    //     - We calculate `a` and `b` using the formulas above.
    //     - Check if `a` and `b` are integers (i.e., no remainder when dividing).
    //     - Check if `a` and `b` are both nonnegative.
    //     - If all conditions are satisfied, the solution is valid, and the cost is `3 * a + b`.
    //
    // If D = 0, the vectors (x_a, y_a) and (x_b, y_b) are collinear.
    //     - In this case, the system cannot determine a unique solution using the direct formula approach.
    //     - Special handling is needed to check if the target coordinate (x_prize, y_prize) is reachable along the single direction defined by the collinear vectors.
    //     - If reachable, we calculate the minimum cost separately (e.g., using parameterized solutions along the line).
    //
    // If the solution is invalid (e.g., non-integer `a` or `b`, or negative values), we skip the machine for Part B.
    //
    let offset = 10_000_000_000_000usize;
    let mut total = 0;
    let mut solved = 0;

    for m in &machines {
        let x_prize_b = (m.x_prize as i64) + offset as i64;
        let y_prize_b = (m.y_prize as i64) + offset as i64;

        let x_a = m.x_a as i64;
        let y_a = m.y_a as i64;
        let x_b = m.x_b as i64;
        let y_b = m.y_b as i64;

        let d = x_a * y_b - y_a * x_b;
        if d == 0 {
            // Collinear or no solution
            continue;
        } else {
            // Non-collinear => unique solution (if integral)
            let a_num = y_b * x_prize_b - x_b * y_prize_b;
            let denom = d;
            if a_num % denom != 0 {
                // not an integer solution
                continue;
            }

            let a = a_num / denom;
            let b_num = x_prize_b - x_a * a;
            if x_b == 0 {
                // If xb=0 but we ended up here, it's likely no solution or handled differently.
                continue;
            }

            if b_num % x_b != 0 {
                // not integer
                continue;
            }

            let b = b_num / x_b;

            // Check nonnegative and that it indeed hits the goal
            if a < 0 || b < 0 {
                continue;
            }

            let final_x = x_a * a + x_b * b;
            let final_y = y_a * a + y_b * b;
            if final_x == x_prize_b && final_y == y_prize_b {
                let cost = 3 * a + b;
                total += cost;
                solved += 1;
            }
        }
    }

    println!("Part B, solved {} with total cost {}", solved, total);
}
