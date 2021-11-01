use crate::utils;
use std::fmt;

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

#[derive(Debug, Clone, Copy)]
enum ProgramError {
    InvalidOpcode(i32),
    InvalidMode(i32, u32),
    InvalidMemory(i32),
}

impl fmt::Display for ProgramError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match self {
            ProgramError::InvalidOpcode(i) => format!("Invalid opcode {}", i),
            ProgramError::InvalidMode(mode, i) => format!("Invalid mode {} at position {}", mode, i),
            ProgramError::InvalidMemory(i) => format!("Out of range memory address {}", i),
        };
        write!(f, "{}", msg)
    }
}

pub struct ExecutionError(usize, ProgramError);

pub type EResult<T> = Result<T, ExecutionError>;

impl fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Execution error at instruction {:#x}: {}", self.0, self.1)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Mode {
    Positional,
    Immediate,
}

impl Mode {
    fn of(modes: i32, pos: u32) -> PResult<Self> {
        let i = (modes / 10_i32.pow(pos)) % 10;
        match i {
            0 => Ok(Mode::Positional),
            1 => Ok(Mode::Immediate),
            _ => Err(ProgramError::InvalidMode(modes, pos)),
        }
    }

    fn resolve(&self, i:  i32, memory: &Vec<i32>) -> PResult<i32>  {
        match self {
            Mode::Positional => {
                if (i < 0) || (i as usize) >= memory.len() {
                    Err(ProgramError::InvalidMemory(i))
                } else {
                    Ok(memory[i as usize])
                }
            }
            Mode::Immediate => Ok(i)
        }
    }
}

type PResult<T> = Result<T, ProgramError>;

struct Arg(Mode, PResult<i32>);

impl Arg {
    fn resolve(&self, mem: &Vec<i32>) -> PResult<i32> {
        self.0.resolve(self.1?, mem)
    }
}

type Args = (
    i32,
    (Arg, Arg, Arg)
);
pub struct Computer {
    position: usize,
    program: Vec<i32>,
}

impl Computer {
    // TODO return Result
    pub fn new(program: Vec<i32>) -> Self {
        Computer {
            position: 0,
            program,
        }
    }

    pub fn run(&mut self) -> EResult<i32> {
        loop {
            let args = self.get_args();
            if let Err(e) = args {
                return Err(ExecutionError(self.position, e));
            }

            let args = args.unwrap();

            let opcode = args.0;

            if ops::HALT == opcode {
                return Ok(self.program[0])
            }

            let opargs = args.1;
            if let Err(err) = self.apply_opcode(opcode, opargs) {
                return Err(ExecutionError(self.position, err) )
            }

            match opcode {
                ops::ADD | ops::MUL => self.position += 4,
                _ => panic!("Invalid opcode was not validated: {}", opcode),
            }
        }
    }

    fn apply_opcode(&mut self, opcode: i32, opargs: (Arg, Arg, Arg)) -> PResult<()> {
        let (a, b, c) = opargs;

        match opcode {
            ops::ADD => self.set_mem_arg(c,self.get_mem_arg(a)? + self.get_mem_arg(b)?),
            ops::MUL => self.set_mem_arg(c, self.get_mem_arg(a)? * self.get_mem_arg(b)?),
            _ => Err(ProgramError::InvalidOpcode(opcode)),
        }
    }

    fn get_args(&self) -> PResult<Args> {
        let raw_op = self.safe_vec_get(0)?;

        let opcode = raw_op % 100;
        // TODO validate opcode

        let modes =  raw_op / 100;

        let arg_modes = (
            Mode::of(modes, 0)?,
            Mode::of(modes, 1)?,
            Mode::of(modes, 2)?,
        );

        Ok((
            opcode,
            (
                self.as_arg(arg_modes.0, 1),
                self.as_arg(arg_modes.1, 2),
                self.as_arg(arg_modes.2, 3),
            )
        ))
    }

    fn as_arg(&self, mode: Mode, offset: usize) -> Arg {
        let location = self.position + offset;
        Arg(mode, if self.mem_safe(location) {
            Ok(location as i32)
        } else {
            Err(ProgramError::InvalidMemory(location as i32))
        })
    }

    fn mem_safe(&self, offset: usize) -> bool {
        self.program.len() > offset
    }

    fn safe_vec_get(&self, offset: usize) -> PResult<i32> {
        let location = self.position + offset;

        if !self.mem_safe(location) {
            Err(ProgramError::InvalidMemory(location as i32))
        } else {
            Ok(self.program[location])
        }
    }

    fn get_mem(&self, offset: i32) -> i32 {
        self.program[offset as usize]
    }

    fn get_mem_arg(&self, arg: Arg) -> PResult<i32> {
        let location = arg.resolve(&self.program)?;

        if !self.mem_safe(location as usize) {
            return Err(ProgramError::InvalidMemory(location as i32))
        }

        Ok(self.get_mem(location))
    }

    fn set_mem(&mut self, offset: i32, value: i32) {
        self.program[offset as usize] = value
    }

    fn set_mem_arg(&mut self, arg: Arg, val: i32) -> PResult<()> {
        Ok(self.set_mem(arg.resolve(&self.program)?, val))
    }
}
