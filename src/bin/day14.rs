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
    let result = polymer.skip(num_steps).next().unwrap();
    let histogram: HashHistogram<char> = result.chars().collect();
    let ranked = histogram.ranking();
    histogram.count(&ranked[0]) - histogram.count(&ranked[ranked.len() - 1])
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
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.chain.clone();
        self.chain = format!("{}{}", self.chain.chars().next().unwrap(), self.chain.chars()
            .zip(self.chain.chars().skip(1))
            .map(|(a, b)| combine(*self.rules.get(&(a, b)).unwrap(), b))
            .collect::<String>());
        Some(result)
    }
}

fn combine(a: char, b: char) -> String {
    format!("{}{}", a, b)
}