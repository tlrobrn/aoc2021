#![warn(clippy::all, clippy::pedantic)]
use aoc2021::input_lines;
use aoc2021::point::Point;
use std::collections::HashSet;
use std::ops::{Index, IndexMut};

fn main() {
    let octopii = parse_input(input_lines());
    let result = part1(octopii);
    println!("Part 1: {}", result);
}

fn part1(mut octopii: Octopii) -> usize {
    (0..100).map(|_| octopii.step()).sum()
}

#[derive(Clone, Copy)]
struct Octopii {
    map: [u8; 100],
}

impl Default for Octopii {
    fn default() -> Self {
        Self { map: [0; 100] }
    }
}

impl Octopii {
    fn step(&mut self) -> usize {
        let mut to_energize = all_points();
        let mut flash_count = 0;
        let mut flashed: HashSet<Point> = HashSet::default();

        while let Some(point) = to_energize.pop() {
            if !flashed.contains(&point) {
                match self[point] {
                    9 => {
                        self[point] = 0;
                        flash_count += 1;
                        flashed.insert(point);
                        to_energize.append(&mut adjacent_points(point));
                    }
                    _ => self[point] += 1,
                }
            }
        }

        flash_count
    }
}

fn all_points() -> Vec<Point> {
    let mut points = Vec::with_capacity(100);
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            points.push(Point { x, y });
        }
    }
    points
}

fn adjacent_points(point: Point) -> Vec<Point> {
    let mut result = vec![];
    let steps: [i64; 3] = [-1, 0, 1];
    for x in steps {
        for y in steps {
            if x != 0 || y != 0 {
                let neighbor = point + Point { x, y };
                if array_index_for_point(neighbor).is_some() {
                    result.push(neighbor);
                }
            }
        }
    }

    result
}

const WIDTH: i64 = 10;
const HEIGHT: i64 = 10;
fn array_index_for_point(point: Point) -> Option<usize> {
    match point {
        Point { x, y } if !(0..WIDTH).contains(&x) || !(0..HEIGHT).contains(&y) => None,
        Point { x, y } => usize::try_from(y * WIDTH + x).ok(),
    }
}

impl Index<Point> for Octopii {
    type Output = u8;

    fn index(&self, point: Point) -> &Self::Output {
        match array_index_for_point(point) {
            Some(i) => &self.map[i],
            None => panic!("out of bounds"),
        }
    }
}

impl IndexMut<Point> for Octopii {
    fn index_mut(&mut self, point: Point) -> &mut Self::Output {
        match array_index_for_point(point) {
            Some(i) => &mut self.map[i],
            None => panic!("out of bounds"),
        }
    }
}

fn parse_input<I>(lines: I) -> Octopii
where
    I: Iterator<Item = String>,
{
    let mut octopii = Octopii::default();
    for (y, line) in lines.enumerate() {
        let line = line.trim();
        for (x, c) in line.chars().enumerate() {
            octopii[Point {
                x: x as i64,
                y: y as i64,
            }] = u8::try_from(c.to_digit(10).expect("not a number")).unwrap();
        }
    }
    octopii
}

#[cfg(test)]
mod day11_tests {
    use super::*;

    const INPUT: [&str; 10] = [
        "5483143223\n",
        "2745854711\n",
        "5264556173\n",
        "6141336146\n",
        "6357385478\n",
        "4167524645\n",
        "2176841721\n",
        "6882881134\n",
        "4846848554\n",
        "5283751526\n",
    ];

    fn test_input() -> Octopii {
        parse_input(INPUT.iter().map(|s| s.to_string()))
    }

    #[test]
    fn test_part1() {
        assert_eq!(1656, part1(test_input()));
    }
}
