use std::io;
use advent_code_lib::{AdjacencySets, all_lines, Arena, generic_main, ParentMapQueue, search, SearchQueue};
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
        let paths = all_paths(&graph, if part == "2" {Rule::Part2} else {Rule::Part1});
        if args.len() >= 4 {show(&paths);}
        println!("Part {}: {}", part, paths.len());
        Ok(())
    })
}

pub fn build_graph_from(filename: &str) -> io::Result<AdjacencySets> {
    let mut graph = AdjacencySets::new();
    for line in all_lines(filename)? {
        let parts: Vec<&str> = line.split('-').collect();
        graph.connect2(parts[0], parts[1]);
    }
    Ok(graph)
}

pub fn has_upper(s: &str) -> bool {
    s.chars().any(|c| c.is_uppercase())
}

fn show(paths: &Vec<Vec<String>>) {
    for path in paths.iter() {
        println!("{:?}", path);
    }
}

fn all_paths(graph: &AdjacencySets, rule: Rule) -> Vec<Vec<String>> {
    let mut all_paths = Vec::new();
    let mut arena = Arena::new();
    let mut stack: ParentMapQueue<(usize, Option<usize>), Vec<(usize, Option<usize>)>> = ParentMapQueue::new();
    stack.enqueue(&(arena.alloc(START.to_string(), None), None));
    search(stack, |(node, parent), stack| {
        let last_name = arena.get(*node).get().as_str();
        if rule.allows(&arena, last_name, *parent) {
            if last_name == END {
                all_paths.push(path_at_addr(&arena, *node));
            } else {
                for neighbor in graph.neighbors_of(last_name).unwrap() {
                    let new_addr = arena.alloc(neighbor.clone(), Some(*node));
                    stack.enqueue(&(new_addr, Some(*node)));
                }
            }
        }
    });
    all_paths
}

fn path_at_addr(arena: &Arena<String>, addr: usize) -> Vec<String> {
    let mut path = arena.iter_from(addr).collect::<Vec<_>>();
    path.reverse();
    path.iter().map(|s| (*s).clone()).collect()
}

#[derive(Copy, Clone, Debug)]
enum Rule {
    Part1, Part2
}

impl Rule {
    fn allows(&self, arena: &Arena<String>, node: &str, parent: Option<usize>) -> bool {
        match parent {
            None => true,
            Some(parent_addr) => {
                let mut path_counts: HashHistogram<String> = arena.iter_from(parent_addr).collect();
                path_counts.bump(&node.to_string());
                let mut potential_problems = path_counts.iter()
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
    }
}