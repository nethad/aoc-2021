pub fn epsilon_at_position(lines: &Vec<String>, position: usize) -> char {
    if gamma_at_position(lines, position) == '0' {
        '1'
    } else {
        '0'
    }
}

pub fn gamma_at_position(lines: &Vec<String>, position: usize) -> char {
    let (zeros, ones): (Vec<&String>, Vec<&String>) = lines
        .iter()
        .partition(|line| line.chars().nth(position) == Some('0'));
    if zeros.len() > ones.len() {
        '0'
    } else {
        '1'
    }
}

pub fn rate_calc(lines: &Vec<String>, func: fn(&Vec<String>, usize) -> char) -> isize {
    let mut rate_binary_str = String::with_capacity(5);
    for pos in 0..lines.first().unwrap().len() {
        rate_binary_str.push(func(lines, pos))
    }
    isize::from_str_radix(&rate_binary_str, 2).unwrap()
}

pub fn gamma_rate(lines: &Vec<String>) -> isize {
    rate_calc(lines, gamma_at_position)
}

pub fn epsilon_rate(lines: &Vec<String>) -> isize {
    rate_calc(lines, epsilon_at_position)
}

pub fn power_consumption(lines: &Vec<String>) -> isize {
    gamma_rate(lines) * epsilon_rate(lines)
}

pub fn day03(lines: &Vec<String>) -> isize {
    power_consumption(lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        let input: Vec<&str> = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];
        let input: Vec<String> = input.iter().map(|e| e.to_string()).collect();

        assert_eq!('1', gamma_at_position(&input, 0));
        assert_eq!('0', gamma_at_position(&input, 1));
        assert_eq!('1', gamma_at_position(&input, 2));
        assert_eq!('1', gamma_at_position(&input, 3));
        assert_eq!('0', gamma_at_position(&input, 4));

        assert_eq!(22, gamma_rate(&input));

        assert_eq!('0', epsilon_at_position(&input, 0));
        assert_eq!('1', epsilon_at_position(&input, 1));
        assert_eq!('0', epsilon_at_position(&input, 2));
        assert_eq!('0', epsilon_at_position(&input, 3));
        assert_eq!('1', epsilon_at_position(&input, 4));

        assert_eq!(9, epsilon_rate(&input));

        assert_eq!(198, power_consumption(&input));

        assert_eq!(198, day03(&input));
    }
}
