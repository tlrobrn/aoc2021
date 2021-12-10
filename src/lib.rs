#![warn(clippy::all, clippy::pedantic)]
#![feature(stdin_forwarders)]

pub mod stack;

use std::io;
use std::result::Result;
use std::str::FromStr;

pub fn input_lines() -> impl Iterator<Item = String> {
    io::stdin().lines().map(Result::unwrap_or_default)
}

pub fn parse_lines<I, T>(lines: I) -> impl Iterator<Item = T>
where
    I: Iterator<Item = String>,
    T: FromStr,
{
    lines.flat_map(|line| {
        if let Ok(result) = line.trim().parse::<T>() {
            vec![result]
        } else {
            vec![]
        }
    })
}

#[cfg(test)]
mod aoc2021_tests {
    use super::*;

    #[test]
    fn test_parse_lines() {
        let input = vec!["123\n".to_string(), "456\n".to_string()];
        let result: Vec<i64> = parse_lines(input.into_iter()).collect();
        assert_eq!(result[0], 123);
        assert_eq!(result[1], 456);
    }
}
