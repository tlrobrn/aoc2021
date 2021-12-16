#![warn(clippy::all, clippy::pedantic)]
use aoc2021::point::Point;
use aoc2021::{input_lines, parse_lines};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

fn main() {
    let line_segments: Vec<LineSegment> = parse_lines(input_lines()).collect();
    let result = part1(&line_segments);
    println!("{}", result);
    let result = part2(&line_segments);
    println!("{}", result);
}

fn part1(line_segments: &[LineSegment]) -> usize {
    let mut points: HashMap<Point, usize> = HashMap::default();
    for line in line_segments {
        if !line.is_diagonal() {
            for point in line.into_iter() {
                let entry = points.entry(point).or_insert(0);
                *entry += 1;
            }
        }
    }
    points.values().filter(|count| **count >= 2).count()
}

fn part2(line_segments: &[LineSegment]) -> usize {
    let mut points: HashMap<Point, usize> = HashMap::default();
    for line in line_segments {
        for point in line.into_iter() {
            let entry = points.entry(point).or_insert(0);
            *entry += 1;
        }
    }
    points.values().filter(|count| **count >= 2).count()
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct LineSegment {
    a: Point,
    b: Point,
}

struct LineSegmentIterator {
    end: Point,
    step: Point,
    next_point: Option<Point>,
}

impl Iterator for LineSegmentIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(point) = self.next_point {
            self.next_point = if point == self.end {
                None
            } else {
                Some(point + self.step)
            };
            return Some(point);
        }
        None
    }
}

impl IntoIterator for LineSegment {
    type Item = Point;
    type IntoIter = LineSegmentIterator;

    fn into_iter(self) -> Self::IntoIter {
        let step = self.step();
        let end = self.b;
        let next_point = Some(self.a);

        Self::IntoIter {
            end,
            step,
            next_point,
        }
    }
}

impl FromStr for LineSegment {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(" -> ").collect();
        let a = parts[0].parse()?;
        let b = parts[1].parse()?;
        Ok(LineSegment { a, b })
    }
}

impl LineSegment {
    fn step(&self) -> Point {
        let x = match self.a.x.cmp(&self.b.x) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        };
        let y = match self.a.y.cmp(&self.b.y) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        };
        Point { x, y }
    }

    fn is_horizontal(&self) -> bool {
        self.a.y == self.b.y
    }

    fn is_vertical(&self) -> bool {
        self.a.x == self.b.x
    }

    fn is_diagonal(&self) -> bool {
        !self.is_horizontal() && !self.is_vertical()
    }
}

#[cfg(test)]
mod line_segment_tests {
    use super::{LineSegment, Point};

    #[test]
    fn test_parse() {
        let result = "1,2 -> 3,4".parse();
        assert_eq!(
            result,
            Ok(LineSegment {
                a: Point { x: 1, y: 2 },
                b: Point { x: 3, y: 4 }
            })
        );
    }

    #[test]
    fn test_horizontal_iterator() {
        let line = LineSegment {
            a: Point { x: 5, y: 2 },
            b: Point { x: 3, y: 2 },
        };
        let mut points = line.into_iter();
        assert_eq!(Some(Point { x: 5, y: 2 }), points.next());
        assert_eq!(Some(Point { x: 4, y: 2 }), points.next());
        assert_eq!(Some(Point { x: 3, y: 2 }), points.next());
        assert_eq!(None, points.next());
    }
}

#[cfg(test)]
mod day5_tests {
    use super::*;

    const EXAMPLE_INPUT: [LineSegment; 10] = [
        LineSegment {
            a: Point { x: 0, y: 9 },
            b: Point { x: 5, y: 9 },
        },
        LineSegment {
            a: Point { x: 8, y: 0 },
            b: Point { x: 0, y: 8 },
        },
        LineSegment {
            a: Point { x: 9, y: 4 },
            b: Point { x: 3, y: 4 },
        },
        LineSegment {
            a: Point { x: 2, y: 2 },
            b: Point { x: 2, y: 1 },
        },
        LineSegment {
            a: Point { x: 7, y: 0 },
            b: Point { x: 7, y: 4 },
        },
        LineSegment {
            a: Point { x: 6, y: 4 },
            b: Point { x: 2, y: 0 },
        },
        LineSegment {
            a: Point { x: 0, y: 9 },
            b: Point { x: 2, y: 9 },
        },
        LineSegment {
            a: Point { x: 3, y: 4 },
            b: Point { x: 1, y: 4 },
        },
        LineSegment {
            a: Point { x: 0, y: 0 },
            b: Point { x: 8, y: 8 },
        },
        LineSegment {
            a: Point { x: 5, y: 5 },
            b: Point { x: 8, y: 2 },
        },
    ];

    #[test]
    fn test_part1() {
        let result = part1(&EXAMPLE_INPUT);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_part2() {
        let result = part2(&EXAMPLE_INPUT);
        assert_eq!(result, 12);
    }
}
