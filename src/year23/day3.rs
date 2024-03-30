use std::fs;
use regex::bytes::Regex;

pub fn solve() {
    let file = fs::read("src/year23/day3.txt").unwrap();

    let regex = Regex::new(r"[^.\d\n]").unwrap();

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

            if regex.is_match(&[char]) {
                targets.push((row, col));
            }
        }
    }

    println!("{:?}", targets);
}