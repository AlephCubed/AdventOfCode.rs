use std::fs::File;
use std::{fs, io};
use std::io::BufRead;

pub fn solve_a() {
    let contents = fs::read_to_string("src/year24/day4.txt")
        .expect("file present");
    
    let grid: Vec<Vec<char>> = contents
        .split('\n')
        .map(|l| l.chars().collect())
        .collect();
    
    let horizontal: u32 = grid.iter().map(|x| check_line(x.clone())).sum();
    println!("Horizontal: {horizontal}");

    println!("Result: {}", horizontal);
}

fn check_line(mut line: Vec<char>) -> u32 {
    let forward = check_line_one_way(&line);
    println!("Forward: {forward}");
    line.reverse();
    let backward = check_line_one_way(&line);
    println!("Backward: {backward}");
    
    forward + backward
}

fn check_line_one_way(line: &Vec<char>) -> u32 {
    let (_, count) = line.iter().fold((0, 0), |(mut state, mut count), c| {
        let b = match state { 
            0 => *c == 'X',
            1 => *c == 'M',
            2 => *c == 'A',
            3 => *c == 'S',
            _ => unreachable!(),
        };
        
        if b {
            if state == 3 {
                state = 0;
                count += 1;
            } else {
                state += 1;
            }
        } else {
            state = if *c == 'X' { 1 } else { 0 };
        }

        (state, count)
    });
    count
}

pub fn solve_b() {
    let file = File::open("src/year24/day4.txt").unwrap();
    let lines = io::BufReader::new(file).lines();
}