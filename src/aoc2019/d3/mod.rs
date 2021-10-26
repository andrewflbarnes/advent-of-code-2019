use crate::utils;
use std::convert::From;

#[derive(Debug)]
enum Travel {
    Up(u16),
    Down(u16),
    Left(u16),
    Right(u16),
}

impl From<&str> for Travel {
    fn from(str: &str) -> Self {
        let distance = str[1..].parse::<u16>().unwrap();
        let direction = str.chars().nth(0).unwrap();
        match direction {
            'U' => Travel::Up(distance),
            'D' => Travel::Down(distance),
            'L' => Travel::Left(distance),
            'R' => Travel::Right(distance),
            _ => panic!("Invalid direction: {}", direction)
        }
    }
}

pub fn solve(input1: String, _: String, _: &[String]) {
    let paths: Vec<Vec<Travel>> = utils::read_file_lines(&input1)
        .into_iter()
        .map(|l| l.split(",").map(|d| d.into()).collect())
        .collect();
    
        println!("{:?}", paths)
}
