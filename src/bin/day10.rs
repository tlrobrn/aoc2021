#![warn(clippy::all, clippy::pedantic)]
use aoc2021::input_lines;
use aoc2021::stack::Stack;
use std::collections::HashSet;

fn main() {
    let input: Vec<String> = input_lines().map(|line| line.trim().to_string()).collect();
    let result = part1(&input);
    println!("Part 1: {}", result);
    let result = part2(&input);
    println!("Part 2: {}", result);
}

fn part1(lines: &[String]) -> usize {
    lines
        .iter()
        .map(|line| parse_line(line))
        .map(|result| result.syntax_error_score())
        .sum()
}

fn part2(lines: &[String]) -> usize {
    let mut scores: Vec<usize> = lines
        .iter()
        .map(|line| parse_line(line))
        .filter(ParseResult::is_incomplete)
        .map(|result| result.completion_score())
        .collect();
    scores.sort_unstable();
    scores[scores.len() / 2]
}

struct ParseResult {
    stack: Stack<char>,
    corrupt_token: Option<char>,
}

impl ParseResult {
    fn is_incomplete(&self) -> bool {
        self.corrupt_token.is_none() && self.stack.size() > 0
    }

    fn syntax_error_score(&self) -> usize {
        match self.corrupt_token {
            Some(')') => 3,
            Some(']') => 57,
            Some('}') => 1197,
            Some('>') => 25137,
            _ => 0,
        }
    }

    fn completion_score(&self) -> usize {
        closing_tokens_for_result(self.stack.clone())
            .iter()
            .map(|&token| completion_score_for_token(token))
            .fold(0, |total, token_score| total * 5 + token_score)
    }
}

fn completion_score_for_token(token: char) -> usize {
    match token {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => 0,
    }
}

fn closing_tokens_for_result(mut stack: Stack<char>) -> Vec<char> {
    let mut completion_tokens: Vec<char> = vec![];
    while let Some(token) = stack.pop() {
        let closing_token = closing_tag_for(token);
        completion_tokens.push(closing_token);
    }
    completion_tokens
}

fn parse_line(line: &str) -> ParseResult {
    let opening_tags = all_opening_tags();
    let mut stack: Stack<char> = Stack::default();
    for token in line.chars() {
        if opening_tags.contains(&token) {
            stack.push(token);
        } else {
            let expected_tag = opening_tag_for(token);
            match stack.pop() {
                Some(tag) if tag == expected_tag => (),
                _ => {
                    return ParseResult {
                        stack,
                        corrupt_token: Some(token),
                    };
                }
            }
        }
    }
    ParseResult {
        stack,
        corrupt_token: None,
    }
}

fn all_opening_tags() -> HashSet<char> {
    HashSet::from(['(', '[', '{', '<'])
}

fn opening_tag_for(token: char) -> char {
    match token {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        _ => panic!("not a valid token"),
    }
}

fn closing_tag_for(token: char) -> char {
    match token {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("not a valid token"),
    }
}

#[cfg(test)]
mod day10_tests {
    use super::*;

    fn input() -> Vec<String> {
        vec![
            "[({(<(())[]>[[{[]{<()<>>".to_string(),
            "[(()[<>])]({[<{<<[]>>(".to_string(),
            "{([(<{}[<>[]}>{[]{[(<()>".to_string(),
            "(((({<>}<{<{<>}{[]{[]{}".to_string(),
            "[[<[([]))<([[{}[[()]]]".to_string(),
            "[{[{({}]{}}([{[{{{}}([]".to_string(),
            "{<[[]]>}<{[{[{[]{()[[[]".to_string(),
            "[<(<(<(<{}))><([]([]()".to_string(),
            "<{([([[(<>()){}]>(<<{{".to_string(),
            "<{([{{}}[<[[[<>{}]]]>[]]".to_string(),
        ]
    }

    #[test]
    fn test_part1() {
        let result = part1(&input());
        assert_eq!(26397, result);
    }

    #[test]
    fn test_part2() {
        let result = part2(&input());
        assert_eq!(288957, result);
    }
}
