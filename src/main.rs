use std::env;
pub mod day1;

fn main() {
    let day = env::args().collect::<Vec<String>>()[1].parse::<i32>().unwrap();
    println!("Running solution for day {}", day);

    match day{
        1 => println!("Solution: {}", day1::solve()),
        _ => { println!("Unknown day")}
    }
}
