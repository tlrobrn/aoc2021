#![warn(clippy::all, clippy::pedantic)]
extern crate aoc2021;
use aoc2021::{input_lines, parse_lines};
use std::str::FromStr;

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
}
