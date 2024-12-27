use itertools::iproduct;
use rayon::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

const MODULO: u64 = 16_777_216;

fn next_secret(s: u32) -> u32 {
    let mut s64 = s as u64;
    let m1 = s64.checked_mul(64).unwrap();
    s64 ^= m1;
    s64 %= MODULO;
    let m2 = s64 / 32;
    s64 ^= m2;
    s64 %= MODULO;
    let m3 = s64.checked_mul(2048).unwrap();
    s64 ^= m3;
    s64 %= MODULO;
    s64 as u32
}

fn generate_2000th_secret(initial: u32) -> u32 {
    let mut s = initial;
    for _ in 0..2000 {
        s = next_secret(s);
    }
    s
}

fn generate_secret_sequence(initial: u32) -> Vec<u32> {
    let mut seq = Vec::with_capacity(2001);
    seq.push(initial);
    let mut current = initial;
    for _ in 0..2000 {
        current = next_secret(current);
        seq.push(current);
    }
    seq
}

fn to_prices(secrets: &[u32]) -> Vec<u8> {
    secrets.iter().map(|&s| (s % 10) as u8).collect()
}

fn to_price_changes(prices: &[u8]) -> Vec<i8> {
    let mut changes = Vec::with_capacity(prices.len().saturating_sub(1));
    for w in prices.windows(2) {
        changes.push(w[1] as i8 - w[0] as i8);
    }
    changes
}

fn first_occurrence_bananas(changes: &[i8], prices: &[u8], pattern: &[i8]) -> u32 {
    if changes.len() < pattern.len() {
        return 0;
    }
    for i in 0..=(changes.len() - pattern.len()) {
        if &changes[i..i + pattern.len()] == pattern {
            return prices[i + pattern.len()] as u32;
        }
    }
    0
}

pub fn solution() {
    // A.
    let file = File::open("./inputs/day22.txt").unwrap();
    let reader = BufReader::new(file);
    let mut inputs_part1 = Vec::new();
    for line in reader.lines() {
        let value: u32 = line.unwrap().trim().parse().unwrap();
        inputs_part1.push(value);
    }

    let mut total_sum_part1 = 0u64;
    for &initial_secret in &inputs_part1 {
        let val = generate_2000th_secret(initial_secret);
        total_sum_part1 += val as u64;
    }

    println!("Part 1 total sum: {}", total_sum_part1);

    // B.
    let mut all_prices = Vec::new();
    let mut all_changes = Vec::new();
    for &initial_secret in &inputs_part1 {
        let secrets = generate_secret_sequence(initial_secret);
        let prices = to_prices(&secrets);
        let changes = to_price_changes(&prices);
        all_prices.push(prices);
        all_changes.push(changes);
    }

    // Brute force all possible patterns.
    // The pattern is a 4-tuple of i8 values.
    // This is a brute force solution, but it's fast enough.
    let best_sum = iproduct!(-9..=9, -9..=9, -9..=9, -9..=9)
        .par_bridge()
        .into_par_iter()
        .map(|(a, b, c, d)| {
            let pattern = [a, b, c, d];
            all_changes
                .par_iter()
                .zip(all_prices.par_iter())
                .map(|(ch, pr)| first_occurrence_bananas(ch, pr, &pattern) as u64)
                .sum::<u64>()
        })
        .max()
        .unwrap_or(0);

    println!("Part 2 max bananas: {}", best_sum);
}
