mod utils;

mod year24 {
    pub mod day1;
}

fn main() {
    println!("Solving A:");
    year24::day1::solve_a();

    println!("Solving B:");
    year24::day1::solve_b();
}