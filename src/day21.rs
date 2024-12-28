use std::collections::HashMap;

struct Solution {
    moves1: HashMap<(char, char), Vec<String>>,
    moves2: HashMap<(char, char), Vec<String>>,
}

impl Solution {
    fn new(keypad1: &[&str], keypad2: &[&str]) -> Self {
        let moves1 = Self::parse_moves(keypad1);
        let moves2 = Self::parse_moves(keypad2);
        Self { moves1, moves2 }
    }

    fn parse_moves(keypad: &[&str]) -> HashMap<(char, char), Vec<String>> {
        fn get_key_pos(keypad: &[&str]) -> HashMap<char, (usize, usize)> {
            let mut pos = HashMap::new();
            for (r, row) in keypad.iter().enumerate() {
                for (c, key) in row.chars().enumerate() {
                    pos.insert(key, (r, c));
                }
            }
            pos
        }

        let pos = get_key_pos(keypad);
        let mut moves: HashMap<(char, char), Vec<String>> = HashMap::new();
        let mut keys: Vec<char> = pos.keys().cloned().collect();
        keys.sort();

        let hash_pos = pos.get(&'#').expect("Keypad must contain '#'");

        for &key1 in &keys {
            if key1 == '#' {
                continue;
            }
            for &key2 in &keys {
                if key2 == '#' || key1 == key2 {
                    continue;
                }
                let p1 = pos[&key1];
                let p2 = pos[&key2];
                let mut sequences = Vec::new();

                if p1.0 == p2.0 {
                    let dir = if p2.1 > p1.1 { '>' } else { '<' };
                    let distance = (p2.1 as isize - p1.1 as isize).abs();
                    let move_seq = format!("{}A", dir.to_string().repeat(distance as usize));
                    sequences.push(move_seq);
                } else if p1.1 == p2.1 {
                    let dir = if p2.0 > p1.0 { 'v' } else { '^' };
                    let distance = (p2.0 as isize - p1.0 as isize).abs();
                    let move_seq = format!("{}A", dir.to_string().repeat(distance as usize));
                    sequences.push(move_seq);
                } else {
                    if p1.0 != hash_pos.0 || p2.1 != hash_pos.1 {
                        let dir1 = if p2.1 > p1.1 { '>' } else { '<' };
                        let distance1 = (p2.1 as isize - p1.1 as isize).abs();
                        let dir2 = if p2.0 > p1.0 { 'v' } else { '^' };
                        let distance2 = (p2.0 as isize - p1.0 as isize).abs();
                        let move_seq = format!(
                            "{}{}A",
                            dir1.to_string().repeat(distance1 as usize),
                            dir2.to_string().repeat(distance2 as usize)
                        );
                        sequences.push(move_seq);
                    }

                    // First vertical then horizontal
                    if p1.1 != hash_pos.1 || p2.0 != hash_pos.0 {
                        let dir1 = if p2.0 > p1.0 { 'v' } else { '^' };
                        let distance1 = (p2.0 as isize - p1.0 as isize).abs();
                        let dir2 = if p2.1 > p1.1 { '>' } else { '<' };
                        let distance2 = (p2.1 as isize - p1.1 as isize).abs();
                        let move_seq = format!(
                            "{}{}A",
                            dir1.to_string().repeat(distance1 as usize),
                            dir2.to_string().repeat(distance2 as usize)
                        );
                        sequences.push(move_seq);
                    }
                }

                if !sequences.is_empty() {
                    moves.insert((key1, key2), sequences);
                }
            }
        }

        moves
    }

    fn build_combinations(&self, arrays: &[Vec<String>]) -> Vec<Vec<String>> {
        let mut results: Vec<Vec<String>> = vec![Vec::new()];
        for array in arrays {
            let mut temp = Vec::new();
            for combination in &results {
                for item in array {
                    let mut new_combination = combination.clone();
                    new_combination.push(item.clone());
                    temp.push(new_combination);
                }
            }
            results = temp;
        }
        results
    }

    fn translate(
        &self,
        code: &str,
        depth: usize,
        cache: &mut HashMap<(String, usize), usize>,
    ) -> usize {
        if let Some(&cached) = cache.get(&(code.to_string(), depth)) {
            return cached;
        }

        let moves = if code.chars().next().unwrap().is_numeric() {
            self.translate_numpad(code)
        } else {
            self.translate_keypad(code)
        };

        let result = if depth == 0 {
            moves
                .iter()
                .map(|move_seq| move_seq.iter().map(|s| s.len()).sum::<usize>())
                .min()
                .unwrap_or(0)
        } else {
            moves
                .iter()
                .map(|move_seq| {
                    move_seq
                        .iter()
                        .map(|s| self.translate(s, depth - 1, cache))
                        .sum::<usize>()
                })
                .min()
                .unwrap_or(0)
        };

        cache.insert((code.to_string(), depth), result);
        result
    }

    fn translate_numpad(&self, code: &str) -> Vec<Vec<String>> {
        let mut code_chars: Vec<char> = code.chars().collect();
        code_chars.insert(0, 'A');
        let mut moves_list = Vec::new();

        for pair in code_chars.windows(2) {
            if let [a, b] = pair {
                if let Some(moves) = self.moves1.get(&(*a, *b)) {
                    moves_list.push(moves.clone());
                } else {
                    moves_list.push(vec!["A".to_string()]);
                }
            }
        }

        self.build_combinations(&moves_list)
    }

    fn translate_keypad(&self, code: &str) -> Vec<Vec<String>> {
        let mut code_chars: Vec<char> = code.chars().collect();
        code_chars.insert(0, 'A');
        let mut moves_list = Vec::new();

        for pair in code_chars.windows(2) {
            if let [a, b] = pair {
                if *a != *b {
                    if let Some(moves) = self.moves2.get(&(*a, *b)) {
                        moves_list.push(moves.clone());
                    } else {
                        // If no move exists, use a default move
                        moves_list.push(vec!["A".to_string()]);
                    }
                } else {
                    moves_list.push(vec!["A".to_string()]);
                }
            }
        }

        self.build_combinations(&moves_list)
    }

    fn solve(&self, data: &[&str], depth: usize) -> usize {
        let mut complexities = 0;
        let mut cache = HashMap::new();

        for code in data {
            let min_len = self.translate(code, depth, &mut cache);
            if let Some(num_str) = code.strip_suffix(&code.chars().last().unwrap().to_string()) {
                if let Ok(num) = num_str.parse::<usize>() {
                    complexities += min_len * num;
                }
            }
        }

        complexities
    }
}

pub fn solution() {
    let keypad1 = ["789", "456", "123", "#0A"];
    let keypad2 = ["#^A", "<v>"];
    let solution = Solution::new(&keypad1, &keypad2);
    let data = ["671A", "279A", "083A", "974A", "386A"];

    let part1 = solution.solve(&data, 2);
    println!("Part 1: {}", part1);

    // Part 2 is insipired by:
    // https://github.com/nitekat1124/advent-of-code-2024/blob/main/solutions/day21.py
    let part2 = solution.solve(&data, 25);
    println!("Part 2: {}", part2);
}
