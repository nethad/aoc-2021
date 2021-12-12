use aoc_2021::common::read_lines;
use aoc_2021::day04::day04;
use aoc_2021::day04::parse_input;

fn main() {
    let lines = read_lines("inputs/day04.txt");

    let (numbers, boards) = parse_input(&lines);
    let result = day04(&lines);

    println!("numbers: {:?}", numbers);
    println!("boards: {:?}", boards);

    println!("Q: What will your final score be if you choose that board?");
    println!("A: {}", result);
}
