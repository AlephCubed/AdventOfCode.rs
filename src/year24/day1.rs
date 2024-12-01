use std::fs;
use crate::utils::get_num;

pub fn solve() {
    let contents = fs::read_to_string("src/year24/day1.txt")
        .expect("file present");
    
    let (mut a, mut b): (Vec<&str>, Vec<&str>) = contents.split('\n').map(|s| {
        let mut split = s.splitn(2, ' ');
        (split.next().unwrap(), split.next().unwrap())
    }).unzip();
    
    a.sort();
    b.sort();
    
    let result: u32 = a.iter().zip(b).map(|(a, b)| {
        u32::abs_diff(get_num::<u32>(a), get_num::<u32>(b))
    }).sum();
    
    println!("The result is {result}.");
}