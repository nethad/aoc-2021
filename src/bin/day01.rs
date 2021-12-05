use aoc_2021::common::read_lines;

fn main() {
    let lines = read_lines("inputs/day01.txt")
        .iter()
        .filter_map(|line| line.parse::<usize>().ok())
        .collect::<Vec<usize>>();
    let increase_count = lines.windows(2).filter(|pair| pair[1] > pair[0]).count();

    println!("Q: How many measurements are larger than the previous measurement?");
    println!("A: {}", increase_count);

    let group_sums = lines
        .windows(3)
        .map(|group| group[0] + group[1] + group[2])
        .collect::<Vec<_>>();

    let increase_count = group_sums
        .windows(2)
        .filter(|pair| pair[1] > pair[0])
        .count();

    println!("Q: How many sums are larger than the previous sum?");
    println!("A: {}", increase_count)
}
