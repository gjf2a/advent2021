use std::{env, io};
use std::collections::{HashMap, HashSet};
use advent_code_lib::all_lines;

const PATTERN_FOR: [&'static str; 10] = ["abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg"];

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: day8 filename");
    } else {
        let entries: Vec<DeviceEntry> = all_lines(args[1].as_str())?
            .map(|s| DeviceEntry::from(s.as_str()))
            .collect();
        println!("Part 1: {}", solve_part_1(&entries));
        println!("Part 2: {}", solve_part_2(&entries));
    }
    Ok(())
}

fn solve_part_1(entries: &Vec<DeviceEntry>) -> usize {
    let easy_lengths: HashSet<usize> = find_easy_lengths(&PATTERN_FOR);
    entries.iter()
        .map(|entry| entry.outputs.iter()
            .filter(|output| easy_lengths.contains(&output.len()))
            .count())
        .sum()
}

fn solve_part_2(entries: &Vec<DeviceEntry>) -> usize {
    entries.iter().map(|entry| entry.output_value()).sum()
}

#[derive(Debug)]
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

    fn output_value(&self) -> usize {
        let mut total = 0;
        for digit in self.output_digits(&self.find_mapping()) {
            total *= 10;
            total += digit;
        }
        total
    }

    fn find_mapping(&self) -> HashMap<char, char> {
        let mut result = HashMap::new();
        let lengths2charsets = char_sets_by_lengths(&self.inputs);
        sanity_check(&lengths2charsets);

        result.insert(find_a(&lengths2charsets), 'a');
        for (c, lengths) in [
            ('g', vec![5, 6]),
            ('d', vec![5]),
            ('f', vec![2, 6]),
            ('c', vec![2]),
            ('b', vec![4]),
            ('e', vec![7])
        ] {
            let found = finalize(&result, &mut intersection(set_chain(&lengths, &lengths2charsets)));
            result.insert(found, c);
        }
        result
    }

    fn output_digits<'a>(&'a self, mapping: &'a HashMap<char, char>) -> impl Iterator<Item=usize> + 'a {
        self.outputs.iter().map(|output| decode(output.as_str(), mapping))
    }
}

fn sanity_check(lengths2charsets: &HashMap<usize, Vec<HashSet<char>>>) {
    for (length, count) in [(2, 1), (3, 1), (4, 1), (5, 3), (6, 3), (7, 1)] {
        assert_eq!(lengths2charsets.get(&length).unwrap().len(), count);
    }
}

fn find_a(lengths2charsets: &HashMap<usize, Vec<HashSet<char>>>) -> char {
    let one = singleton(lengths2charsets, 2);
    let mut seven = singleton(lengths2charsets, 3).clone();
    seven.retain(|c| !one.contains(c));
    retrieve_singleton(&seven)
}

fn decode(output: &str, mapping: &HashMap<char, char>) -> usize {
    let mut chars: Vec<char> = output.chars().map(|c| mapping.get(&c).unwrap()).copied().collect();
    chars.sort();
    let decoded: String = chars.iter().collect();
    PATTERN_FOR.iter().enumerate().find(|(_, s)| decoded.as_str() == **s).map(|(i, _)| i).unwrap()
}

fn set_chain<'a>(lengths: &'a [usize], lengths2charsets: &'a HashMap<usize, Vec<HashSet<char>>>)
                 -> impl Iterator<Item=&'a HashSet<char>> {
    lengths.iter()
        .flat_map(|length| lengths2charsets.get(length).unwrap().iter())
}

fn intersection<'a, I: Iterator<Item=&'a HashSet<char>>>(mut sets: I) -> HashSet<char> {
    let mut result = sets.next().unwrap().clone();
    for set in sets {
        result.retain(|c| set.contains(c));
    }
    result
}

fn finalize(known: &HashMap<char, char>, inter: &mut HashSet<char>) -> char {
    for c in known.keys() {
        inter.remove(c);
    }
    retrieve_singleton(&inter)
}

fn retrieve_singleton(inter: &HashSet<char>) -> char {
    assert_eq!(inter.len(), 1);
    inter.iter().copied().next().unwrap()
}

fn singleton(lengths2charsets: &HashMap<usize, Vec<HashSet<char>>>, length: usize) -> &HashSet<char> {
    &lengths2charsets.get(&length).unwrap()[0]
}

fn char_sets_by_lengths(inputs: &Vec<String>) -> HashMap<usize, Vec<HashSet<char>>> {
    by_lengths(inputs.iter().map(|s| s.as_str())).iter()
        .map(|(length, strs)|
            (*length, strs.iter()
                .map(|s| s.chars().collect::<HashSet<char>>())
                .collect()))
        .collect()
}

fn snag_put(part: &str) -> Vec<String> {
    part.split_whitespace().map(|s| s.to_owned()).collect()
}

fn find_easy_lengths(strs: &[&str]) -> HashSet<usize> {
    by_lengths(strs.iter().copied()).iter()
        .filter(|(_, patterns)| patterns.len() == 1)
        .map(|(len, _)| *len)
        .collect()
}

fn by_lengths<'a, I: Iterator<Item=&'a str>>(strs: I) -> HashMap<usize, Vec<String>> {
    let mut result = HashMap::new();
    for s in strs {
        match result.get_mut(&s.len()) {
            None => {result.insert(s.len(), vec![s.to_string()]);}
            Some(v) => {v.push(s.to_string());}
        }
    }
    result
}