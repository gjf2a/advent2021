use std::collections::HashMap;
use std::io;
use advent_code_lib::{all_lines, generic_main};
use hash_histogram::HashHistogram;

fn main() -> io::Result<()> {
    generic_main("day13", &[], &[], |args| {
        let polymer = PolymerIterator::new(args[1].as_str())?;
        println!("Part 1 score: {}", score_after(&polymer, 10));
        //println!("Part 2 score: {}", score_after(&polymer, 40));
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
    final_pair: (char, char),
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
        let final_pair = *pairs.last().unwrap();

        lines.next();
        let rules = lines.map(|line| {
            let mut parts = line.split(" -> ");
            (key_from(parts.next().unwrap()), value_from(parts.next().unwrap()))
        }).collect();
        Ok(PolymerIterator {state, final_pair, rules})
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
        let result = score_histogram(&self.state, self.final_pair);
        let mut updated_state = HashHistogram::new();
        for ((a, c), _) in self.state.iter() {
            let b = self.rules.get(&(*a, *c)).unwrap();
            updated_state.bump(&(*a, *b));
            updated_state.bump(&(*b, *c));
            if (*a, *c) == self.final_pair {
                self.final_pair = (*b, *c);
            }
        }
        self.state = updated_state;
        Some(result)
    }
}

fn score_histogram(pair_counts: &HashHistogram<(char,char)>, final_pair: (char, char)) -> usize {
    let mut histogram: HashHistogram<char> = pair_counts.iter().map(|((a, _), _)| *a).collect();
    histogram.bump(&final_pair.1);
    let ranked = histogram.ranking();
    histogram.count(&ranked[0]) - histogram.count(&ranked[ranked.len() - 1])
}

#[cfg(test)]
mod tests {
    use crate::PolymerIterator;

    #[test]
    fn test_example_1() {
        let polymer = PolymerIterator::new("ex/day14.txt").unwrap();
        assert_eq!(polymer.skip(10).next().unwrap(), 1588);
    }

    fn num_pairs_after(start_size: usize, num_steps: usize) -> usize {
        let mut size = start_size;
        for _ in 0..num_steps {
            size = size * 2 - 1
        }
        size
    }

    #[test]
    fn ideas() {

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