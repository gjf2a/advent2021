use std::{env, io};
use std::collections::{HashMap, HashSet};
use advent_code_lib::all_lines;

const PATTERN_FOR: [&'static str; 10] = ["abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg"];

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: day8 filename");
    } else {
        let entries = all_lines(args[1].as_str())?
            .map(|s| DeviceEntry::from(s.as_str()))
            .collect::<Vec<_>>();
        let part1 = solve_part_1(&entries);
        println!("Part 1: {}", part1);
        let part2 = solve_part_2(&entries);
        println!("Part 2: {}", part2);
    }
    Ok(())
}

struct DeviceEntry {
    inputs: Vec<String>,
    outputs: Vec<String>
}

impl DeviceEntry {
    fn from(line: &str) -> Self {
        let mut parts = line.split('|');
        let inputs = snag_put(parts.next().unwrap());
        let outputs = snag_put(parts.next().unwrap());
        DeviceEntry {inputs, outputs}
    }

    fn find_mapping(&self) -> HashMap<char, char> {
        let mut result = HashMap::new();
        result
    }
}

fn snag_put(part: &str) -> Vec<String> {
    part.split_whitespace().map(|s| s.to_owned()).collect()
}

fn solve_part_1(entries: &Vec<DeviceEntry>) -> usize {
    let patterns_by_lengths = by_lengths(&PATTERN_FOR);
    let easy_lengths: HashSet<usize> = patterns_by_lengths.iter()
        .filter(|(_, patterns)| patterns.len() == 1)
        .map(|(len, _)| *len)
        .collect();
    entries.iter()
        .map(|entry| entry.outputs.iter()
            .filter(|output| easy_lengths.contains(&output.len()))
            .count())
        .sum()
}

fn by_lengths(strs: &[&str]) -> HashMap<usize, Vec<String>> {
    let mut result = HashMap::new();
    for s in strs {
        match result.get_mut(&s.len()) {
            None => {result.insert(s.len(), vec![s.to_string()]);}
            Some(v) => {v.push(s.to_string());}
        }
    }
    result
}

fn solve_part_2(entries: &Vec<DeviceEntry>) -> usize {
    0
}