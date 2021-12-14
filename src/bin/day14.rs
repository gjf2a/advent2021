use std::collections::HashMap;
use std::io;
use advent_code_lib::{all_lines, generic_main};
use hash_histogram::HashHistogram;

fn main() -> io::Result<()> {
    generic_main("day13", &[], &[], |args| {
        let polymer = PolymerIterator::new(args[1].as_str())?;
        println!("Part 1 score: {}", score_after(&polymer, 10));
        println!("Part 2 score: {}", score_after(&polymer, 40));
        Ok(())
    })
}

fn score_after(polymer: &PolymerIterator, num_steps: usize) -> usize {
    let polymer = polymer.clone();
    polymer.skip(num_steps).next().unwrap()
}

#[derive(Debug, Clone)]
struct PolymerIterator {
    state: HashHistogram<(char,char)>,
    final_letter: char,
    rules: HashMap<(char,char), char>
}

impl PolymerIterator {
    fn new(filename: &str) -> io::Result<Self> {
        let mut lines = all_lines(filename)?;
        let first_line = lines.next().unwrap();
        let pairs: Vec<(char, char)> = first_line.chars()
            .zip(first_line.chars().skip(1))
            .map(|(a, b)| (a, b))
            .collect();
        let state = pairs.iter().collect();
        let final_letter = pairs.last().unwrap().1;

        lines.next();
        let rules = lines.map(|line| {
            let mut parts = line.split(" -> ");
            (key_from(parts.next().unwrap()), value_from(parts.next().unwrap()))
        }).collect();
        Ok(PolymerIterator {state, final_letter, rules})
    }
}

fn key_from(key_str: &str) -> (char, char) {
    let mut key_iter = key_str.chars();
    (key_iter.next().unwrap(), key_iter.next().unwrap())
}

fn value_from(value_str: &str) -> char {
    value_str.chars().next().unwrap()
}

impl Iterator for PolymerIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let result = score_histogram(&self.state, self.final_letter);
        let mut updated_state = HashHistogram::new();
        for ((a, c), count) in self.state.iter() {
            let b = self.rules.get(&(*a, *c)).unwrap();
            updated_state.bump_by(&(*a, *b), *count);
            updated_state.bump_by(&(*b, *c), *count);
        }
        assert_eq!(self.state.total_count() * 2, updated_state.total_count());
        self.state = updated_state;
        Some(result)
    }
}

fn score_histogram(pair_counts: &HashHistogram<(char,char)>, final_letter: char) -> usize {
    let mut histogram = HashHistogram::new();
    for (pair, count) in pair_counts.iter() {
        histogram.bump_by(&pair.0, *count);
    }
    histogram.bump(&final_letter);
    assert_eq!(histogram.total_count(), pair_counts.total_count() + 1);
    let ranked = histogram.ranking();
    let score = histogram.count(&ranked[0]) - histogram.count(&ranked[ranked.len() - 1]);
    score
}

#[cfg(test)]
mod tests {
    use crate::PolymerIterator;

    #[test]
    fn test_example_1() {
        for (skip, count) in [(0, 1), (1, 1), (2, 5), (10, 1588)].iter().copied() {
            let polymer = PolymerIterator::new("ex/day14.txt").unwrap();
            assert_eq!(polymer.skip(skip).next().unwrap(), count);
        }
    }
}

// NNCB - NN: 1, NC: 1, CB: 1
// Final pair: CB
// 2 start w/N, 1 starts w/C, then add Bs from final pair
// N: 2, C: 1, B: 1

// NCNBCHB - NC: 1, CN: 1, NB: 1, BC: 1, CH: 1, HB: 1
// Final pair: HB
// 2 start w/N, 2 start w/C, 1 starts w/B, 1 starts w/H
// N: 2, C: 2, B: 1 + 1 = 2, H: 1

// NBCCNBBBCBHCB: NB: 2, BC: 2, CC: 1, CN: 1, BB: 2, CB: 2, BH: 1, HC: 1