use std::fs;
use crate::year24::temp::part2;

mod utils;

mod year24 {
    pub mod day2;
    pub mod temp;
}

fn main() {
    println!("Solving A:");
    year24::day2::solve_a();

    println!("Solving B:");
    year24::day2::solve_b();

    // let contents = fs::read_to_string("src/year24/day2.txt")
    //     .expect("file present");
    // println!("{}", part2(&contents));
}