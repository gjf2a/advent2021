use std::collections::{BTreeMap, VecDeque};
use std::io;
use advent_code_lib::{AdjacencySets, all_lines, Arena, breadth_first_search, generic_main, path_back_from, search, SearchQueue};

// NOTE:
// * No big cave is ever connected to another big cave!
// * If 2 big caves were connected, you could bounce between them indefinitely, leading
//   to an infinite number of paths.

const START: &'static str = "start";
const END: &'static str = "end";

fn main() -> io::Result<()> {
    generic_main("day12", &[], &[], |args| {
        let graph = build_graph_from(args[1].as_str())?;
        let primary_map = bfs(START, &graph);
        for key in graph.keys() {
            let pm = bfs(key, &graph);
            let mut path = path_back_from(&key.to_string(), &primary_map);
            let mut path_suffix = path_back_from(&"end".to_string(), &pm);
            path_suffix.pop_front();
            loop {
                match path_suffix.pop_front() {
                    None => break,
                    Some(node) => path.push_back(node)
                }
            }
            println!("{:?}", path);
        }
        Ok(())
    })
}

fn build_graph_from(filename: &str) -> io::Result<AdjacencySets> {
    let mut graph = AdjacencySets::new();
    for line in all_lines(filename)? {
        let parts: Vec<&str> = line.split('-').collect();
        graph.connect2(parts[0], parts[1]);
    }
    Ok(graph)
}

fn bfs(node: &str, graph: &AdjacencySets) -> BTreeMap<String,Option<String>> {
    breadth_first_search(&node.to_string(),
                         |node, q| graph.neighbors_of(node).unwrap().iter().for_each(|n| q.enqueue(n)))
}

// New idea:
//

struct PathTable {
    table: Vec<BTreeMap<String,Vec<usize>>>,
    arena: Arena<String>
}

impl PathTable {
    fn new(graph: &AdjacencySets) -> Self {
        let mut table: Vec<BTreeMap<String,Vec<usize>>> = Vec::new();
        let mut arena = Arena::new();
        let mut open_list = VecDeque::new();
        open_list.push_back((0, START.to_string(), None));
        search(open_list, |(level, node, parent): &(usize, String, Option<String>), q| {
            let parent_paths = parent.clone().map(|p| table[*level - 1].get(p.as_str()).unwrap());
            let paths_to = PathTable::make_paths_for(node.as_str(), &parent_paths, &mut arena);
            if paths_to.len() > 0 {
                if table.len() + 1 == *level {
                    table.push(BTreeMap::new());
                }
                table[*level].insert(node.clone(), paths_to);
                if node.as_str() != END {
                    for neighbor in graph.neighbors_of(node.as_str()).unwrap() {
                        q.enqueue(&(level + 1, neighbor.clone(), Some(node.clone())));
                    }
                }
            }
        });

        PathTable {table, arena}
    }

    fn make_paths_for(node: &str, parent_paths: &Option<&Vec<usize>>, arena: &mut Arena<String>) -> Vec<usize> {
        match parent_paths {
            None => {
                vec![arena.alloc(node.to_string(), None)]
            }
            Some(parent_paths) => {
                let can_repeat = node.chars().any(|c| c.is_uppercase());
                let mut all_paths: Vec<usize> = parent_paths.iter()
                    .map(|addr| arena.alloc(node.to_string(), Some(*addr)))
                    .collect();
                all_paths.retain(|alloc_addr| can_repeat || !arena.get(*alloc_addr).iter(arena).any(|s| s.as_str() == node));
                all_paths
            }
        }
    }
}
