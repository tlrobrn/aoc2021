#![warn(clippy::all, clippy::pedantic)]
use aoc2021::{input_lines, parse_lines};
use std::str::FromStr;

fn main() {
    let notes: Vec<NoteEntry> = parse_lines(input_lines()).collect();
    let result = part1(&notes);
    println!("Part 1: {}", result);
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
        let length = self.0.len();
        length == 2 || length == 4 || length == 3 || length == 7
    }
}

#[derive(Default, Clone)]
struct NoteEntry {
    signal_patterns: [Signal; 10],
    output_value: [Signal; 4],
}

impl FromStr for NoteEntry {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut entry = Self::default();
        let parts: Vec<&str> = s.split(" | ").collect();

        let signal_patterns = parts[0].split(' ');
        for (i, signal) in signal_patterns.enumerate() {
            if let Ok(signal) = Signal::from_str(signal) {
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

    fn input() -> [NoteEntry; 10] {
        [
            NoteEntry {
                signal_patterns: [
                    Signal("be".to_string()),
                    Signal("cfbegad".to_string()),
                    Signal("cbdgef".to_string()),
                    Signal("fgaecd".to_string()),
                    Signal("cgeb".to_string()),
                    Signal("fdcge".to_string()),
                    Signal("agebfd".to_string()),
                    Signal("fecdb".to_string()),
                    Signal("fabcd".to_string()),
                    Signal("edb".to_string()),
                ],
                output_value: [
                    Signal("fdgacbe".to_string()),
                    Signal("cefdb".to_string()),
                    Signal("cefbgd".to_string()),
                    Signal("gcbe".to_string()),
                ],
            },
            NoteEntry {
                signal_patterns: [
                    Signal("edbfga".to_string()),
                    Signal("begcd".to_string()),
                    Signal("cbg".to_string()),
                    Signal("gc".to_string()),
                    Signal("gcadebf".to_string()),
                    Signal("fbgde".to_string()),
                    Signal("acbgfd".to_string()),
                    Signal("abcde".to_string()),
                    Signal("gfcbed".to_string()),
                    Signal("gfec".to_string()),
                ],
                output_value: [
                    Signal("fcgedb".to_string()),
                    Signal("cgb".to_string()),
                    Signal("dgebacf".to_string()),
                    Signal("gc".to_string()),
                ],
            },
            NoteEntry {
                signal_patterns: [
                    Signal("fgaebd".to_string()),
                    Signal("cg".to_string()),
                    Signal("bdaec".to_string()),
                    Signal("gdafb".to_string()),
                    Signal("agbcfd".to_string()),
                    Signal("gdcbef".to_string()),
                    Signal("bgcad".to_string()),
                    Signal("gfac".to_string()),
                    Signal("gcb".to_string()),
                    Signal("cdgabef".to_string()),
                ],
                output_value: [
                    Signal("cg".to_string()),
                    Signal("cg".to_string()),
                    Signal("fdcagb".to_string()),
                    Signal("cbg".to_string()),
                ],
            },
            NoteEntry {
                signal_patterns: [
                    Signal("fbegcd".to_string()),
                    Signal("cbd".to_string()),
                    Signal("adcefb".to_string()),
                    Signal("dageb".to_string()),
                    Signal("afcb".to_string()),
                    Signal("bc".to_string()),
                    Signal("aefdc".to_string()),
                    Signal("ecdab".to_string()),
                    Signal("fgdeca".to_string()),
                    Signal("fcdbega".to_string()),
                ],
                output_value: [
                    Signal("efabcd".to_string()),
                    Signal("cedba".to_string()),
                    Signal("gadfec".to_string()),
                    Signal("cb".to_string()),
                ],
            },
            NoteEntry {
                signal_patterns: [
                    Signal("aecbfdg".to_string()),
                    Signal("fbg".to_string()),
                    Signal("gf".to_string()),
                    Signal("bafeg".to_string()),
                    Signal("dbefa".to_string()),
                    Signal("fcge".to_string()),
                    Signal("gcbea".to_string()),
                    Signal("fcaegb".to_string()),
                    Signal("dgceab".to_string()),
                    Signal("fcbdga".to_string()),
                ],
                output_value: [
                    Signal("gecf".to_string()),
                    Signal("egdcabf".to_string()),
                    Signal("bgf".to_string()),
                    Signal("bfgea".to_string()),
                ],
            },
            NoteEntry {
                signal_patterns: [
                    Signal("fgeab".to_string()),
                    Signal("ca".to_string()),
                    Signal("afcebg".to_string()),
                    Signal("bdacfeg".to_string()),
                    Signal("cfaedg".to_string()),
                    Signal("gcfdb".to_string()),
                    Signal("baec".to_string()),
                    Signal("bfadeg".to_string()),
                    Signal("bafgc".to_string()),
                    Signal("acf".to_string()),
                ],
                output_value: [
                    Signal("gebdcfa".to_string()),
                    Signal("ecba".to_string()),
                    Signal("ca".to_string()),
                    Signal("fadegcb".to_string()),
                ],
            },
            NoteEntry {
                signal_patterns: [
                    Signal("dbcfg".to_string()),
                    Signal("fgd".to_string()),
                    Signal("bdegcaf".to_string()),
                    Signal("fgec".to_string()),
                    Signal("aegbdf".to_string()),
                    Signal("ecdfab".to_string()),
                    Signal("fbedc".to_string()),
                    Signal("dacgb".to_string()),
                    Signal("gdcebf".to_string()),
                    Signal("gf".to_string()),
                ],
                output_value: [
                    Signal("cefg".to_string()),
                    Signal("dcbef".to_string()),
                    Signal("fcge".to_string()),
                    Signal("gbcadfe".to_string()),
                ],
            },
            NoteEntry {
                signal_patterns: [
                    Signal("bdfegc".to_string()),
                    Signal("cbegaf".to_string()),
                    Signal("gecbf".to_string()),
                    Signal("dfcage".to_string()),
                    Signal("bdacg".to_string()),
                    Signal("ed".to_string()),
                    Signal("bedf".to_string()),
                    Signal("ced".to_string()),
                    Signal("adcbefg".to_string()),
                    Signal("gebcd".to_string()),
                ],
                output_value: [
                    Signal("ed".to_string()),
                    Signal("bcgafe".to_string()),
                    Signal("cdgba".to_string()),
                    Signal("cbgef".to_string()),
                ],
            },
            NoteEntry {
                signal_patterns: [
                    Signal("egadfb".to_string()),
                    Signal("cdbfeg".to_string()),
                    Signal("cegd".to_string()),
                    Signal("fecab".to_string()),
                    Signal("cgb".to_string()),
                    Signal("gbdefca".to_string()),
                    Signal("cg".to_string()),
                    Signal("fgcdab".to_string()),
                    Signal("egfdb".to_string()),
                    Signal("bfceg".to_string()),
                ],
                output_value: [
                    Signal("gbdfcae".to_string()),
                    Signal("bgc".to_string()),
                    Signal("cg".to_string()),
                    Signal("cgb".to_string()),
                ],
            },
            NoteEntry {
                signal_patterns: [
                    Signal("gcafb".to_string()),
                    Signal("gcf".to_string()),
                    Signal("dcaebfg".to_string()),
                    Signal("ecagb".to_string()),
                    Signal("gf".to_string()),
                    Signal("abcdeg".to_string()),
                    Signal("gaef".to_string()),
                    Signal("cafbge".to_string()),
                    Signal("fdbac".to_string()),
                    Signal("fegbdc".to_string()),
                ],
                output_value: [
                    Signal("fgae".to_string()),
                    Signal("cfgab".to_string()),
                    Signal("fg".to_string()),
                    Signal("bagce".to_string()),
                ],
            },
        ]
    }

    #[test]
    fn test_part1() {
        let result = part1(&input());
        assert_eq!(result, 26);
    }
}
