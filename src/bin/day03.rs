use aoc_2021::common::read_lines;
use aoc_2021::day03::day03;

fn main() {
    let lines = read_lines("inputs/day03.txt");
    println!("{}", day03(&lines));
}
