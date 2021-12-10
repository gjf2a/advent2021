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
        .filter_map(|line| line_analysis(line.as_str()).corruption())
        .map(|(expected, actual, penalty)| penalty)
        .sum())
}

fn part_2(filename: &str) -> io::Result<usize> {
    let mut scores: Vec<usize> = all_lines(filename)?
        .filter_map(|line| line_analysis(line.as_str()).completion())
        .map(|completion| completion_score(completion.as_str()))
        .collect();
    scores.sort();
    Ok(scores[scores.len() / 2])
}

#[derive(Debug, Clone)]
enum AnalyzedLine {
    Corrupt(char, char, usize),
    Completion(String)
}

impl AnalyzedLine {
    fn corruption(&self) -> Option<(char, char, usize)> {
        match self {
            AnalyzedLine::Corrupt(ex, ac, p) => Option::Some((*ex, *ac, *p)),
            AnalyzedLine::Completion(_) => None
        }
    }

    fn completion(&self) -> Option<String> {
        match self {
            AnalyzedLine::Corrupt(_, _, _) => None,
            AnalyzedLine::Completion(s) => Some(s.clone())
        }
    }
}

fn line_analysis(line: &str) -> AnalyzedLine {
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
                return AnalyzedLine::Corrupt(expected, c, penalty);
            }
        }
    }
    let mut completion = String::new();
    loop {
        match stack.pop() {
            None => return AnalyzedLine::Completion(completion),
            Some(popped) => {
                completion.push(CLOSERS[OPENERS.iter().position(|op| *op == popped).unwrap()]);
            }
        }
    }
}

fn completion_score(completion: &str) -> usize {
    let mut result = 0;
    for c in completion.chars() {
        result *= 5;
        result += 1 + CLOSERS.iter().position(|cl| *cl == c).unwrap();
    }
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
            match line_analysis(line) {
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