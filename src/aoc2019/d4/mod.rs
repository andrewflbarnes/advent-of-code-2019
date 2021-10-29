use crate::utils;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub fn solve(input1: String, _: String, _: &[String]) {
    let range: Vec<i32> = utils::read_file_lines(&input1)
        .into_iter()
        .flat_map(|l| l.split("-").map(str::to_string).collect::<Vec<_>>())
        .filter_map(|l| l.parse::<i32>().ok())
        .collect();

    let count = get_valid_codes(range[0], range[1]);

    println!("{} valid codes", count.0);
    println!("{} valid restricted codes", count.1);
}

fn get_valid_codes(from: i32, to: i32) -> (i32, i32) {
    let mut count = (0, 0);

    for code in from..=to {
        let d = extract_digits(code);
        if is_ascending(d) {
            if is_consecutive(d) {
                count.0 += 1;
            }
            if is_consecutive_restricted(d) {
                count.1 += 1;
            }
        }
    }

    count
}

type Digits = (i32, i32, i32, i32, i32, i32);

fn extract_digits(code: i32) -> Digits {
    (
        extract_digit(code, 0),
        extract_digit(code, 1),
        extract_digit(code, 2),
        extract_digit(code, 3),
        extract_digit(code, 4),
        extract_digit(code, 5),
    )
}

fn extract_digit(code: i32, digit: u32) -> i32 {
    code / (100000 / 10_i32.pow(digit)) % 10
}

fn is_ascending(d: Digits) -> bool {
    d.0 <= d.1 && d.1 <= d.2 && d.2 <= d.3 && d.3 <= d.4 && d.4 <= d.5
}

fn is_consecutive(d: Digits) -> bool {
    d.0 == d.1 || d.1 == d.2 || d.2 == d.3 || d.3 == d.4 || d.4 == d.5
}

fn is_consecutive_restricted(d: Digits) -> bool {
    let mut map: HashMap<i32, i32> = HashMap::new();
    update_entry(&mut map, d.0);
    update_entry(&mut map, d.1);
    update_entry(&mut map, d.2);
    update_entry(&mut map, d.3);
    update_entry(&mut map, d.4);
    update_entry(&mut map, d.5);

    map.iter().any(|(_, v)| *v == 2)
}

fn update_entry(map: &mut HashMap<i32, i32>, key: i32) {
    let v = match map.entry(key) {
        Entry::Occupied(o) => o.into_mut(),
        Entry::Vacant(v) => v.insert(0),
    };
    *v += 1;
}
