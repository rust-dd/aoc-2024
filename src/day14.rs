use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Robot {
    x: i32,
    y: i32,
    velocity_x: i32,
    velocity_y: i32,
}

fn read_input(path: &str) -> std::io::Result<Vec<Robot>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let re = Regex::new(r"^p=(-?\d+),(-?\d+)\s+v=(-?\d+),(-?\d+)$").unwrap();

    let mut robots = Vec::new();
    let mut lines = reader.lines().filter_map(|l| l.ok());

    while let Some(line) = lines.next() {
        if let Some(caps) = re.captures(&line) {
            let p1 = caps.get(1).unwrap().as_str();
            let p2 = caps.get(2).unwrap().as_str();
            let v1 = caps.get(3).unwrap().as_str();
            let v2 = caps.get(4).unwrap().as_str();

            robots.push(Robot {
                x: p1.parse::<i32>().unwrap(),
                y: p2.parse::<i32>().unwrap(),
                velocity_x: v1.parse::<i32>().unwrap(),
                velocity_y: v2.parse::<i32>().unwrap(),
            });
        } else {
            println!("No match found.");
        }
    }

    Ok(robots)
}

pub fn solution() {
    let mut robots = read_input("./inputs/day14.txt").unwrap();

    const WIDE: usize = 101;
    const TALL: usize = 103;
    const SECONDS: i32 = 100;
    let middle_x = (WIDE as i32) / 2;
    let middle_y = (TALL as i32) / 2;

    let mut quadrant_counts = [0; 4];

    for robot in &mut robots {
        robot.x = (robot.x + SECONDS * robot.velocity_x) % (WIDE as i32);
        robot.y = (robot.y + SECONDS * robot.velocity_y) % (TALL as i32);

        if robot.x < 0 {
            robot.x += WIDE as i32;
        }
        if robot.y < 0 {
            robot.y += TALL as i32;
        }
        if robot.x != middle_x && robot.y != middle_y {
            if robot.x > middle_x && robot.y < middle_y {
                quadrant_counts[0] += 1; 
            } else if robot.x < middle_x && robot.y < middle_y {
                quadrant_counts[1] += 1;
            } else if robot.x < middle_x && robot.y > middle_y {
                quadrant_counts[2] += 1;
            } else if robot.x > middle_x && robot.y > middle_y {
                quadrant_counts[3] += 1;
            }
        }

    }

    let result = quadrant_counts[0] * quadrant_counts[1] * quadrant_counts[2] * quadrant_counts[3];
    println!("Result: {}", result);

}
