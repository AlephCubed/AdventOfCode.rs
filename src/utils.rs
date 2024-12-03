use std::fmt::Debug;
use std::str::FromStr;
use regex::Regex;

#[allow(dead_code)]
#[deprecated]
pub fn get_num<T: FromStr>(input: &str) -> T 
    where <T as FromStr>::Err: Debug 
{
    let regex = Regex::new(r"\D").expect("valid regex");
    regex.replace_all(input, "").parse::<T>().expect("valid parse")
}