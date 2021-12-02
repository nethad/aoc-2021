use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("src/bin/inputs/day02.txt").expect("Could not open file!");
    let lines = BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .collect::<Vec<_>>();

    let mut horizontal_position = 0;
    let mut depth = 0;

    for step in &lines {
        let parts = step.split(" ").collect::<Vec<_>>();
        let direction = parts[0];
        let unit = parts[1].parse::<usize>().ok();
        match (direction, unit) {
            ("forward", Some(unit)) => horizontal_position += unit,
            ("up", Some(unit)) => depth -= unit,
            ("down", Some(unit)) => depth += unit,
            _ => println!("UNKNOWN!"),
        }
    }

    let result = horizontal_position * depth;

    println!(
        "Q: What do you get if you multiply your final horizontal position by your final depth?"
    );
    println!("A: {}", result);

    horizontal_position = 0;
    depth = 0;
    let mut aim = 0;

    for step in &lines {
        let parts = step.split(" ").collect::<Vec<_>>();
        let direction = parts[0];
        let unit = parts[1].parse::<usize>().ok();
        match (direction, unit) {
            ("forward", Some(unit)) => {
                horizontal_position += unit;
                depth += aim * unit;
            }
            ("up", Some(unit)) => aim -= unit,
            ("down", Some(unit)) => aim += unit,
            _ => println!("UNKNOWN!"),
        }
    }

    let result = horizontal_position * depth;
    println!(
        "Q: What do you get if you multiply your final horizontal position by your final depth?"
    );
    println!("A: {}", result);
}
