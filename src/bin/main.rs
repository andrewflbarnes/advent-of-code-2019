use aoc2019::solutions::{d1, d2};
use std::env;

fn no_solution(_: String, _: String, _: &[String]) {
    println!("No solution for day");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("At least 1 argument is required - the day you want a solution for");
        return;
    }

    let day: u8 = args[1]
        .parse::<u8>()
        .expect("First argument must be a day number");

    let args = &args[2..];

    let exec = match day {
        1 => d1,
        2 => d2,
        _ => no_solution,
    };

    exec(format!("input/{}", day), format!("input/{}_2", day), args);
}
