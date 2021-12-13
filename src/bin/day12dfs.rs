use std::collections::VecDeque;
use std::io;
use advent_code_lib::{AdjacencySets, all_lines, generic_main, ParentMapQueue, search, SearchQueue};

// NOTE:
// * No big cave is ever connected to another big cave!
// * If 2 big caves were connected, you could bounce between them indefinitely, leading
//   to an infinite number of paths.

const START: &'static str = "start";
const END: &'static str = "end";

fn main() -> io::Result<()> {
    generic_main("day12", &["(1|2)"], &[], |args| {
        let graph = build_graph_from(args[1].as_str())?;
        let part = args[2].as_str();
        println!("Part {}: {}", part, num_paths(&graph, if part == "1" {Rule::Part1} else {Rule::Part2}));
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

fn num_paths(graph: &AdjacencySets, rule: Rule) -> usize {
    let mut all_paths = Vec::new();
    let mut stack: ParentMapQueue<(String,usize,usize, Option<String>), Vec<(String, usize,usize, Option<String>)>> = ParentMapQueue::new();
    stack.enqueue(&(START.to_string(), 1, 1, None));
    search(stack, |node: &(String, usize, usize, Option<String>), stack| {
        println!("dequeued: {:?}", node);
        let history = stack.path_back_from(&node);
        println!("history: {:?}", history);
        if rule.allows(&history) {
            println!("allowed");
            if node.0 == END {
                all_paths.push(history);
            } else {
                print!("enqueuing:");
                for neighbor in graph.neighbors_of(node.0.as_str()).unwrap() {
                    let updated_count = 1 + count_for(neighbor.as_str(), &history);
                    let node = (neighbor.clone(), updated_count, node.2 + 1, Some(node.0.clone()));
                    stack.enqueue(&node);
                    print!("{:?}", node);
                }
                println!();
            }
        } else {
            println!("pruned");
        }
    });
    for path in all_paths.iter() {
        println!("{:?}", path.iter().map(|(s, _, _, _)| format!("{},", s)).collect::<String>());
    }
    all_paths.len()
}

fn count_for(name: &str, history: &VecDeque<(String, usize, usize, Option<String>)>) -> usize {
    history.iter().find(|(s, _, _, _)| *s == name)
        .map_or(0, |(_, count, _, _)| *count)
}

#[derive(Copy, Clone, Debug)]
enum Rule {
    Part1, Part2
}

impl Rule {
    fn allows(&self, history: &VecDeque<(String, usize, usize, Option<String>)>) -> bool {
        let mut potential_problems = history.iter()
            .filter(|(s, count, _, _)| !has_upper(s.as_str()) && *count > 1);
        match self {
            Rule::Part1 => potential_problems.next().is_none(),
            Rule::Part2 => potential_problems.find(|(s, _, _, _)|
                    [START, END].contains(&s.as_str())).as_ref().is_none()
                    && potential_problems.count() <= 1
        }
    }
}

pub fn has_upper(s: &str) -> bool {
    s.chars().any(|c| c.is_uppercase())
}