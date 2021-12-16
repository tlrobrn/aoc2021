#![warn(clippy::all, clippy::pedantic)]
use std::collections::HashMap;
use std::str::FromStr;

fn main() {
    let input_lines: Vec<String> = aoc2021::input_lines().collect();
    let draws: Vec<u64> = input_lines[0]
        .trim()
        .split(',')
        .map(|n| n.parse::<u64>().unwrap())
        .collect();

    let mut boards = build_boards(
        &input_lines[1..]
            .chunks(6)
            .map(|bar| bar[1..].join("\n").trim().to_string())
            .collect::<Vec<String>>(),
    );

    let mut boards_cloned = boards.clone();

    let result = part1(&mut boards, &draws);
    println!("Part 1: {}", result);
    let result = part2(&mut boards_cloned, &draws);
    println!("Part 2: {}", result);
}

fn build_boards(board_strings: &[String]) -> Vec<Board> {
    board_strings
        .iter()
        .map(|board| Board::from_str(board).unwrap())
        .collect()
}

const BINGO_SIZE: usize = 5;

#[derive(Default, Clone)]
struct Board {
    numbers: HashMap<u64, (usize, usize)>,
    row_marks: [u8; BINGO_SIZE],
    col_marks: [u8; BINGO_SIZE],
    unmarked_sum: u64,
}

impl Board {
    pub fn new(input_rows: &[Vec<u64>]) -> Self {
        let mut numbers = HashMap::with_capacity(BINGO_SIZE * BINGO_SIZE);
        let mut unmarked_sum = 0;
        for (row_idx, row) in input_rows.iter().enumerate().take(BINGO_SIZE) {
            for (col_idx, number) in row.iter().enumerate().take(BINGO_SIZE) {
                numbers.insert(*number, (row_idx, col_idx));
                unmarked_sum += number;
            }
        }
        Self {
            numbers,
            row_marks: Default::default(),
            col_marks: Default::default(),
            unmarked_sum,
        }
    }

    pub fn mark(&mut self, draw: u64) -> Option<u64> {
        if let Some((row, col)) = self.numbers.get(&draw) {
            self.row_marks[*row] += 1;
            self.col_marks[*col] += 1;
            self.unmarked_sum -= draw;

            let row_count = self.row_marks[*row] as usize;
            let col_count = self.col_marks[*col] as usize;

            if row_count == BINGO_SIZE || col_count == BINGO_SIZE {
                return Some(self.unmarked_sum * draw);
            }
        }
        None
    }

    pub fn has_won(&self) -> bool {
        self.row_marks.iter().any(|n| *n >= 5) || self.col_marks.iter().any(|n| *n >= 5)
    }
}

impl FromStr for Board {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows = s.split('\n');
        let numbers = rows
            .map(str::trim)
            .map(str::split_whitespace)
            .map(|ns| ns.map(|n| n.parse::<u64>().unwrap()).collect::<Vec<u64>>())
            .collect::<Vec<Vec<u64>>>();

        Ok(Board::new(&numbers))
    }
}

fn part1(boards: &mut [Board], draws: &[u64]) -> u64 {
    for draw in draws {
        for board in &mut *boards {
            if let Some(score) = board.mark(*draw) {
                return score;
            }
        }
    }
    0
}

fn part2(boards: &mut [Board], draws: &[u64]) -> u64 {
    for draw in draws {
        let mut count: u64 = 0;
        let mut last_result: Option<u64> = None;
        for board in &mut *boards {
            if !board.has_won() {
                count += 1;
                last_result = board.mark(*draw);
            }
        }
        if count == 1 {
            if let Some(score) = last_result {
                return score;
            }
        }
    }
    0
}

#[cfg(test)]
mod day3_tests {
    use super::*;

    const EXAMPLE_DRAWS: [u64; 27] = [
        7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19, 3,
        26, 1,
    ];

    fn example_boards() -> [Board; 3] {
        [
            Board::new(&vec![
                vec![22, 13, 17, 11, 0],
                vec![8, 2, 23, 4, 24],
                vec![21, 9, 14, 16, 7],
                vec![6, 10, 3, 18, 5],
                vec![1, 12, 20, 15, 19],
            ]),
            Board::new(&vec![
                vec![3, 15, 0, 2, 22],
                vec![9, 18, 13, 17, 5],
                vec![19, 8, 7, 25, 23],
                vec![20, 11, 10, 24, 4],
                vec![14, 21, 16, 12, 6],
            ]),
            Board::new(&vec![
                vec![14, 21, 17, 24, 4],
                vec![10, 16, 15, 9, 19],
                vec![18, 8, 23, 26, 20],
                vec![22, 11, 13, 6, 5],
                vec![2, 0, 12, 3, 7],
            ]),
        ]
    }

    #[test]
    fn part1_example() {
        let mut boards = example_boards();
        let result = part1(&mut boards, &EXAMPLE_DRAWS);
        assert_eq!(result, 4512);
    }

    #[test]
    fn part2_example() {
        let mut boards = example_boards();
        let result = part2(&mut boards, &EXAMPLE_DRAWS);
        assert_eq!(result, 1924);
    }
}
