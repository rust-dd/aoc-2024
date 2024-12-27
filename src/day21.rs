use pathfinding::prelude::bfs;

static NUMPAD: [[Option<char>; 3]; 4] = [
    [Some('7'), Some('8'), Some('9')],
    [Some('4'), Some('5'), Some('6')],
    [Some('1'), Some('2'), Some('3')],
    [None, Some('0'), Some('A')],
];

static DIRPAD: [[Option<char>; 3]; 2] = [
    [None, Some('^'), Some('A')],
    [Some('<'), Some('v'), Some('>')],
];

#[derive(Clone, Copy, Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn all() -> &'static [Dir] {
        &[Dir::Up, Dir::Down, Dir::Left, Dir::Right]
    }

    fn delta(&self) -> (i32, i32) {
        match *self {
            Dir::Up => (-1, 0),
            Dir::Down => (1, 0),
            Dir::Left => (0, -1),
            Dir::Right => (0, 1),
        }
    }
}

/// The state consists of: (pos3, pos2, pos1, typed)
/// - pos3: your hand on the 2×3 directional keypad
/// - pos2: the second robot's hand on the 2×3 directional keypad
/// - pos1: the first robot's hand on the 4×3 numeric keypad
/// - typed: how many characters of the code have been typed so far
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct State {
    pos3: (i32, i32),
    pos2: (i32, i32),
    pos1: (i32, i32),
    typed: usize,
}

/// Initial state: all robots (and you) start on the 'A' button
fn start_state() -> State {
    State {
        pos3: (0, 2),
        pos2: (0, 2),
        pos1: (3, 2),
        typed: 0,
    }
}

fn dirpad_button(r: i32, c: i32) -> Option<char> {
    if r < 0 || r >= 2 || c < 0 || c >= 3 {
        None
    } else {
        DIRPAD[r as usize][c as usize]
    }
}

fn numpad_button(r: i32, c: i32) -> Option<char> {
    if r < 0 || r >= 4 || c < 0 || c >= 3 {
        None
    } else {
        NUMPAD[r as usize][c as usize]
    }
}

fn next_states(state: &State, code: &str) -> Vec<State> {
    let mut results = Vec::new();

    for d in Dir::all() {
        let (dr, dc) = d.delta();
        let nr = state.pos3.0 + dr;
        let nc = state.pos3.1 + dc;
        if let Some(_ch) = dirpad_button(nr, nc) {
            let mut st2 = state.clone();
            st2.pos3 = (nr, nc);
            results.push(st2);
        }
    }

    if let Some(ch3) = dirpad_button(state.pos3.0, state.pos3.1) {
        if ch3 == 'A' {
            if let Some(ch2) = dirpad_button(state.pos2.0, state.pos2.1) {
                if "^v<>".contains(ch2) {
                    let dir = match ch2 {
                        '^' => Dir::Up,
                        'v' => Dir::Down,
                        '<' => Dir::Left,
                        '>' => Dir::Right,
                        _ => unreachable!(),
                    };
                    let (dr, dc) = dir.delta();
                    let r1 = state.pos1.0 + dr;
                    let c1 = state.pos1.1 + dc;
                    if let Some(_nbtn) = numpad_button(r1, c1) {
                        let mut st2 = state.clone();
                        st2.pos1 = (r1, c1);
                        results.push(st2);
                    }
                } else if ch2 == 'A' {
                    if let Some(numch) = numpad_button(state.pos1.0, state.pos1.1) {
                        let mut st2 = state.clone();
                        if st2.typed < code.len() && code.as_bytes()[st2.typed] == numch as u8 {
                            st2.typed += 1;
                        }
                        results.push(st2);
                    }
                }
            }
        } else if "^v<>".contains(ch3) {
            let dir = match ch3 {
                '^' => Dir::Up,
                'v' => Dir::Down,
                '<' => Dir::Left,
                '>' => Dir::Right,
                _ => unreachable!(),
            };
            let (dr, dc) = dir.delta();
            let r2 = state.pos2.0 + dr;
            let c2 = state.pos2.1 + dc;
            if let Some(_ch2) = dirpad_button(r2, c2) {
                let mut st2 = state.clone();
                st2.pos2 = (r2, c2);
                results.push(st2);
            }
        }
    }

    results
}

pub fn solution() {
    let codes = ["671A", "279A", "083A", "974A", "386A"];
    let mut total_complexity = 0_u64;

    let bfs_shortest_len = |code: &str| -> usize {
        let goal_check = |s: &State| s.typed == code.len();
        let neighbors_fn = |s: &State| next_states(s, code);
        if let Some(path) = bfs(&start_state(), neighbors_fn, goal_check) {
            path.len() - 1
        } else {
            panic!("No solution found for code: {code}");
        }
    };

    let numeric_part = |code: &str| -> u32 {
        let trimmed = code.trim_end_matches('A');
        trimmed.parse::<u32>().unwrap()
    };

    // A.
    for &code in &codes {
        let length = bfs_shortest_len(code);
        let num = numeric_part(code);
        let complexity = (length as u64) * (num as u64);

        println!(
            "Code: {}, bfs_len: {}, numeric: {}, complexity: {}",
            code, length, num, complexity
        );

        total_complexity += complexity;
    }

    println!("Total complexity: {}", total_complexity);

    // B.
}
