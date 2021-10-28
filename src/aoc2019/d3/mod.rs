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

        let nearest_intersection = intersections.iter()
            .reduce(|last, next| {
                if last.0.abs() + last.1.abs() < next.0.abs() + next.1.abs() {
                    last
                } else {
                    next
                }
            });

        println!("Nearest {:?}", nearest_intersection);

        let it = get_time_distances(wire_1, &intersections);
        let it2 = get_time_distances(wire_2, &intersections);

        it.iter()
            .for_each(|(pos, distance)| {
                println!("Total distance to {:?}: {}", pos, it2.get(pos).unwrap() + distance);
            });
}

fn get_edges(path: &Vec<Travel>) -> (HashMap<i32, HashSet<i32>>, HashMap<i32, HashSet<i32>>) {
    let (h, v, ..) = get_all_edge_data(path, &vec![]);
    (h, v)
}

fn get_time_distances(path: &Vec<Travel>, intersections: &Vec<(i32, i32)>) -> HashMap<(i32, i32), i32> {
    let (_, _, time_distances) = get_all_edge_data(path, intersections);
    time_distances
}

fn get_all_edge_data(path: &Vec<Travel>, intersections: &Vec<(i32, i32)>) -> (
    HashMap<i32, HashSet<i32>>,
    HashMap<i32, HashSet<i32>>,
    HashMap<(i32, i32), i32>
) {

    let mut verticals: HashMap<i32, HashSet<i32>> =  HashMap::new();
    let mut horizontals: HashMap<i32, HashSet<i32>> =  HashMap::new();
    let mut current = (0i32, 0i32);
    let mut distance = 0;
    let mut time_distances = HashMap::new();

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
                update_crossings(y, &mut verticals, x + 1, current.0 - 1, |path_distance: i32, on: i32| {
                    let pos = (on, y);
                    if on == -1435 && y == 2474 {
                        println!("FOUND! {} + {}", distance, path_distance);
                    }
                    if intersections.contains(&pos) {
                        time_distances.entry(pos).or_insert(distance + path_distance);
                        println!("Found intersection after {} at {:?}", distance + path_distance, (on, y));
                    }
                });
            },
            Travel::Up(_)|Travel::Down(_) => {
                update_crossings(x, &mut horizontals, y + 1, current.1 - 1, |path_distance: i32, on: i32| {
                    let pos = (x, on);
                    if x == -1435 && on == 2474 {
                        println!("FOUND! {} + {}", distance, path_distance);
                    }
                    if intersections.contains(&pos) {
                        time_distances.entry(pos).or_insert(distance + path_distance);
                        println!("Found intersection after {} at {:?}", distance + path_distance, (x, on));
                    }
                });
            },
        }

        distance += (x - current.0).abs() + (y - current.1).abs();
    }

    return (horizontals, verticals, time_distances);
}

fn update_crossings<F: FnMut(i32, i32)>(at: i32, track: &mut HashMap<i32, HashSet<i32>>, from: i32, to: i32,
    mut check_each: F) {

    for i in abs_range_inclusive(from, to) {
        track.entry(i)
            .or_insert_with(|| HashSet::new())
            .insert(at);
        check_each((from.max(to) - i).abs(), i);
    }
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