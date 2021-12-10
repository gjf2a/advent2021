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
        .filter_map(|line| ParsedLine::from(line.as_str()).corruption_score())
        .sum())
}

fn part_2(filename: &str) -> io::Result<usize> {
    let mut scores: Vec<usize> = all_lines(filename)?
        .filter_map(|line| ParsedLine::from(line.as_str()).completion_score())
        .collect();
    scores.sort();
    Ok(scores[scores.len() / 2])
}

fn closer_for(opener: char) -> char {
    parallel_in(opener, &OPENERS, &CLOSERS)
}

fn penalty_for(closer: char) -> usize {
    parallel_in(closer, &CLOSERS, &PENALTIES)
}

fn parallel_in<V: Copy>(target: char, origin: &[char], destination: &[V]) -> V {
    destination[index_of(target, origin.iter())]
}

fn index_of<'a, I: Iterator<Item=&'a char>>(value: char, mut items: I) -> usize {
    items.position(|item| *item == value).unwrap()
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
                let expected = closer_for(popped);
                if c != expected {
                    return ParsedLine::Corruption(expected, c, penalty_for(c));
                }
            }
        }
        ParsedLine::Completion(ParsedLine::completion_of(stack))
    }

    fn completion_of(mut stack: Vec<char>) -> String {
        let mut completion = String::new();
        loop {
            match stack.pop() {
                None => return completion,
                Some(popped) => {
                    completion.push(closer_for(popped));
                }
            }
        }
    }

    fn corruption_score(&self) -> Option<usize> {
        match self {
            ParsedLine::Corruption(_, _, p) => Some(*p),
            ParsedLine::Completion(_) => None
        }
    }

    fn completion_score(&self) -> Option<usize> {
        match self {
            ParsedLine::Corruption(_, _, _) => None,
            ParsedLine::Completion(s) => {
                let mut result = 0;
                for c in s.chars() {
                    result *= COMPLETION_MULTIPLIER;
                    result += 1 + index_of(c, CLOSERS.iter());
                }
                Some(result)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_corruption() {
        for (line, outcome) in [("{([(<{}[<>[]}>{[]{[(<()>", Some((']', '}')))] {
            match ParsedLine::from(line) {
                ParsedLine::Completion(_) => {assert_eq!(outcome, None);}
                ParsedLine::Corruption(expected, actual, penalty) => {
                    let (outcome_expected, outcome_actual) = outcome.unwrap();
                    let outcome_penalty = penalty_for(outcome_actual);
                    assert_eq!(expected, outcome_expected);
                    assert_eq!(actual, outcome_actual);
                    assert_eq!(penalty, outcome_penalty);
                }
            }
        }
    }
}