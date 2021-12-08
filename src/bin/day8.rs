use std::{env, io};
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
}

fn snag_put(part: &str) -> Vec<String> {
    part.split_whitespace().map(|s| s.to_owned()).collect()
}

fn solve_part_1(entries: &Vec<DeviceEntry>) -> usize {
    let easy_lengths: Vec<usize> = PATTERN_FOR.iter()
        .enumerate()
        .filter(|(i, _)| [1, 4, 7, 8].contains(i))
        .map(|(_, s)| s.len())
        .collect();
    let lengths = PATTERN_FOR.iter().map(|s| s.len()).collect::<Vec<_>>();
    entries.iter()
        .map(|entry| entry.outputs.iter()
            .map(|output| output.len())
            .filter(|output_len| easy_lengths.contains(output_len))
            .map(|output_len| lengths[output_len])
            .count())
        .sum()
}