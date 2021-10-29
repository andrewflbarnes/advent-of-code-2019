use crate::utils;

pub fn read_program(input: String) -> Vec<i32> {
    utils::read_file_lines(&input)
        .into_iter()
        .flat_map(|l| l.split(",").map(str::to_owned).collect::<Vec<_>>())
        .filter_map(|i| i.to_owned().parse::<i32>().ok())
        .collect::<Vec<_>>()
}

mod ops {
    pub const ADD: i32 = 1;
    pub const MUL: i32 = 2;
    pub const HALT: i32 = 99;
}

pub struct Computer {
    position: usize,
    program: Vec<i32>,
}

impl Computer {
    pub fn new(program: Vec<i32>) -> Self {
        Computer {
            position: 0,
            program,
        }
    }

    pub fn run(&mut self) -> i32 {
        loop {
            let (opcode, a, b, c) = self.get_args();

            match opcode {
                ops::ADD | ops::MUL => {
                    if !self.mem_safe(a) || !self.mem_safe(b) || !self.mem_safe(c) {
                        return -1;
                    }
                }
                _ => {}
            }

            match opcode {
                ops::ADD => self.set_mem(c, self.get_mem(a) + self.get_mem(b)),
                ops::MUL => self.set_mem(c, self.get_mem(a) * self.get_mem(b)),
                ops::HALT => return self.program[0],
                _ => panic!("Invalid opcode found: {}", opcode),
            };

            match opcode {
                ops::ADD | ops::MUL => self.position += 4,
                _ => panic!("Invalid opcode found: {}", opcode),
            }
        }
    }

    fn get_args(&self) -> (i32, i32, i32, i32) {
        return (
            self.safe_vec_get(0),
            self.safe_vec_get(1),
            self.safe_vec_get(2),
            self.safe_vec_get(3),
        );
    }

    fn mem_safe(&self, offset: i32) -> bool {
        self.program.len() > (offset as usize)
    }

    fn safe_vec_get(&self, offset: usize) -> i32 {
        let location = self.position + offset;

        if !self.mem_safe(location as i32) {
            -1
        } else {
            self.program[location]
        }
    }

    fn get_mem(&self, offset: i32) -> i32 {
        self.program[offset as usize]
    }

    fn set_mem(&mut self, offset: i32, value: i32) {
        self.program[offset as usize] = value
    }
}
