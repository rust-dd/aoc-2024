use std::collections::{HashMap, HashSet};
use std::fs;

pub fn solution() {
    let input = fs::read_to_string("./inputs/day24.txt").unwrap();
    let lines: Vec<&str> = input
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .collect();
    let mut init = HashMap::new();
    let mut gs = Vec::new();

    // A.
    let mut parsing_init = true;
    for l in &lines {
        if parsing_init {
            if let Some((w, v)) = l.split_once(':') {
                let v = v.trim();
                if v == "0" || v == "1" {
                    init.insert(w.trim().to_string(), v == "1");
                } else {
                    parsing_init = false;
                }
            } else {
                parsing_init = false;
            }
        }
        if !parsing_init {
            let parts: Vec<_> = l.split("->").collect();
            if parts.len() != 2 {
                continue;
            }
            let s: Vec<_> = parts[0].split_whitespace().collect();
            if s.len() != 3 {
                continue;
            }
            let g = match s[1].to_uppercase().as_str() {
                "AND" => 'A',
                "OR" => 'O',
                "XOR" => 'X',
                _ => continue,
            };
            gs.push((
                s[0].to_string(),
                s[2].to_string(),
                parts[1].trim().to_string(),
                g,
            ));
        }
    }
    let mut wires = HashMap::new();
    for (k, &v) in &init {
        wires.insert(k.clone(), Some(v));
    }
    for (a, b, o, _) in &gs {
        wires.entry(a.clone()).or_insert(None);
        wires.entry(b.clone()).or_insert(None);
        wires.entry(o.clone()).or_insert(None);
    }
    let mut change = true;
    while change {
        change = false;
        for (a, b, o, g) in &gs {
            if let (Some(a_val), Some(b_val)) = (wires[a], wires[b]) {
                let r = match g {
                    'A' => a_val && b_val,
                    'O' => a_val || b_val,
                    'X' => a_val ^ b_val,
                    _ => false,
                };
                if wires[o] != Some(r) {
                    wires.insert(o.clone(), Some(r));
                    change = true;
                }
            }
        }
    }
    let mut zs: Vec<_> = wires.keys().filter(|x| x.starts_with('z')).collect();
    zs.sort_by_key(|x| x[1..].parse::<usize>().unwrap_or(0));
    let dec = zs.iter().enumerate().fold(0u64, |acc, (i, k)| {
        acc | ((wires[*k] == Some(true)) as u64) << i
    });
    println!("Part A: {}", dec);

    // B.
    let connections = input.split_once("\n\n").map(|(_, c)| c).unwrap();
    let parsed: Vec<(String, char, String, String)> = connections
        .lines()
        .filter_map(|line| {
            let tokens: Vec<_> = line.split_whitespace().collect();
            if tokens.len() >= 5 && tokens[3] == "->" {
                let op = match tokens[1].to_uppercase().as_str() {
                    "AND" => 'A',
                    "OR" => 'O',
                    "XOR" => 'X',
                    _ => return None,
                };
                Some((
                    tokens[0].to_string(),
                    op,
                    tokens[2].to_string(),
                    tokens[4].to_string(),
                ))
            } else {
                None
            }
        })
        .collect();
    let connection_cache: HashSet<_> = parsed
        .iter()
        .flat_map(|(l, op, r, _)| [(l.clone(), *op), (r.clone(), *op)])
        .collect();
    let mut results: Vec<String> = parsed
        .iter()
        .filter_map(|(l, op, r, ret)| match *op {
            'A' if l != "x00" && r != "x00" && !connection_cache.contains(&(ret.clone(), 'O')) => {
                Some(ret.clone())
            }
            'X' if ((l.starts_with('x') || r.starts_with('x'))
                && l != "x00"
                && r != "x00"
                && !connection_cache.contains(&(ret.clone(), 'X')))
                || (!ret.starts_with('z') && !l.starts_with('x') && !r.starts_with('x')) =>
            {
                Some(ret.clone())
            }
            'O' if ret.starts_with('z') && ret != "z45" => Some(ret.clone()),
            _ => None,
        })
        .collect();
    results.sort();
    println!("Part B: {}", results.join(","));
}
