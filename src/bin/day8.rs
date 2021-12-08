#![warn(clippy::all, clippy::pedantic)]
use aoc2021::{input_lines, parse_lines};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::str::FromStr;

fn main() {
    let notes: Vec<NoteEntry> = parse_lines(input_lines()).collect();
    let result = part1(&notes);
    println!("Part 1: {}", result);
    let result = part2(&notes);
    println!("Part 2: {}", result);
}

fn part1(notes: &[NoteEntry]) -> usize {
    notes
        .iter()
        .flat_map(|note| note.output_value.iter())
        .fold(0, |sum, signal| {
            if signal.is_unique_length() {
                sum + 1
            } else {
                sum
            }
        })
}

fn part2(notes: &[NoteEntry]) -> usize {
    notes.iter().map(decode).sum()
}

fn decode(note: &NoteEntry) -> usize {
    let mapping = encoding(&note.signal_patterns);
    let digits = note
        .output_value
        .iter()
        .map(|digit| {
            mapping.iter().fold(digit.0.clone(), |s, (k, v)| {
                s.replace(&k.to_string(), &v.to_string())
            })
        })
        .map(|s| Digit::from_str(&s).expect("oops"));
    digits
        .map(|d| d.0.to_string())
        .collect::<String>()
        .parse::<usize>()
        .expect("oops again")
}

#[derive(Clone, Default)]
struct Pattern {
    segments: HashSet<char>,
}

impl FromStr for Pattern {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let segments: HashSet<char> = s.chars().collect();
        Ok(Pattern { segments })
    }
}

impl Pattern {
    fn len(&self) -> usize {
        self.segments.len()
    }

    fn is_one(&self) -> bool {
        self.len() == 2
    }

    fn is_seven(&self) -> bool {
        self.len() == 3
    }

    fn is_four(&self) -> bool {
        self.len() == 4
    }

    fn is_eight(&self) -> bool {
        self.len() == 7
    }
}

fn difference<T>(a: &HashSet<T>, b: &HashSet<T>) -> HashSet<T>
where
    T: Eq + Hash + Clone,
{
    a.difference(b).cloned().collect()
}

fn intersection<T>(a: &HashSet<T>, b: &HashSet<T>) -> HashSet<T>
where
    T: Eq + Hash + Clone,
{
    a.intersection(b).cloned().collect()
}

fn union<T>(a: &HashSet<T>, b: &HashSet<T>) -> HashSet<T>
where
    T: Eq + Hash + Clone,
{
    a.union(b).cloned().collect()
}

#[allow(clippy::similar_names, clippy::many_single_char_names)]
fn encoding(patterns: &[Pattern]) -> HashMap<char, char> {
    let seven = patterns
        .iter()
        .find(|&p| p.is_seven())
        .expect("could not find 7");
    let one = patterns
        .iter()
        .find(|&p| p.is_one())
        .expect("could not find 1");

    let a: HashSet<char> = difference(&seven.segments, &one.segments);

    let mut zero_six_nine = patterns.iter().filter(|&p| p.len() == 6);
    let one_of_069 = zero_six_nine.next().expect("069");
    let zero_six_nine_segments = zero_six_nine.fold(one_of_069.segments.clone(), |acc, zsn| {
        intersection(&acc, &zsn.segments)
    });

    let f = intersection(&zero_six_nine_segments, &one.segments);
    let c = difference(&one.segments, &f);

    let mut two_three_five = patterns.iter().filter(|&p| p.len() == 5);
    let one_of_235 = two_three_five.next().expect("235");
    let two_three_five_segments = two_three_five.fold(one_of_235.segments.clone(), |acc, ttf| {
        intersection(&acc, &ttf.segments)
    });
    let d_or_g = difference(&two_three_five_segments, &a);
    let four = patterns.iter().find(|&p| p.is_four()).expect("4");
    let d = intersection(&d_or_g, &four.segments);
    let g = difference(&d_or_g, &four.segments);

    let cd = union(&c, &d);
    let cdf = union(&cd, &f);
    let b = difference(&four.segments, &cdf);

    let acdf = union(&cdf, &a);
    let abcdf = union(&acdf, &b);
    let abcdfg = union(&abcdf, &g);
    let eight = patterns.iter().find(|&p| p.is_eight()).expect("8");
    let e = difference(&eight.segments, &abcdfg);

    let keys = [a, b, c, d, e, f, g].into_iter().flatten();
    let values = ['A', 'B', 'C', 'D', 'E', 'F', 'G'];
    keys.zip(values.into_iter()).collect()
}

#[derive(Default, Clone)]
struct Signal(String);

impl FromStr for Signal {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

impl Signal {
    fn is_unique_length(&self) -> bool {
        self.possibile_digits().len() == 1
    }

    fn possibile_digits(&self) -> Vec<u8> {
        match self.0.len() {
            2 => vec![1],
            3 => vec![7],
            4 => vec![4],
            5 => vec![2, 3, 5],
            6 => vec![0, 6, 9],
            7 => vec![8],
            _ => vec![],
        }
    }
}

#[derive(Clone, Copy)]
struct Digit(u8);

impl FromStr for Digit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars: Vec<char> = s.chars().collect();
        chars.sort_unstable();
        let segments: String = chars.iter().collect();
        match segments.as_str() {
            "ABCEFG" => Ok(Digit(0)),
            "CF" => Ok(Digit(1)),
            "ACDEG" => Ok(Digit(2)),
            "ACDFG" => Ok(Digit(3)),
            "BCDF" => Ok(Digit(4)),
            "ABDFG" => Ok(Digit(5)),
            "ABDEFG" => Ok(Digit(6)),
            "ACF" => Ok(Digit(7)),
            "ABCDEFG" => Ok(Digit(8)),
            "ABCDFG" => Ok(Digit(9)),
            _ => Err(segments),
        }
    }
}

#[derive(Default, Clone)]
struct NoteEntry {
    signal_patterns: [Pattern; 10],
    output_value: [Signal; 4],
}

impl FromStr for NoteEntry {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut entry = Self::default();
        let parts: Vec<&str> = s.split(" | ").collect();

        let signal_patterns = parts[0].split(' ');
        for (i, signal) in signal_patterns.enumerate() {
            if let Ok(signal) = Pattern::from_str(signal) {
                entry.signal_patterns[i] = signal;
            }
        }

        let output_value = parts[1].split(' ');
        for (i, signal) in output_value.enumerate() {
            if let Ok(signal) = Signal::from_str(signal) {
                entry.output_value[i] = signal;
            }
        }

        Ok(entry)
    }
}

#[cfg(test)]
mod day8_tests {
    use super::*;

    const SHORT_EXAMPLE: [&str; 1] =
        ["acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"];

    const LONG_EXAMPLE: [&str; 10] = [
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
        "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc",
        "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg",
        "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb",
        "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea",
        "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb",
        "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe",
        "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef",
        "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb",
        "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce",
    ];

    fn parse(lines: &[&str]) -> Vec<NoteEntry> {
        lines
            .iter()
            .map(|&line| line.parse().expect("how"))
            .collect()
    }

    #[test]
    fn test_part1_short() {
        let input = parse(&SHORT_EXAMPLE);
        let result = part1(&input);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part1_long() {
        let input = parse(&LONG_EXAMPLE);
        let result = part1(&input);
        assert_eq!(result, 26);
    }

    #[test]
    fn test_part2_short() {
        let input = parse(&SHORT_EXAMPLE);
        let result = part2(&input);
        assert_eq!(result, 5353);
    }

    #[test]
    fn test_part2_long() {
        let input = parse(&LONG_EXAMPLE);
        let result = part2(&input);
        assert_eq!(result, 61229);
    }
}
