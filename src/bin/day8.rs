use std::{env, io};
use std::collections::HashMap;
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
    outputs: Vec<String>,
    easy_lengths: Vec<usize>
}

impl DeviceEntry {
    fn from(line: &str) -> Self {
        let mut parts = line.split('|');
        let inputs = snag_put(parts.next().unwrap());
        let outputs = snag_put(parts.next().unwrap());
        let easy_lengths = PATTERN_FOR.iter()
            .enumerate()
            .filter(|(i, _)| [1, 4, 7, 8].contains(i))
            .map(|(_, s)| s.len())
            .collect();
        DeviceEntry {inputs, outputs, easy_lengths}
    }

    fn easy_for(&self, entries: &Vec<String>) -> Vec<String> {
        entries.iter()
            .filter(|s| self.easy_lengths.contains(&s.len()))
            .cloned()
            .collect()
    }

    fn find_mapping(&self) -> HashMap<char, char> {
        let mut result = HashMap::new();
        let starting_points = self.easy_for(&self.inputs);
        result
    }
}

fn snag_put(part: &str) -> Vec<String> {
    part.split_whitespace().map(|s| s.to_owned()).collect()
}

fn solve_part_1(entries: &Vec<DeviceEntry>) -> usize {
    let patterns_by_lengths = by_lengths(&PATTERN_FOR);
    entries.iter()
        .map(|entry| entry.easy_for(&entry.outputs).len())
        .sum()
}

fn by_lengths(strs: &[&str]) -> HashMap<usize, Vec<String>> {
    let mut result = HashMap::new();
    for s in strs {
        match result.get_mut(&s.len()) {
            None => {result.insert(s.len(), vec![s.to_string()])}
            Some(v) => {v.push(s.to_string())}
        }
    }
    result
}

fn solve_part_2(entries: &Vec<DeviceEntry>) -> usize {
    0
}