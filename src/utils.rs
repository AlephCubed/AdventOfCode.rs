use regex::Regex;

pub fn get_num(input: &str) -> i32 {
    let regex = Regex::new(r"\D").expect("valid regex");
    regex.replace_all(input, "").parse::<i32>().expect("valid parse")
}