#![warn(clippy::all, clippy::pedantic)]
use aoc2021::{input_lines, parse_lines};
use std::str::FromStr;

fn main() {
    let population: LanternFishPopulation = parse_lines(input_lines()).next().expect("no fishes");
    let result = part1(population);
    println!("Part 1: {}", result);
    let result = part2(population);
    println!("Part 2: {}", result);
}

fn part1(mut population: LanternFishPopulation) -> u128 {
    population.live(80).count()
}

fn part2(mut population: LanternFishPopulation) -> u128 {
    population.live(256).count()
}

#[derive(Debug, Clone, Copy, Default)]
struct LanternFishPopulation([u128; 9]);

impl FromStr for LanternFishPopulation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fishes = s
            .trim()
            .split(',')
            .map(|age| usize::from_str(age).expect("wtf"));

        let mut population = Self::default();
        for age in fishes {
            population.0[age] += 1;
        }
        Ok(population)
    }
}

impl LanternFishPopulation {
    fn count(&self) -> u128 {
        self.0.iter().sum()
    }

    fn age(&mut self) {
        self.0.rotate_left(1);
        self.0[6] += self.0[8];
    }

    fn live(&mut self, days: usize) -> &mut Self {
        for _day in 1..=days {
            self.age();
        }
        self
    }
}

#[cfg(test)]
mod day6_tests {
    use super::*;

    fn initial_state() -> LanternFishPopulation {
        LanternFishPopulation([0, 1, 1, 2, 1, 0, 0, 0, 0])
    }

    #[test]
    fn test_live_short() {
        let mut pop = initial_state();
        let result = pop.live(18).count();
        assert_eq!(result, 26);
    }

    #[test]
    fn test_live_long() {
        let mut pop = initial_state();
        let result = pop.live(80).count();
        assert_eq!(result, 5934);
    }

    #[test]
    fn test_live_really_long() {
        let mut pop = initial_state();
        let result = pop.live(256).count();
        assert_eq!(result, 26984457539);
    }
}
