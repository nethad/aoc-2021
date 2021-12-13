use aoc_2021::common::read_lines;
use aoc_2021::day04::day04;

fn main() {
    let lines = read_lines("inputs/day04.txt");
    let result = day04(&lines);

    println!("Q: What will your final score be if you choose that board?");
    println!("A: {}", result);
}
