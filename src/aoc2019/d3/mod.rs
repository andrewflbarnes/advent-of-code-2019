use crate::utils;
use std::collections::HashMap;
use std::collections::HashSet;
use std::convert::From;
use std::iter::Iterator;

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
            _ => panic!("Invalid direction: {}", direction),
        }
    }
}

type Tracker = HashSet<(i32, i32)>;

pub fn solve(input1: String, _: String, _: &[String]) {
    let paths: Vec<Vec<Travel>> = utils::read_file_lines(&input1)
        .into_iter()
        .map(|l| l.split(",").map(|d| d.into()).collect())
        .collect();

    let path_1 = &paths[1];
    let path_2 = &paths[0];

    let path_1_edges = get_edges(path_1);
    let intersections = get_intersections(path_2, path_1_edges);

    println!("Intersections {:?}", intersections);

    let nearest_intersection = intersections
        .iter()
        .reduce(|last, next| {
            if last.0.abs() + last.1.abs() < next.0.abs() + next.1.abs() {
                last
            } else {
                next
            }
        })
        .unwrap();

    println!(
        "Nearest {:?} with distance {}",
        nearest_intersection,
        nearest_intersection.0.abs() + nearest_intersection.1.abs()
    );

    let it = get_time_distances(path_1, &intersections);
    let it2 = get_time_distances(path_2, &intersections);

    let fastest_intersection = it
        .iter()
        .map(|(pos, distance)| (pos, it2.get(pos).unwrap() + distance))
        .reduce(|last, next| if last.1 > next.1 { next } else { last })
        .unwrap();

    println!(
        "Fastest {:?} with distance {}",
        fastest_intersection.0, fastest_intersection.1
    );
}

fn update_location(mut current: (i32, i32), t: &Travel) -> (i32, i32) {
    match t {
        Travel::Right(d) => current.0 += *d as i32,
        Travel::Left(d) => current.0 -= *d as i32,
        Travel::Up(d) => current.1 += *d as i32,
        Travel::Down(d) => current.1 -= *d as i32,
    }
    current
}

fn get_edges(path: &Vec<Travel>) -> (Tracker, Tracker) {
    let (h, v, ..) = get_all_edge_data(path, &vec![]);
    (h, v)
}

fn get_time_distances(
    path: &Vec<Travel>,
    intersections: &Vec<(i32, i32)>,
) -> HashMap<(i32, i32), i32> {
    let (_, _, time_distances) = get_all_edge_data(path, intersections);
    time_distances
}

fn get_all_edge_data(
    path: &Vec<Travel>,
    intersections: &Vec<(i32, i32)>,
) -> (Tracker, Tracker, HashMap<(i32, i32), i32>) {
    let mut verticals: Tracker = HashSet::new();
    let mut horizontals: Tracker = HashSet::new();
    let mut current = (0i32, 0i32);
    let mut distance = 0;
    let mut time_distances = HashMap::new();

    let mut check_each_total_distance = |path_distance: i32, pos: (i32, i32)| {
        if intersections.contains(&pos) {
            time_distances.entry(pos).or_insert_with(|| path_distance);
        }
    };

    for t in path {
        let (x, y) = current;

        current = update_location(current, t);

        let mut check_each = |path_distance: i32, pos: (i32, i32)| {
            return check_each_total_distance(distance + path_distance, pos);
        };

        match t {
            Travel::Right(_) | Travel::Left(_) => {
                update_crossings(&mut verticals, x, current.0, |i| (i, y), &mut check_each);
            }
            Travel::Up(_) | Travel::Down(_) => {
                update_crossings(&mut horizontals, y, current.1, |i| (x, i), &mut check_each);
            }
        }

        distance += (x - current.0).abs() + (y - current.1).abs();
    }

    return (horizontals, verticals, time_distances);
}

fn update_crossings<F: FnMut(i32, (i32, i32)), P: Fn(i32) -> (i32, i32)>(
    track: &mut Tracker,
    from: i32,
    to: i32,
    to_pos: P,
    check_each: &mut F,
) {
    let mut path_distance = 0;
    let mut range = abs_range_inclusive(from, to).skip(1).peekable();
    // don't care about the corners so skip the first and last in the range
    while let Some(i) = range.next() {
        let pos = to_pos(i);
        if range.peek().is_none() {
            break;
        }
        path_distance += 1;
        track.insert(pos);
        check_each(path_distance, pos);
    }
}

fn get_intersections(path: &Vec<Travel>, trackers: (Tracker, Tracker)) -> Vec<(i32, i32)> {
    let (horizontals, verticals) = trackers;
    let mut intersections = vec![];
    let mut current = (0i32, 0i32);

    for t in path {
        let (last_x, last_y) = current;

        current = update_location(current, t);

        let (crossings, from, to, to_pos): (&Tracker, i32, i32, Box<dyn Fn(i32) -> (i32, i32)>) =
            match t {
                Travel::Right(_) | Travel::Left(_) => {
                    (&horizontals, last_x, current.0, Box::new(|x| (x, last_y)))
                }
                Travel::Up(_) | Travel::Down(_) => {
                    (&verticals, last_y, current.1, Box::new(|y| (last_x, y)))
                }
            };

        abs_range_inclusive(from, to)
            .map(to_pos)
            .filter(|pos| crossings.contains(pos))
            .for_each(|pos| {
                if !intersections.contains(&pos) {
                    intersections.push(pos);
                }
            });
    }

    intersections
}

fn abs_range_inclusive(i1: i32, i2: i32) -> Box<dyn Iterator<Item = i32>> {
    if i1 > i2 {
        Box::new((i2..=i1).rev())
    } else {
        Box::new(i1..=i2)
    }
}
