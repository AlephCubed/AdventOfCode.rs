use std::fs::File;
use std::io;
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
    
}