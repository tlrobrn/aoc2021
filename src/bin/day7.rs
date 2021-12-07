#![warn(clippy::all, clippy::pedantic)]
#![feature(int_abs_diff)]
use aoc2021::{input_lines, parse_lines};
use std::collections::HashMap;
use std::str::FromStr;

fn main() {
    let positions: CrabPositions = parse_lines(input_lines()).next().expect("no crabs?");
    let result = part1(&positions);
    println!("Part 1: {}", result);
    let result = part2(&positions);
    println!("Part 2: {}", result);
}

fn part1(positions: &CrabPositions) -> usize {
    let min = *positions.0.keys().min().expect("please");
    let max = *positions.0.keys().max().expect("please");
    let mut minimum_cost = usize::MAX;

    for position in min..=max {
        let cost = positions.cost_to_align_at(position);
        if cost < minimum_cost {
            minimum_cost = cost;
        }
    }

    minimum_cost
}

fn part2(positions: &CrabPositions) -> usize {
    let min = *positions.0.keys().min().expect("please");
    let max = *positions.0.keys().max().expect("please");
    let mut minimum_cost = usize::MAX;

    for position in min..=max {
        let cost = positions.accurate_cost_to_align_at(position);
        if cost < minimum_cost {
            minimum_cost = cost;
        }
    }

    minimum_cost
}

#[derive(Debug, Clone, Default)]
struct CrabPositions(HashMap<usize, usize>);

impl FromStr for CrabPositions {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let crabs = s
            .trim()
            .split(',')
            .map(|position| usize::from_str(position).expect("wtf"));

        let mut positions = Self::default();
        for crab_position in crabs {
            let entry = positions.0.entry(crab_position).or_insert(0);
            *entry += 1;
        }
        Ok(positions)
    }
}

impl CrabPositions {
    fn cost_to_align_at(&self, position: usize) -> usize {
        self.0.iter().fold(0, |sum, (crab_position, crab_count)| {
            sum + position.abs_diff(*crab_position) * crab_count
        })
    }

    fn accurate_cost_to_align_at(&self, position: usize) -> usize {
        self.0.iter().fold(0, |sum, (crab_position, crab_count)| {
            let steps = position.abs_diff(*crab_position);
            let cost = steps * (steps + 1) / 2;
            sum + cost * crab_count
        })
    }
}

#[cfg(test)]
mod day6_tests {
    use super::*;

    const INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_part1() {
        let result = part1(&CrabPositions::from_str(INPUT).unwrap());
        assert_eq!(result, 37);
    }

    #[test]
    fn test_part2() {
        let result = part2(&CrabPositions::from_str(INPUT).unwrap());
        assert_eq!(result, 168);
    }
}
