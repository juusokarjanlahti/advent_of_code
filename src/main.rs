mod days {
    pub mod day1;
    pub mod day2;
}

fn main() {
    println!("Advent of Code!");

    // Run specific days
    days::day1::run();
    days::day2::run();
}

pub fn run() {
    println!("Running Day 1");
    println!("Running Day 2");
}