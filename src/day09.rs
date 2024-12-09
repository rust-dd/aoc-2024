use itertools::Itertools;
use std::fs;

pub fn solution() {
    let input = fs::read_to_string("./inputs/day09.txt").expect("Failed to read input file");
    let input = input.trim();
    let mut disk = Vec::new();

    let bytes = input.bytes().enumerate();
    let mut bytes_rev = bytes.clone().rev();

    let mut bytes_rev_i;
    let mut bytes_rev_i_id;
    let mut bytes_rev_b;
    let mut bytes_rev_b_remaining;

    (bytes_rev_i, bytes_rev_b) = bytes_rev.next().unwrap();

    if bytes_rev_i % 2 == 1 {
        (bytes_rev_i, bytes_rev_b) = bytes_rev.next().unwrap();
    }

    bytes_rev_i_id = bytes_rev_i / 2;
    bytes_rev_b_remaining = bytes_rev_b - b'0';

    for (i, b) in bytes {
        if i >= bytes_rev_i {
            for _ in 0..bytes_rev_b_remaining {
                disk.push(bytes_rev_i_id);
            }
            break;
        }
        if i % 2 == 0 {
            let i_id = i / 2;
            for _ in 0..(b - b'0') {
                disk.push(i_id);
            }
        } else {
            for _ in 0..(b - b'0') {
                if bytes_rev_b_remaining == 0 {
                    _ = bytes_rev.next().unwrap();
                    (bytes_rev_i, bytes_rev_b) = bytes_rev.next().unwrap();
                    if i >= bytes_rev_i {
                        break;
                    }
                    bytes_rev_i_id = bytes_rev_i / 2;
                    bytes_rev_b_remaining = bytes_rev_b - b'0';
                }
                bytes_rev_b_remaining -= 1;
                disk.push(bytes_rev_i_id);
            }
        }
    }

    let sum: usize = disk.into_iter().enumerate().map(|(i, id)| i * id).sum();
    println!("Result part A: {}", sum);

    // B.
    // (id, start, length)
    let mut files = Vec::new();

    let _ = input.bytes().enumerate().fold(0, |mut c, (i, b)| {
        let length = b - b'0';

        if i % 2 == 0 {
            files.push((i / 2, c, length));
        }

        c += length as usize;
        c
    });

    for id in (0..=files.last().unwrap().0).rev() {
        let file_index = files.iter().position(|file| file.0 == id).unwrap();

        if let Some(new_position) = files
            .iter()
            .tuple_windows()
            .find_map(|(a, b)| {
                let a_end = a.1 + a.2 as usize;
                let f = &files[file_index];

                if a_end > f.1 {
                    return Some(None);
                }

                let gap = b.1 - a_end;
                if gap >= f.2 as usize {
                    return Some(Some(a_end));
                }

                None
            })
            .flatten()
        {
            files[file_index].1 = new_position;
        }

        files.sort_by_key(|f| f.1);
    }

    let sum = files
        .into_iter()
        .map(|f| {
            (f.1..f.1 + f.2 as usize)
                .map(|idx| idx * f.0)
                .sum::<usize>()
        })
        .sum::<usize>();

    println!("Result part B: {}", sum);
}
