use std::fs;
use regex::Regex;

fn main() {
    let contents = fs::read_to_string("src/day1.txt")
        .expect("file present");

    let mut total : i32 = 0;

    for line in contents.split("\n") {
        let regex = Regex::new(r"\D");
        let digits = regex.unwrap().replace_all(line, "");
        let string =
            format!("{}{}",
            digits.chars().nth(0).expect("expected char"),
            digits.chars().nth(digits.len() - 1).expect("expected char"));
        total += string.parse::<i32>().unwrap();
    }

    println!("{total}");
}
