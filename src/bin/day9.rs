#![warn(clippy::all, clippy::pedantic)]
use aoc2021::input_lines;
use std::collections::{HashSet, VecDeque};
use std::ops::{Index, IndexMut};

fn main() {
    let height_map = HeightMap::from_lines(input_lines());
    let result = part1(&height_map);
    println!("Part 1: {}", result);
    let result = part2(&height_map);
    println!("Part 2: {}", result);
}

fn part1(height_map: &HeightMap) -> usize {
    let mut risk_level = 0usize;
    for &point in &height_map.low_points() {
        let height = height_map[point];
        risk_level += usize::try_from(height).expect("how?") + 1;
    }
    risk_level
}

fn part2(height_map: &HeightMap) -> usize {
    let mut basins: Vec<usize> = height_map
        .low_points()
        .iter()
        .map(|&pt| height_map.basin_for(pt))
        .collect();
    basins.sort_unstable_by_key(|&b| -as_isize(b));

    basins.iter().take(3).product()
}

fn as_isize(u: usize) -> isize {
    u.try_into().expect("overflow")
}

fn as_usize(i: isize) -> usize {
    usize::try_from(i).expect("lost sign")
}

#[derive(Default)]
struct HeightMap {
    locations: Vec<u32>,
    width: isize,
    height: isize,
}

type Point = (isize, isize);
impl HeightMap {
    fn from_lines<I>(lines: I) -> Self
    where
        I: Iterator<Item = String>,
    {
        let mut height_map = Self::default();
        for line in lines {
            let line = line.trim();
            if height_map.width == 0 {
                height_map.width = line.len().try_into().expect("overflow");
            }
            height_map.height += 1;
            for c in line.chars() {
                let height = c.to_digit(10).expect("not a number");
                height_map.locations.push(height);
            }
        }
        height_map
    }

    fn low_points(&self) -> Vec<Point> {
        let mut lows = vec![];
        for y in 0..self.height {
            for x in 0..self.width {
                let point = (x, y);
                let height = self[point];
                if self
                    .neighboring_points((x, y))
                    .iter()
                    .all(|&pt| self[pt] > height)
                {
                    lows.push(point);
                }
            }
        }
        lows
    }

    fn basin_for(&self, point: Point) -> usize {
        let mut queue = VecDeque::from([point]);
        let mut seen: HashSet<Point> = HashSet::default();
        let mut size = 0;
        while let Some(p) = queue.pop_front() {
            if seen.contains(&p) {
                continue;
            }
            seen.insert(p);
            let height = self[p];
            if height < 9 {
                size += 1;
                queue.append(&mut VecDeque::from(self.neighboring_points(p)));
            }
        }
        size
    }

    fn neighboring_points(&self, (x, y): Point) -> Vec<Point> {
        [(x - 1, y), (x, y - 1), (x + 1, y), (x, y + 1)]
            .iter()
            .filter(|&&pt| self.index_for(pt).is_ok())
            .copied()
            .collect()
    }

    fn index_for(&self, (x, y): Point) -> Result<usize, String> {
        if self.width == 0 {
            return Err("width must me greater than 0".to_string());
        }
        if x < 0 || x >= self.width || y < 0 || y >= self.width {
            return Err("index out of bounds".to_string());
        }
        match (y * self.width) + x {
            i if i < 0 => Err("index out of bounds".to_string()),
            i if as_usize(i) >= self.locations.len() => Err("index out of bounds".to_string()),
            i => Ok(as_usize(i)),
        }
    }
}

impl Index<Point> for HeightMap {
    type Output = u32;

    fn index(&self, point: Point) -> &Self::Output {
        let idx = self.index_for(point).unwrap();
        &self.locations[idx]
    }
}

impl IndexMut<Point> for HeightMap {
    fn index_mut(&mut self, point: Point) -> &mut Self::Output {
        let idx = self.index_for(point).unwrap();
        &mut self.locations[idx]
    }
}

#[cfg(test)]
mod day9_tests {
    use super::*;

    const EXAMPLE_INPUT: [&str; 5] = [
        "2199943210\n",
        "3987894921\n",
        "9856789892\n",
        "8767896789\n",
        "9899965678\n",
    ];

    fn string_input() -> impl Iterator<Item = String> {
        EXAMPLE_INPUT.iter().map(|line| line.to_string())
    }

    #[test]
    fn part1_example() {
        let hm = HeightMap::from_lines(string_input());
        let result = part1(&hm);
        assert_eq!(result, 15);
    }

    #[test]
    fn part2_example() {
        let hm = HeightMap::from_lines(string_input());
        let result = part2(&hm);
        assert_eq!(result, 1134);
    }
}
