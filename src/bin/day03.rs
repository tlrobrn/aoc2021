#![warn(clippy::all, clippy::pedantic)]
use aoc2021::input_lines;

fn main() {
    let binary_numbers: Vec<String> = input_lines().map(|line| line.trim().to_string()).collect();
    let result = part1(&binary_numbers);
    println!("Part 1: {}", result);
    let result = part2(&binary_numbers);
    println!("Part 2: {}", result);
}

fn part1(binary_numbers: &[String]) -> u64 {
    let (zero_counts, one_counts) = counts(binary_numbers);
    gamma_rate(&zero_counts, &one_counts) * epsilon_rate(&zero_counts, &one_counts)
}

fn part2(binary_numbers: &[String]) -> u64 {
    oxygen_rate(binary_numbers, 0) * co2_rate(binary_numbers, 0)
}

fn counts(binary_numbers: &[String]) -> (Vec<u64>, Vec<u64>) {
    let num_digits = binary_numbers[0].len();
    let mut zero_counts: Vec<u64> = vec![0; num_digits];
    let mut one_counts: Vec<u64> = vec![0; num_digits];

    for binary_number in binary_numbers {
        for (i, digit) in binary_number.chars().enumerate() {
            match digit {
                '0' => zero_counts[i] += 1,
                '1' => one_counts[i] += 1,
                _ => {}
            }
        }
    }
    (zero_counts, one_counts)
}

fn gamma_rate(zero_counts: &[u64], one_counts: &[u64]) -> u64 {
    let mut gamma = String::with_capacity(zero_counts.len());
    for (zeros, ones) in zero_counts.iter().zip(one_counts.iter()) {
        if zeros > ones {
            gamma.push('0');
        } else {
            gamma.push('1');
        }
    }
    u64::from_str_radix(&gamma, 2).unwrap_or_default()
}

fn epsilon_rate(zero_counts: &[u64], one_counts: &[u64]) -> u64 {
    let mut epsilon = String::with_capacity(zero_counts.len());
    for (zeros, ones) in zero_counts.iter().zip(one_counts.iter()) {
        if zeros < ones {
            epsilon.push('0');
        } else {
            epsilon.push('1');
        }
    }
    u64::from_str_radix(&epsilon, 2).unwrap_or_default()
}

fn oxygen_rate(binary_numbers: &[String], idx: usize) -> u64 {
    if binary_numbers.len() == 1 {
        return u64::from_str_radix(&binary_numbers[0], 2).unwrap_or_default();
    }

    let (zero_counts, one_counts) = counts(binary_numbers);
    let binary_numbers: Vec<String> = binary_numbers
        .iter()
        .cloned()
        .filter(|number| {
            let number: Vec<char> = number.chars().take(idx + 1).collect();
            if zero_counts[idx] > one_counts[idx] {
                number[idx] == '0'
            } else {
                number[idx] == '1'
            }
        })
        .collect();
    oxygen_rate(&binary_numbers, idx + 1)
}

fn co2_rate(binary_numbers: &[String], idx: usize) -> u64 {
    if binary_numbers.len() == 1 {
        return u64::from_str_radix(&binary_numbers[0], 2).unwrap_or_default();
    }

    let (zero_counts, one_counts) = counts(binary_numbers);
    let binary_numbers: Vec<String> = binary_numbers
        .iter()
        .cloned()
        .filter(|number| {
            let number: Vec<char> = number.chars().take(idx + 1).collect();
            if zero_counts[idx] <= one_counts[idx] {
                number[idx] == '0'
            } else {
                number[idx] == '1'
            }
        })
        .collect();
    co2_rate(&binary_numbers, idx + 1)
}

#[cfg(test)]
mod day3_tests {
    use super::*;

    const EXAMPLE_INPUT: [&str; 12] = [
        "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001",
        "00010", "01010",
    ];

    #[test]
    fn part1_example() {
        let result = part1(&EXAMPLE_INPUT.map(|line| line.to_string()));
        assert_eq!(result, 198);
    }

    #[test]
    fn part2_example() {
        let result = part2(&EXAMPLE_INPUT.map(|line| line.to_string()));
        assert_eq!(result, 230);
    }
}
