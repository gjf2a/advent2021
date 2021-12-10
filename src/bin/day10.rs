use std::io;
use advent_code_lib::{all_lines, generic_main};

fn main() -> io::Result<()> {
    generic_main("day10", &["(1|2)"], &[], |args| {
        let filename = args[1].as_str();
        let part = args[2].as_str();
        let score = match part {
            "1" => {part_1(filename)}
            "2" => {part_2(filename)}
            bad => {panic!("Illegal option {}", bad);}
        }?;
        println!("Part {} score: {}", part, score);
        Ok(())
    })
}

const OPENERS: [char; 4] = ['(', '[', '{', '<'];
const CLOSERS: [char; 4] = [')', ']', '}', '>'];
const PENALTIES: [usize; 4] = [3, 57, 1197, 25137];

fn part_1(filename: &str) -> io::Result<usize> {
    Ok(all_lines(filename)?
        .filter_map(|line| corrupted_line(line.as_str()))
        .map(|(expected, actual, penalty)| penalty)
        .sum())
}

fn part_2(filename: &str) -> io::Result<usize> {
    let mut scores: Vec<usize> = all_lines(filename)?
        .filter(|line| corrupted_line(line.as_str()).is_none())
        .map(|line| completion_score(line_completion(line.as_str()).as_str()))
        .collect();
    scores.sort();
    Ok(scores[scores.len() / 2])
}

fn corrupted_line(line: &str) -> Option<(char, char, usize)> {
    let mut stack = Vec::new();
    for c in line.chars() {
        if OPENERS.contains(&c) {
            stack.push(c);
        } else {
            let popped = stack.pop().unwrap();
            let popped_i = OPENERS.iter().position(|opener| *opener == popped).unwrap();
            let expected = CLOSERS[popped_i];
            if c != expected {
                let penalty = PENALTIES[CLOSERS.iter().position(|closer| *closer == c).unwrap()];
                return Some((expected, c, penalty));
            }
        }
    }
    None
}

fn line_completion(incomplete: &str) -> String {
    let mut result = String::new();

    result
}

fn completion_score(completion: &str) -> usize {
    let mut result = 0;

    result
}

fn report_corruption(result: Option<(char,char,usize)>) {
    match result {
        None => {println!("Line not corrupt")}
        Some((expected, actual, penalty)) => {
            println!("Expected {}, but found {} instead. Penalty: {}", expected, actual, penalty);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_corruption() {
        for (line, outcome) in [("{([(<{}[<>[]}>{[]{[(<()>", Some((']', '}')))] {
            match corrupted_line(line) {
                None => {assert_eq!(outcome, None);}
                Some((expected, actual, penalty)) => {
                    let (outcome_expected, outcome_actual) = outcome.unwrap();
                    let outcome_penalty = PENALTIES[CLOSERS.iter().position(|c| *c == outcome_actual).unwrap()];
                    assert_eq!(expected, outcome_expected);
                    assert_eq!(actual, outcome_actual);
                    assert_eq!(penalty, outcome_penalty);
                }
            }
        }
    }
}