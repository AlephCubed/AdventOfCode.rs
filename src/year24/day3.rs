use std::fs::File;
use std::{fs, io};
use std::io::BufRead;
use regex::Regex;

pub fn solve_a() {
    let file = File::open("src/year24/day3.txt").unwrap();
    let lines = io::BufReader::new(file).lines();
    let regex = Regex::new(r"mul\(\d?\d?\d?,\d?\d?\d?\)").expect("valid regex");

    let mut total = 0;
    
    for line in lines.flatten() {
        for mul in regex.find_iter(line.as_str()) {
            let s = mul.as_str();
            let s = s.strip_prefix("mul(").unwrap();
            let s = s.strip_suffix(')').unwrap();
            let num_strings = s.split(',');
            let nums = num_strings.map(|s| s.parse::<u32>().unwrap());
            
            total += nums.product::<u32>();
        }
    }

    println!("Result: {total}");
}

pub fn solve_b() {
    let contents = fs::read_to_string("src/year24/day3.txt")
        .expect("file present");
    let regex = Regex::new(r"mul\(\d?\d?\d?,\d?\d?\d?\)").expect("valid regex");
    let strip_regex = Regex::new(r"(?s)don't\(\).*?do\(\)").expect("valid regex");

    let mut total = 0;

    let string = strip_regex.replace_all(&contents, "");
    
    for mul in regex.find_iter(&*string) {
        let s = mul.as_str();
        let s = s.strip_prefix("mul(").unwrap();
        let s = s.strip_suffix(')').unwrap();
        let num_strings = s.split(',');
        let nums = num_strings.map(|s| s.parse::<u32>().unwrap());

        total += nums.product::<u32>();
    }

    println!("Result: {total}");
}