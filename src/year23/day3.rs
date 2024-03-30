use std::fs;
use regex::bytes::Regex;

pub fn solve() {
    let file = fs::read("src/year23/day3.txt").unwrap();

    let part_regex = Regex::new(r"[^.\d\n]").unwrap();
    let num_regex = Regex::new(r"\d").unwrap();

    let mut width = 0;

    for char in &file {
        if char == &b'\n' {
            break
        }
        width += 1;
    }

    let map: Vec<&[u8]> = file
        .chunks(width)
        .collect();

    let mut targets: Vec<(usize, usize)> = vec![];

    for row in 0..map.len() {
        for col in 0..map[row].len() {
            let char = map[row][col];

            if part_regex.is_match(&[char]) {
                targets.push((row, col));
            }
        }
    }

    for target in targets {
        for row_off in 0..3 {
            if target.0 == 0 && row_off == 0 {
                continue
            }

            let row = target.0 + row_off - 1;

            if row >= map.len() {
                continue
            }

            for col_off in 0..3 {
                if target.1 == 0 && col_off == 0 {
                    continue
                }

                let col = target.0 + col_off - 1;

                if col >= map[row].len() {
                    continue
                }

                if num_regex.is_match(&[map[row][col]]) {
                    println!("{:?} is adjacent to {:?}", (row, col), target);
                }
            }
        }
    }
}