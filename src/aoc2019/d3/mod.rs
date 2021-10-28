use crate::utils;
use std::convert::From;
use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::RangeInclusive;

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

        let wire_1 = &paths[1];
        let wire_2 = &paths[0];

        let wire_1_edges = get_edges(wire_1);
        let intersections = get_intersections(wire_2, wire_1_edges.0, wire_1_edges.1);

        println!("Intersections {:?}", intersections);

        let nearest_intersection = intersections.into_iter()
            .reduce(|last, next| {
                if last.0.abs() + last.1.abs() < next.0.abs() + next.1.abs() {
                    last
                } else {
                    next
                }
            });

            println!("Nearest {:?}", nearest_intersection);
}

fn get_edges(path: &Vec<Travel>) -> (HashMap<i32, HashSet<i32>>, HashMap<i32, HashSet<i32>>) {

    let mut verticals: HashMap<i32, HashSet<i32>> =  HashMap::new();
    let mut horizontals: HashMap<i32, HashSet<i32>> =  HashMap::new();
    let mut current = (0i32, 0i32);

    for t in path {
        let (x, y) = current;

        match t {
            Travel::Right(d) => current.0 += *d as i32,
            Travel::Left(d) => current.0 -= *d as i32,
            Travel::Up(d) => current.1 += *d as i32,
            Travel::Down(d) => current.1 -= *d as i32,
        }

        match t {
            Travel::Right(_)|Travel::Left(_) => {
                for i in abs_range_inclusive(x, current.0) {
                    let crossings  = verticals.entry(i).or_insert_with(|| HashSet::new());
                    crossings.insert(y);
                }
            },
            Travel::Up(_)|Travel::Down(_) => {
                for i in abs_range_inclusive(y, current.1) {
                    let crossings  = horizontals.entry(i).or_insert_with(|| HashSet::new());
                    crossings.insert(x);
                }
            },
        }
    }

    return (horizontals, verticals);
}

fn get_intersections(
    path: &Vec<Travel>,
    horizontals: HashMap<i32, HashSet<i32>>,
    verticals:HashMap<i32, HashSet<i32>>
) -> Vec<(i32, i32)> {
    let mut intersections = vec![];
    let mut current = (0i32, 0i32);

    for t in path {
        let (last_x, last_y) = current;

        match t {
            Travel::Right(d) => current.0 += *d as i32,
            Travel::Left(d) => current.0 -= *d as i32,
            Travel::Up(d) => current.1 += *d as i32,
            Travel::Down(d) => current.1 -= *d as i32,
        }

        match t {
            Travel::Right(_)|Travel::Left(_) => {
                if let Some(crossings) = horizontals.get(&last_y) {
                    abs_range_inclusive(last_x, current.0)
                        .filter(|x| crossings.contains(x))
                        .for_each(|x| intersections.push((x, last_y)))

                }
            },
            Travel::Up(_)|Travel::Down(_) => {
                if let Some(crossings) = verticals.get(&last_x) {
                    abs_range_inclusive(last_y, current.1)
                        .filter(|y| crossings.contains(y))
                        .for_each(|y| intersections.push((last_x, y)))

                }
            },
        }
    }

    intersections
}

fn abs_range_inclusive(i1: i32, i2: i32)-> RangeInclusive<i32> {
    if i1 > i2 {
        i2..=i1
    } else {
        i1..=i2
    }
}