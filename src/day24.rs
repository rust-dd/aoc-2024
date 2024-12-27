use std::collections::{HashMap, HashSet};
use std::fs;

enum G {
    A,
    O,
    X,
}
struct Q {
    a: String,
    b: String,
    o: String,
    g: G,
}

pub fn solution() {
    let input = fs::read_to_string("./inputs/day24.txt").unwrap();
    let lines = input
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>();

    let mut init = HashMap::<String, bool>::new();
    let mut gs = Vec::<Q>::new();
    let mut parsing_init = true;

    for l in &lines {
        if parsing_init {
            if let Some((w, v)) = l.split_once(':') {
                let (w, v) = (w.trim().to_string(), v.trim());
                if v == "0" || v == "1" {
                    init.insert(w, v == "1");
                } else {
                    parsing_init = false;
                }
            } else {
                parsing_init = false;
            }
        }
        if !parsing_init {
            let parts = l.split("->").collect::<Vec<_>>();
            if parts.len() != 2 {
                continue;
            }
            let out = parts[1].trim().to_string();
            let s: Vec<_> = parts[0].split_whitespace().collect();
            if s.len() != 3 {
                continue;
            }
            let g = match s[1].to_uppercase().as_str() {
                "AND" => G::A,
                "OR" => G::O,
                "XOR" => G::X,
                _ => continue,
            };
            gs.push(Q {
                a: s[0].into(),
                b: s[2].into(),
                o: out,
                g,
            });
        }
    }

    let mut wires = HashMap::<String, Option<bool>>::new();

    for (k, &v) in &init {
        wires.insert(k.clone(), Some(v));
    }

    for g in &gs {
        wires.entry(g.a.clone()).or_insert(None);
        wires.entry(g.b.clone()).or_insert(None);
        wires.entry(g.o.clone()).or_insert(None);
    }

    let mut change = true;
    while change {
        change = false;
        for g in &gs {
            if let (Some(a), Some(b)) = (wires[&g.a], wires[&g.b]) {
                let r = match g.g {
                    G::A => a && b,
                    G::O => a || b,
                    G::X => a ^ b,
                };
                let out = wires.get_mut(&g.o).unwrap();
                if out.is_none() || out.unwrap() != r {
                    *out = Some(r);
                    change = true;
                }
            }
        }
    }

    let mut zs = wires
        .keys()
        .filter(|x| x.starts_with('z'))
        .collect::<Vec<_>>();
    zs.sort_by_key(|x| x[1..].parse::<usize>().unwrap_or(0));
    let mut dec = 0u64;

    for (i, k) in zs.iter().enumerate() {
        if wires[*k] == Some(true) {
            dec |= 1 << i;
        }
    }

    println!("Part A: {}", dec);

    let (_, connections) = input.split_once("\n\n").unwrap();
    let lines = connections
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>();
    let mut parsed = Vec::new();

    for line in lines {
        let tokens: Vec<_> = line.split_whitespace().collect();
        if tokens.len() < 5 {
            continue;
        }
        if tokens[3] != "->" {
            continue;
        }
        let op_char = match tokens[1].to_uppercase().as_str() {
            "AND" => 'A',
            "OR" => 'O',
            "XOR" => 'X',
            _ => continue,
        };
        parsed.push((
            tokens[0].to_string(),
            op_char,
            tokens[2].to_string(),
            tokens[4].to_string(),
        ));
    }

    let mut connection_cache = HashSet::new();

    for (l, op, r, _) in &parsed {
        connection_cache.insert((l.clone(), *op));
        connection_cache.insert((r.clone(), *op));
    }

    let mut results = Vec::new();

    for (l, op, r, ret) in &parsed {
        match op {
            'A' => {
                if l != "x00" && r != "x00" && !connection_cache.contains(&(ret.clone(), 'O')) {
                    results.push(ret.clone());
                }
            }
            'X' => {
                if ((l.starts_with('x') || r.starts_with('x'))
                    && l != "x00"
                    && r != "x00"
                    && !connection_cache.contains(&(ret.clone(), 'X')))
                    || (!ret.starts_with('z') && !l.starts_with('x') && !r.starts_with('x'))
                {
                    results.push(ret.clone());
                }
            }
            'O' => {
                if ret.starts_with('z') && ret != "z45" {
                    results.push(ret.clone());
                }
            }
            _ => {}
        }
    }

    results.sort();
    println!("Part B: {}", results.join(","));
}
