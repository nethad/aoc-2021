use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_lines(path: &str) -> Vec<String> {
    let file = File::open(path).expect("Could not open file!");
    BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .collect::<Vec<_>>()
}
