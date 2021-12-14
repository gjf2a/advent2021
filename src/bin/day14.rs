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
    chain: String,
    rules: HashMap<(char,char),char>
}

impl PolymerIterator {
    fn new(filename: &str) -> io::Result<Self> {
        let mut lines = all_lines(filename)?;
        let chain = lines.next().unwrap();
        lines.next();
        let rules = lines.map(|line| {
            let mut parts = line.split(" -> ");
            (key_from(parts.next().unwrap()), value_from(parts.next().unwrap()))
        }).collect();
        Ok(PolymerIterator {chain, rules})
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
        let result = self.chain.clone();
        self.chain = format!("{}{}", self.chain.chars().next().unwrap(), self.chain.chars()
            .zip(self.chain.chars().skip(1))
            .map(|(a, b)| combine(*self.rules.get(&(a, b)).unwrap(), b))
            .collect::<String>());
        Some(score_for(result))
    }
}

fn score_for(chain: String) -> usize {
    let histogram: HashHistogram<char> = chain.chars().collect();
    let ranked = histogram.ranking();
    histogram.count(&ranked[0]) - histogram.count(&ranked[ranked.len() - 1])
}

fn combine(a: char, b: char) -> String {
    format!("{}{}", a, b)
}

#[cfg(test)]
mod tests {
    use crate::PolymerIterator;

    #[test]
    fn test_example_1() {
        let polymer = PolymerIterator::new("ex/day14.txt").unwrap();
        assert_eq!(polymer.skip(10).next().unwrap(), 1588);
    }
}