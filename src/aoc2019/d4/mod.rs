use crate::utils;

pub fn solve(input1: String, _: String, _: &[String]) {
    let range: Vec<i32> = utils::read_file_lines(&input1)
        .into_iter()
        .flat_map(|l| l.split("-").map(str::to_string).collect::<Vec<_>>())
        .filter_map(|l| l.parse::<i32>().ok())
        .collect();

    let mut count = 0;
    for i in range[0]..=range[1] {
        if valid_code(i) {
            count += 1;
        }
    }

    println!("{} valid codes", count);
}

fn valid_code(code: i32) -> bool {
    false
}
