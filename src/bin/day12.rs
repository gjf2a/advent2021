use std::collections::{BTreeMap, BTreeSet};
use std::fmt::{Display, Formatter};
use std::io;
use advent_code_lib::{AdjacencySets, all_lines, Arena, breadth_first_search, generic_main, SearchQueue};
use common_macros::b_tree_set;
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
        let table = PathTable::new(&graph, part == "2");
        if let Some(arg) = args.get(3) {
            if arg.as_str() == SHOW_PATH_ARG {
                println!("{}", table);
                for path in table.all_paths_to(END).iter() {
                    println!("{:?}", path);
                }
            }
        }
        println!("Part {}: {}", part, table.total_path_count_to(END));
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

#[derive(Debug, Clone)]
struct PathTable {
    table: Vec<BTreeMap<String,BTreeSet<usize>>>,
    arena: Arena<String>
}

impl PathTable {
    fn new(graph: &AdjacencySets, allow_extra_small: bool) -> Self {
        let mut table: Vec<BTreeMap<String,BTreeSet<usize>>> = Vec::new();
        let mut arena = Arena::new();
        breadth_first_search(&(0, START.to_string(), None), |search_node: &(usize, String, Option<String>), q| {
            let (level, node, parent) = search_node;
            let q_parent = q.parent_of(search_node).clone().map(|(_, p, _)| p);
            println!("parent: {:?} q_parent: {:?}", parent, q_parent);
            assert_eq!(q_parent, parent.clone());
            let parent_paths = parent.clone().map(|p| table[*level - 1].get(p.as_str()).unwrap());
            let paths_to = PathTable::make_paths_for(node.as_str(), &parent_paths, &mut arena, allow_extra_small);
            if paths_to.len() > 0 {
                PathTable::update_table(&mut table, *level, node.as_str(), paths_to);
                if node.as_str() != END {
                    for neighbor in graph.neighbors_of(node.as_str()).unwrap() {
                        q.enqueue(&(level + 1, neighbor.clone(), Some(node.clone())));
                    }
                }
            }
        });

        PathTable {table, arena}
    }

    fn make_paths_for(node: &str, parent_paths: &Option<&BTreeSet<usize>>, arena: &mut Arena<String>, allow_extra_small: bool) -> BTreeSet<usize> {
        match parent_paths {
            None => {
                b_tree_set![arena.alloc(node.to_string(), None)]
            }
            Some(parent_paths) => {
                let mut allow_extra_small = allow_extra_small;
                if [START, END].contains(&node) {
                    allow_extra_small = false;
                }
                let path_prefixes = PathTable::filter_parent_paths(*parent_paths, node, arena, allow_extra_small);
                PathTable::allocate_new_paths(&path_prefixes, node, arena)
            }
        }
    }

    fn filter_parent_paths(parent_paths: &BTreeSet<usize>, node: &str, arena: &mut Arena<String>, allow_extra_small: bool) -> Vec<usize> {
        parent_paths.iter()
            .filter(|addr| has_upper(node) ||
                !allow_extra_small && PathTable::rigid_small_constraint(arena, **addr, node) ||
                allow_extra_small && PathTable::looser_small_constraint(arena, **addr, node))
            .copied()
            .collect()
    }

    fn allocate_new_paths(path_prefixes: &Vec<usize>, node: &str, arena: &mut Arena<String>) -> BTreeSet<usize> {
        path_prefixes.iter()
            .map(|addr| arena.alloc(node.to_string(), Some(*addr)))
            .collect()
    }

    fn rigid_small_constraint(arena: &Arena<String>, addr: usize, node: &str) -> bool {
        !arena.get(addr).iter(arena).any(|s| s.as_str() == node)
    }

    fn looser_small_constraint(arena: &Arena<String>, addr: usize, node: &str) -> bool {
        let small_counts: HashHistogram<String> = arena.get(addr).iter(arena)
            .filter(|s| !has_upper((*s).as_str()))
            .collect();
        let num_2 = small_counts.iter().filter(|(_, count)| **count > 1).count();
        let node_count = small_counts.count(&node.to_string());
        node_count == 1 && num_2 == 0 || node_count == 0 && num_2 <= 1
    }

    fn update_table(table: &mut Vec<BTreeMap<String,BTreeSet<usize>>>, level: usize, node: &str, mut paths_to: BTreeSet<usize>) {
        if table.len() == level {
            table.push(BTreeMap::new());
        }
        match table[level].get_mut(node) {
            None => { table[level].insert(node.to_string(), paths_to.clone()); }
            Some(paths) => { paths.append(&mut paths_to); }
        }
    }

    fn all_paths_to(&self, node: &str) -> Vec<Vec<String>> {
        let mut result = Vec::new();
        for row in self.table.iter() {
            if let Some(row) = row.get(node) {
                for path_end in row.iter() {
                    result.push(self.path_at_addr(*path_end));
                }
            }
        }
        result
    }

    fn path_at_addr(&self, addr: usize) -> Vec<String> {
        let mut path = self.arena.get(addr).iter(&self.arena).collect::<Vec<_>>();
        path.reverse();
        path.iter().map(|s| (*s).clone()).collect()
    }

    fn total_path_count_to(&self, node: &str) -> usize {
        self.all_paths_to(node).len()
    }
}

fn has_upper(s: &str) -> bool {
    s.chars().any(|c| c.is_uppercase())
}

impl Display for PathTable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (row_num, row) in self.table.iter().enumerate() {
            writeln!(f, "Row {}", row_num)?;
            for (node, parents) in row.iter() {
                writeln!(f, "Node: {}", node)?;
                for (id, path) in parents.iter().map(|id| (id, self.path_at_addr(*id))) {
                    writeln!(f, "({}) {:?}", id, path)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
