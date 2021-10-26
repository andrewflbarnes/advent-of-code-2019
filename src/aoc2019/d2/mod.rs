use crate::utils;
mod vm;
use vm::Computer;

pub fn solve(input1: String, _: String, _: &[String]) {
    let program: Vec<i32> = utils::read_file_lines(&input1)
        .into_iter()
        .flat_map(|l| l.split(",").map(str::to_owned).collect::<Vec<_>>())
        .filter_map(|i| i.to_owned().parse::<i32>().ok())
        .collect();

    let part1 = run_with(&program, (12, 2));
    println!("Program execution ended with result: {}", part1);

    for noun in 0..=255 {
        for verb in 0..=255 {
            if 19690720 == run_with(&program, (noun, verb)) {
                println!(
                    "Program execution ended with program code: {}",
                    noun * 100 + verb
                );
            }
        }
    }
}

fn run_with(program: &Vec<i32>, init: (i32, i32)) -> i32 {
    let mut copy = program.clone();
    copy[1] = init.0;
    copy[2] = init.1;

    let mut computer = Computer::new(copy);

    computer.run()
}
