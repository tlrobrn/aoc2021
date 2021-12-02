#![warn(clippy::all, clippy::pedantic)]
use aoc2021::{input_lines, parse_lines};
use std::str::FromStr;

#[derive(Clone)]
enum Command {
    Forward(i64),
    Down(i64),
    Up(i64),
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();
        let direction = parts[0];
        let units = parts[1].parse::<i64>().map_err(|e| e.to_string())?;

        match direction {
            "forward" => Ok(Command::Forward(units)),
            "down" => Ok(Command::Down(units)),
            "up" => Ok(Command::Up(units)),
            _ => Err(String::from("wtf")),
        }
    }
}

fn main() {
    let input: Vec<Command> = parse_lines(input_lines()).collect();
    let result = part1(&input);
    println!("Part 1: {}", result);
    let result = part2(&input);
    println!("Part 2: {}", result);
}

fn part1(commands: &[Command]) -> i64 {
    let (horizontal_position, depth) =
        commands
            .iter()
            .fold((0, 0), |(hp, d), command| match command {
                Command::Forward(units) => (hp + units, d),
                Command::Down(units) => (hp, d + units),
                Command::Up(units) => (hp, d - units),
            });
    horizontal_position * depth
}

#[derive(Default)]
struct Heading {
    depth: i64,
    horizontal_position: i64,
    aim: i64,
}

impl Heading {
    fn down(&self, units: i64) -> Self {
        Self {
            depth: self.depth,
            horizontal_position: self.horizontal_position,
            aim: self.aim + units,
        }
    }

    fn up(&self, units: i64) -> Self {
        Self {
            depth: self.depth,
            horizontal_position: self.horizontal_position,
            aim: self.aim - units,
        }
    }

    fn forward(&self, units: i64) -> Self {
        Self {
            depth: self.depth + (self.aim * units),
            horizontal_position: self.horizontal_position + units,
            aim: self.aim,
        }
    }
}

fn part2(commands: &[Command]) -> i64 {
    let heading =
        commands
            .iter()
            .cloned()
            .fold(Heading::default(), |heading, command| match command {
                Command::Forward(units) => heading.forward(units),
                Command::Down(units) => heading.down(units),
                Command::Up(units) => heading.up(units),
            });
    heading.horizontal_position * heading.depth
}

#[cfg(test)]
mod day2_tests {
    use super::*;

    const EXAMPLE_INPUT: [Command; 6] = [
        Command::Forward(5),
        Command::Down(5),
        Command::Forward(8),
        Command::Up(3),
        Command::Down(8),
        Command::Forward(2),
    ];

    #[test]
    fn part1_example() {
        let result = part1(&EXAMPLE_INPUT);
        assert_eq!(result, 150);
    }

    #[test]
    fn part2_example() {
        let result = part2(&EXAMPLE_INPUT);
        assert_eq!(result, 900);
    }
}
