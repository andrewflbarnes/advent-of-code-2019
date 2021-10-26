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

    pub fn run(&mut self) {
        loop {
            let (opcode, a, b , c) = self.get_args();
            match opcode {
                ops::ADD => self.set_mem(c, self.get_mem(a) + self.get_mem(b)),
                ops::MUL => self.set_mem(c, self.get_mem(a) * self.get_mem(b)),
                ops::HALT => {
                    println!("Program execution ended, final value at position 0: {}", self.program[0]);
                    return
                },
                _ => panic!("Invalid opcode found: {}", opcode)
            };

            match opcode {
                ops::ADD|ops::MUL => self.position += 4,
                _ => panic!("Invalid opcode found: {}", opcode)
            }
        }
    }

    fn get_args(&self) -> (i32, i32, i32, i32) {
        return (self.safe_vec_get(0), self.safe_vec_get(1), self.safe_vec_get(2), self.safe_vec_get(3))
    }
    
    fn safe_vec_get(&self, offset: usize) -> i32 {
        let location = self.position + offset;

        if self.program.len() < location {
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