use std::fs;

use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

pub fn solution() {
    let input = fs::read_to_string("./inputs/day07.txt").expect("Failed to read input file");
    let data: IResult<&str, Vec<(u64, Vec<u64>)>, nom::error::Error<&str>> = separated_list1(
        line_ending,
        separated_pair(
            complete::u64,
            tag(": "),
            separated_list1(space1, complete::u64),
        ),
    )(input.as_str());
    let (_, data) = data.unwrap();

    // A.
    let mut result = 0;

    for (left, right) in &data {
        let len = right.len();

        if len < 1 {
            continue;
        }

        // Test every combination of add/mul between the right side
        // All combinations are 2^(len - 1)
        for i in 0..(1 << (len - 1)) {
            let mut sum = right[0];
            for j in 0..(len - 1) {
                if i & (1 << j) != 0 {
                    sum += right[j + 1];
                } else {
                    sum *= right[j + 1];
                }
            }

            if &sum == left {
                result += left;
                break;
            }
        }
    }

    println!("Result part A: {}", result);

    // B.
    let mut result = 0;

    for (left, right) in &data {
        let len = right.len();

        if len < 1 {
            continue;
        }

        // Test every combination of add/mul between the right side
        // All combinations are 2^(len - 1)
        // || use for concatenation and get the result of the left side
        // e.g 156: 15 6
        // 156 = 15 || 6
        for mut i in 0..3i64.pow((len - 1) as u32) {
            let mut sum = right[0];

            for j in 0..(len - 1) {
                let op = i % 3;
                i /= 3;

                match op {
                    0 => sum += right[j + 1],
                    1 => sum *= right[j + 1],
                    _ => {
                        sum = format!("{}{}", sum, right[j + 1].to_string())
                            .parse()
                            .unwrap()
                    }
                }
            }

            if &sum == left {
                result += left;
                break;
            }
        }
    }

    println!("Result part B: {}", result);
}
