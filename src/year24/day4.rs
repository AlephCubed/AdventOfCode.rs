use std::fs::File;
use std::io;
use std::io::BufRead;

pub fn solve_a() {
    let file = File::open("src/year24/day4.txt").unwrap();
    let lines = io::BufReader::new(file).lines();
}

pub fn solve_b() {
    let file = File::open("src/year24/day4.txt").unwrap();
    let lines = io::BufReader::new(file).lines();
}