#![warn(clippy::all, clippy::pedantic)]
use aoc2021::input_lines;
use std::num::ParseIntError;
use std::str::FromStr;

fn main() {
    let mut initial_state = parse();
    let result = part1(&mut initial_state, 80);
    println!("Part 1: {}", result);
}

fn parse() -> Vec<LanternFish> {
    if let Some(line) = input_lines().next() {
        line.trim().split(',').map(|f| f.parse().unwrap()).collect()
    } else {
        vec![]
    }
}

fn part1(school: &mut Vec<LanternFish>, days: usize) -> usize {
    for _day in 1..=days {
        for i in 0..school.len() {
            if let Some(child) = school[i].mature() {
                school.push(child);
            }
        }
    }
    school.len()
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct LanternFish(usize);

impl FromStr for LanternFish {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(LanternFish(usize::from_str(s)?))
    }
}

impl LanternFish {
    fn mature(&mut self) -> Option<Self> {
        if self.0 == 0 {
            self.0 = 6;
            Some(LanternFish(8))
        } else {
            self.0 -= 1;
            None
        }
    }
}

#[cfg(test)]
mod day6_tests {
    use super::*;

    fn initial_state() -> Vec<LanternFish> {
        vec![
            LanternFish(3),
            LanternFish(4),
            LanternFish(3),
            LanternFish(1),
            LanternFish(2),
        ]
    }

    #[test]
    fn test_part1_short() {
        let result = part1(&mut initial_state(), 18);
        assert_eq!(result, 26);
    }

    #[test]
    fn test_part1_long() {
        let result = part1(&mut initial_state(), 80);
        assert_eq!(result, 5934);
    }
}
