use crate::vm;
use crate::vm::{Computer, EResult};

pub fn solve(input1: String, _: String, _: &[String]) {
    let program: Vec<i32> = vm::read_program(input1);

    match run_with(&program, (12, 2)) {
        Ok(res) => println!("Program execution ended with result: {}", res),
        Err(e) => panic!("{}", e),
    }

    for noun in 0..=255 {
        for verb in 0..=255 {
            match run_with(&program, (noun, verb)) {
                Ok(19690720) => println!(
                    "Program execution ended with program code: {}",
                    noun * 100 + verb
                ),
                _ => {}
            }
        }
    }
}

fn run_with(program: &Vec<i32>, init: (i32, i32)) -> EResult<i32> {
    let mut copy = program.clone();
    copy[1] = init.0;
    copy[2] = init.1;

    let mut computer = Computer::new(copy);

    computer.run()
}
