use std::fs;

pub fn read_file_lines<'a>(path: &str) -> Vec<String> {
    fs::read_to_string(path)
        .expect(&format!("Unable to read file {}", path))
        .lines()
        .map(|l| l.to_string())
        .collect::<Vec<String>>()
}
