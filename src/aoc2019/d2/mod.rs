use crate::utils;
mod vm;
use vm::Computer;

pub fn solve(input1: String, _: String, _: &[String]) {
    let mut program: Vec<i32> = utils::read_file_lines(&input1)
        .into_iter()
        .flat_map(|l| l.split(",").map(str::to_owned).collect::<Vec<_>>())
        .filter_map(|i| i.to_owned().parse::<i32>().ok())
        .collect();

    program[1] = 12;
    program[2] = 2;

    let mut computer = Computer::new(program);

    computer.run();
        
}