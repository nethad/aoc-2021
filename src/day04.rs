type Board = Vec<Vec<isize>>;

pub fn determine_bingo_column(numbers: &Vec<isize>, board: &Board) -> Option<Vec<isize>> {
    for pos in 0..board.first().unwrap().len() {
        let mut column = board.iter().map(|row| row[pos]);
        if column.all(|number| numbers.contains(&number)) {
            return Some(board.iter().map(|row| row[pos]).collect());
        }
    }
    None
}

pub fn determine_bingo_row<'a>(numbers: &Vec<isize>, board: &'a Board) -> Option<&'a Vec<isize>> {
    board
        .iter()
        .any(|line| line.iter().all(|number| numbers.contains(number)));

    for row in board {
        if row.iter().all(|number| numbers.contains(number)) {
            return Some(row);
        }
    }
    None
}

pub fn board_sum(board: &Board) -> isize {
    board.iter().map(|row| row.iter().sum::<isize>()).sum()
}

pub fn determine_bingo(numbers: &Vec<isize>, board: &Board) -> Option<(isize, isize)> {
    let rows = determine_bingo_row(numbers, board);
    let columns = determine_bingo_column(numbers, board);

    if rows.is_some() || columns.is_some() {
        let board_sum = board_sum(board);
        let numbers_sum = numbers.iter().sum::<isize>();
        let last_number = numbers.iter().last().unwrap();
        Some((*last_number, board_sum - numbers_sum))
    } else {
        None
    }
}

pub fn determine_bingo_boards(numbers: &Vec<isize>, boards: &Vec<Board>) -> Option<(isize, isize)> {
    for board in boards {
        let sum = determine_bingo(numbers, board);
        if sum.is_some() {
            return sum;
        }
    }
    None
}

pub fn parse_bingo_numbers(line: &str) -> Vec<isize> {
    line.trim()
        .split(",")
        .filter_map(|number| number.trim().parse::<isize>().ok())
        .collect()
}

pub fn parse_boards(lines: &Vec<String>) -> Vec<Board> {
    let mut boards: Vec<Board> = vec![];
    let mut board: Board = vec![];
    for line in lines.iter().skip(1) {
        if line.trim().is_empty() {
            if !board.is_empty() {
                boards.push(board);
            }
            board = vec![];
        } else {
            board.push(
                line.trim()
                    .split(" ")
                    .filter_map(|n| n.trim().parse::<isize>().ok())
                    .collect(),
            );
        }
    }
    if !board.is_empty() {
        boards.push(board);
    }
    boards
}

pub fn parse_input(lines: &Vec<String>) -> (Vec<isize>, Vec<Board>) {
    let numbers = parse_bingo_numbers(lines.first().unwrap());
    let boards = parse_boards(lines);

    (numbers, boards)
}

pub fn day04(lines: &Vec<String>) -> isize {
    let (numbers, boards) = parse_input(lines);

    for index in 1..numbers.len() {
        if let Some((last_number, board_sum)) =
            determine_bingo_boards(&numbers[0..index].to_vec(), &boards)
        {
            return last_number * board_sum;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        let fixture = {
            "
                          7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

                          22 13 17 11  0
                           8  2 23  4 24
                          21  9 14 16  7
                           6 10  3 18  5
                           1 12 20 15 19

                           3 15  0  2 22
                           9 18 13 17  5
                          19  8  7 25 23
                          20 11 10 24  4
                          14 21 16 12  6

                          14 21 17 24  4
                          10 16 15  9 19
                          18  8 23 26 20
                          22 11 13  6  5
                           2  0 12  3  7
"
        };

        let input: Vec<String> = fixture.trim().lines().map(|s| s.to_string()).collect();

        assert_eq!(
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1",
            input.iter().nth(0).unwrap()
        );

        let (numbers, boards) = parse_input(&input);

        assert_eq!(27, numbers.len());
        assert_eq!(
            vec![
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1
            ],
            numbers
        );
        assert_eq!(3, boards.len());
        assert_eq!(
            &vec![22, 13, 17, 11, 0],
            boards.first().unwrap().first().unwrap()
        );
        assert_eq!(
            &vec![1, 12, 20, 15, 19],
            boards.first().unwrap().last().unwrap()
        );
        assert_eq!(
            &vec![14, 21, 17, 24, 4],
            boards.last().unwrap().first().unwrap()
        );
        assert_eq!(
            &vec![2, 0, 12, 3, 7],
            boards.last().unwrap().last().unwrap()
        );
    }

    #[test]
    fn test_determine_bingo() {
        let board: Board = vec![
            vec![22, 13, 17, 11, 0],
            vec![8, 2, 23, 4, 24],
            vec![21, 9, 14, 16, 7],
            vec![6, 10, 3, 18, 5],
            vec![1, 12, 20, 15, 19],
        ];

        assert_eq!(None, determine_bingo(&vec![22, 13, 17, 11], &board));

        assert_eq!(
            Some((0, 237)),
            determine_bingo(&vec![22, 13, 17, 11, 0], &board)
        );

        assert_eq!(
            Some((19, 233)),
            determine_bingo(&vec![1, 12, 20, 15, 19], &board)
        );

        assert_eq!(None, determine_bingo(&vec![22, 8, 21, 6], &board));
        assert_eq!(
            Some((1, 242)),
            determine_bingo(&vec![22, 8, 21, 6, 1], &board)
        );
        assert_eq!(
            Some((19, 245)),
            determine_bingo(&vec![0, 24, 7, 5, 19], &board)
        );
    }

    #[test]
    fn test_play_game() {
        let fixture = {
            "
                          7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

                          22 13 17 11  0
                           8  2 23  4 24
                          21  9 14 16  7
                           6 10  3 18  5
                           1 12 20 15 19

                           3 15  0  2 22
                           9 18 13 17  5
                          19  8  7 25 23
                          20 11 10 24  4
                          14 21 16 12  6

                          14 21 17 24  4
                          10 16 15  9 19
                          18  8 23 26 20
                          22 11 13  6  5
                           2  0 12  3  7
"
        };

        let input: Vec<String> = fixture.trim().lines().map(|s| s.to_string()).collect();
        let (numbers, boards) = parse_input(&input);
        let result = day04(&input);

        assert_eq!(
            None,
            determine_bingo_boards(&numbers[0..11].to_vec(), &boards)
        );
        assert_eq!(
            Some((24, 188)),
            determine_bingo_boards(&numbers[0..12].to_vec(), &boards)
        );

        assert_eq!(4512, result);
    }
}
