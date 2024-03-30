use std::fs;
use regex::Regex;

fn main() {
    let numbers = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let contents = fs::read_to_string("src/day1.txt")
        .expect("file present");

    let mut total : i32 = 0;

    for line in contents.split("\n") {

        let mut new_line: String = line.to_string();
        for (index, number) in numbers.iter().copied().enumerate() {
            let result = [number, number].join(&(index + 1).to_string());
            new_line = new_line.replace(number, &result);
        }

        let regex = Regex::new(r"\D").unwrap();
        let digits = regex.replace_all(&new_line, "");
        let string =
            format!("{}{}",
            digits.chars().nth(0).expect("expected char"),
            digits.chars().nth(digits.len() - 1).expect("expected char"));
        total += string.parse::<i32>().unwrap();
    }

    println!("{total}");
}