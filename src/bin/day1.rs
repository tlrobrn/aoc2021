#![warn(clippy::all, clippy::pedantic)]
extern crate aoc2021;
use aoc2021::{input_lines, parse_lines};

fn main() {
    let input: Vec<u64> = parse_lines(input_lines()).collect();
    let result2 = part2(&input);
    let result = part1(input);
    println!("Part 1: {}", result);
    println!("Part 2: {}", result2);
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

fn part2(depths: &[u64]) -> u64 {
    let mut count = 0;
    let mut prev = None;
    for depth_window in depths.windows(3) {
        let sum = depth_window[0] + depth_window[1] + depth_window[2];
        if let Some(prev_sum) = prev {
            if prev_sum < sum {
                count += 1;
            }
        }
        prev = Some(sum);
    }
    count
}

#[cfg(test)]
mod day1_tests {
    use super::*;

    const EXAMPLE_INPUT: [u64; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    #[test]
    fn part1_example() {
        let result = part1(EXAMPLE_INPUT);
        assert_eq!(result, 7);
    }

    #[test]
    fn part2_example() {
        let result = part2(&EXAMPLE_INPUT);
        assert_eq!(result, 5);
    }
}
