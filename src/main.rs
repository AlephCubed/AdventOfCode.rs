mod utils;

mod year24 {
    pub mod day2;
}

fn main() {
    println!("Solving A:");
    year24::day2::solve_a();

    println!("Solving B:");
    year24::day2::solve_b();
}