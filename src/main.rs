mod utils;

mod year24 {
    pub mod day4;
}

fn main() {
    println!("Solving A:");
    year24::day4::solve_a();

    println!("Solving B:");
    year24::day4::solve_b();
}