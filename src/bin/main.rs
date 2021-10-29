use aoc2019::solutions::{d1, d2, d3};
use std::env;

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

    if let Some(exec) = get_solution(day) {
        exec(format!("input/{}", day), format!("input/{}_2", day), args);
    } else {
        println!("No solution for day {}", day);
    }
}

fn get_solution(day: u8) -> Option<fn(String, String, &[String])> {
    let exec = match day {
        1 => d1,
        2 => d2,
        3 => d3,
        _ => return None,
    };

    Some(exec)
}
