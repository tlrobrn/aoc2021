#![warn(clippy::all, clippy::pedantic)]
use aoc2021::{input_lines, parse_lines};
use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

fn main() {
    let line_segments: Vec<LineSegment> = parse_lines(input_lines()).collect();
    let result = part1(&line_segments);
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').collect();
        let x = parts[0].parse()?;
        let y = parts[1].parse()?;
        Ok(Point { x, y })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct LineSegment {
    a: Point,
    b: Point,
}

struct LineSegmentIterator {
    start: Point,
    end: Point,
    current: Option<Point>,
}

impl Iterator for LineSegmentIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(point) = self.current {
            if self.is_vertical() {
                let y = point.y + 1;
                self.current = if y > self.end.y {
                    None
                } else {
                    Some(Point { x: point.x, y })
                };
            } else {
                let x = point.x + 1;
                self.current = if x > self.end.x {
                    None
                } else {
                    Some(Point { x, y: point.y })
                };
            }
        } else {
            self.current = Some(self.start);
        }
        self.current
    }
}

impl LineSegmentIterator {
    fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }
}

impl IntoIterator for LineSegment {
    type Item = Point;
    type IntoIter = LineSegmentIterator;

    fn into_iter(self) -> Self::IntoIter {
        let (start, end) = if (self.is_vertical() && self.a.y < self.b.y) || self.a.x < self.b.x {
            (self.a, self.b)
        } else {
            (self.b, self.a)
        };

        LineSegmentIterator {
            start,
            end,
            current: None,
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
        assert_eq!(Some(Point { x: 3, y: 2 }), points.next());
        assert_eq!(Some(Point { x: 4, y: 2 }), points.next());
        assert_eq!(Some(Point { x: 5, y: 2 }), points.next());
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
}
