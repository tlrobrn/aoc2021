#![warn(clippy::all, clippy::pedantic)]
extern crate aoc2021;
use aoc2021::{input_lines, parse_lines};

fn main() {
    let input = parse_lines(input_lines());
    let result = part1(input);
    println!("Part 1: {}", result);
}

fn part1<I>(depths: I) -> u64
where
    I: IntoIterator<Item = u64>,
{
    let mut count = 0;
    let mut prev = None;
    for depth in depths {
        if let Some(prev_depth) = prev {
            if prev_depth < depth {
                count += 1;
            }
        }
        prev = Some(depth);
    }
    count
}

#[cfg(test)]
mod day1_tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let result = part1(input);
        assert_eq!(result, 7);
    }
}
