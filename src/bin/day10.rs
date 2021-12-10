use std::io;
use advent_code_lib::{all_lines, generic_main};

fn main() -> io::Result<()> {
    generic_main("day10", &["(1|2)"], &[], |args| {
        let filename = args[1].as_str();
        let part = args[2].as_str();
        let score = match part {
            "1" => part_1(filename),
            "2" => part_2(filename),
            bad => panic!("Illegal option {}", bad)
        }?;
        println!("Part {} score: {}", part, score);
        Ok(())
    })
}

const OPENERS: [char; 4] = ['(', '[', '{', '<'];
const CLOSERS: [char; 4] = [')', ']', '}', '>'];
const PENALTIES: [usize; 4] = [3, 57, 1197, 25137];
const COMPLETION_MULTIPLIER: usize = CLOSERS.len() + 1;

fn part_1(filename: &str) -> io::Result<usize> {
    Ok(all_lines(filename)?
        .filter_map(|line| ParsedLine::from(line.as_str()).corruption())
        .map(|(_, _, penalty)| penalty)
        .sum())
}

fn part_2(filename: &str) -> io::Result<usize> {
    let mut scores: Vec<usize> = all_lines(filename)?
        .filter_map(|line| ParsedLine::from(line.as_str()).completion())
        .map(|completion| completion_score(completion.as_str()))
        .collect();
    scores.sort();
    Ok(scores[scores.len() / 2])
}

#[derive(Debug, Clone)]
enum ParsedLine {
    Corruption(char, char, usize),
    Completion(String)
}

impl ParsedLine {
    fn from(line: &str) -> Self {
        let mut stack = Vec::new();
        for c in line.chars() {
            if OPENERS.contains(&c) {
                stack.push(c);
            } else {
                let popped = stack.pop().unwrap();
                let expected = CLOSERS[index_of(popped, OPENERS.iter())];
                if c != expected {
                    let penalty = PENALTIES[index_of(c, CLOSERS.iter())];
                    return ParsedLine::Corruption(expected, c, penalty);
                }
            }
        }
        ParsedLine::Completion(completion_of(stack))
    }

    // It would be great to auto-generate these methods.
    // This crate is along those lines, but not quite what I need:
    // https://github.com/alekratz/enum-methods

    fn corruption(&self) -> Option<(char, char, usize)> {
        match self {
            ParsedLine::Corruption(ex, ac, p) => Some((*ex, *ac, *p)),
            ParsedLine::Completion(_) => None
        }
    }

    fn completion(&self) -> Option<String> {
        match self {
            ParsedLine::Corruption(_, _, _) => None,
            ParsedLine::Completion(s) => Some(s.clone())
        }
    }
}

fn completion_of(mut stack: Vec<char>) -> String {
    let mut completion = String::new();
    loop {
        match stack.pop() {
            None => return completion,
            Some(popped) => {
                completion.push(CLOSERS[index_of(popped, OPENERS.iter())]);
            }
        }
    }
}

fn completion_score(completion: &str) -> usize {
    let mut result = 0;
    for c in completion.chars() {
        result *= COMPLETION_MULTIPLIER;
        result += 1 + index_of(c, CLOSERS.iter());
    }
    result
}

fn index_of<'a, I: Iterator<Item=&'a char>>(value: char, mut items: I) -> usize {
    items.position(|item| *item == value).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_corruption() {
        for (line, outcome) in [("{([(<{}[<>[]}>{[]{[(<()>", Some((']', '}')))] {
            match line_analysis(line) {
                ParsedLine::Completion(_) => {assert_eq!(outcome, None);}
                ParsedLine::Corruption(expected, actual, penalty) => {
                    let (outcome_expected, outcome_actual) = outcome.unwrap();
                    let outcome_penalty = PENALTIES[index_of(outcome_actual, CLOSERS.iter())];
                    assert_eq!(expected, outcome_expected);
                    assert_eq!(actual, outcome_actual);
                    assert_eq!(penalty, outcome_penalty);
                }
            }
        }
    }
}