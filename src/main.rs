mod utils;

mod year24 {
    pub mod day3;
}

fn main() {
    println!("Solving A:");
    year24::day3::solve_a();

    println!("Solving B:");
    year24::day3::solve_b();
}