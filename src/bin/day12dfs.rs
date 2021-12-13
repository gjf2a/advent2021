use std::cmp::Ordering;
use std::io;
use advent_code_lib::{AdjacencySets, all_lines, generic_main, ParentMapQueue, search, SearchQueue};
use hash_histogram::HashHistogram;

// NOTE:
// * No big cave is ever connected to another big cave!
// * If 2 big caves were connected, you could bounce between them indefinitely, leading
//   to an infinite number of paths.

const START: &'static str = "start";
const END: &'static str = "end";
const SHOW_PATH_ARG: &'static str = "-show-paths";

fn main() -> io::Result<()> {
    generic_main("day12", &["(1|2)"], &[SHOW_PATH_ARG], |args| {
        let graph = build_graph_from(args[1].as_str())?;
        let part = args[2].as_str();
        let paths = all_paths(&graph, if part == "1" {Rule::Part1} else {Rule::Part2});
        if args.len() >= 4 {show(&paths);}
        println!("Part {}: {}", part, paths.len());
        Ok(())
    })
}

#[derive(Debug, Clone, Eq, PartialEq, Ord)]
struct Path {
    path: Vec<String>
}

impl Path {
    fn start(starter: &str) -> Self {
        Path {path: vec![starter.to_string()]}
    }

    fn end(&self) -> &str {
        self.path.last().unwrap()
    }

    fn with(&self, new_end: &str) -> Self {
        let mut path = self.path.clone();
        path.push(new_end.to_string());
        Path {path}
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.path.len() == other.path.len() {
            for (p1, p2) in self.path.iter().zip(other.path.iter()) {
                match p1.partial_cmp(p2).unwrap() {
                    Ordering::Equal => {},
                    other => return Some(other)
                }
            }
            Some(Ordering::Equal)
        } else if self.path.len() > other.path.len() {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Less)
        }
    }
}

pub fn build_graph_from(filename: &str) -> io::Result<AdjacencySets> {
    let mut graph = AdjacencySets::new();
    for line in all_lines(filename)? {
        let parts: Vec<&str> = line.split('-').collect();
        graph.connect2(parts[0], parts[1]);
    }
    Ok(graph)
}

fn show(paths: &Vec<Path>) {
    for path in paths.iter() {
        println!("{:?}", path);
    }
}

fn all_paths(graph: &AdjacencySets, rule: Rule) -> Vec<Path> {
    let mut all_paths = Vec::new();
    let mut stack: ParentMapQueue<Path, Vec<Path>> = ParentMapQueue::new();
    stack.enqueue(&Path::start(START));
    search(stack, |node, stack| {
        if rule.allows(&node.path) {
            if node.end() == END {
                all_paths.push(node.clone());
            } else {
                for neighbor in graph.neighbors_of(node.end()).unwrap() {
                    stack.enqueue(&node.with(neighbor.as_str()));
                }
            }
        }
    });

    all_paths
}

#[derive(Copy, Clone, Debug)]
enum Rule {
    Part1, Part2
}

impl Rule {
    fn allows(&self, path: &Vec<String>) -> bool {
        let counts: HashHistogram<String> = path.iter().collect();
        let mut potential_problems = counts.iter()
            .filter(|(s, c)| !has_upper(s.as_str()) && **c > 1);
        match self {
            Rule::Part1 => potential_problems.next().is_none(),
            Rule::Part2 => {
                let problems = potential_problems.collect::<Vec<_>>();
                problems.iter().find(|(s, _)| [START, END].contains(&s.as_str())).is_none()
                    && !problems.iter().any(|(_, count)| **count > 2)
                    && problems.len() <= 1
            }
        }
    }
}

pub fn has_upper(s: &str) -> bool {
    s.chars().any(|c| c.is_uppercase())
}

#[cfg(test)]
mod tests {
    use crate::Rule;

    #[test]
    fn test() {
        for disallowed in [["start", "b", "d", "b", "d"]].iter() {
            let path = disallowed.iter().map(|s| s.to_string()).collect();
            assert!(!Rule::Part2.allows(&path));
        }
    }
}