use aoc2019::solutions;
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
        1 => solutions::d1,
        2 => solutions::d2,
        3 => solutions::d3,
        4 => solutions::d4,
        _ => return None,
    };

    Some(exec)
}
