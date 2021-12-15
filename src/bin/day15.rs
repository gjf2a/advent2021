use std::cmp::Ordering;
use std::io;
use advent_code_lib::{advent_main, nums2map, Position, search, SearchQueue, map_width_height, path_back_from};
use std::collections::{HashMap, BinaryHeap, BTreeMap};

fn main() -> io::Result<()> {
    advent_main(&[], &[], |args| {
        let map = RiskMap::new(args[1].as_str())?;
        let path = map.a_star_search();
        let cost: usize = path.iter().map(|(_, cost)| *cost).sum();
        println!("Part 1 score: {}", cost - map.risk(Position::new()).unwrap());
        Ok(())
    })
}

struct RiskMap {
    risks: HashMap<Position, u32>,
    width: usize,
    height: usize
}

impl RiskMap {
    fn new(filename: &str) -> io::Result<Self> {
        let risks = nums2map(filename)?;
        let (width, height) = map_width_height(&risks);
        Ok(RiskMap {risks, width: width, height: height})
    }

    fn risk(&self, p: Position) -> Option<usize> {
        self.risks.get(&p).map(|risk| *risk as usize)
    }

    fn a_star_search(&self) -> Vec<(Position, usize)> {
        let mut open_list: BinaryHeap<AStarSearchNode> = BinaryHeap::new();
        let start = Position::new();
        let mut visited = BTreeMap::new();
        visited.insert(start, None);
        let goal = Position::from(((self.width - 1) as isize, (self.height - 1) as isize));
        open_list.enqueue(&AStarSearchNode::new(start, 0, |p| manhattan_cost_estimate(p, goal)));
        let mut goal_node = None;
        search(open_list, |node, queue| {
            if node.p == goal {
                goal_node = Some(node.clone());
            } else {
                for neighbor in node.p.manhattan_neighbors() {
                    if let Some(risk) = self.risk(neighbor) {
                        if !visited.contains_key(&neighbor) {
                            let neighbor_node = AStarSearchNode::new(neighbor, node.cost_so_far + risk, |p| manhattan_cost_estimate(p, goal));
                            visited.insert(neighbor, Some(node.p));
                            queue.enqueue(&neighbor_node);
                        }
                    }
                }
            }
        });
        path_back_from(&goal, &visited).iter().map(|node| (*node, self.risk(*node).unwrap())).collect()
    }
}


pub fn manhattan_cost_estimate(p: Position, goal: Position) -> usize {
    let manhattan = goal - p;
    (manhattan.col + manhattan.row) as usize
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, Debug)]
pub struct AStarSearchNode {
    p: Position,
    cost_so_far: usize,
    heuristic_estimate: usize
}

impl AStarSearchNode {
    pub fn new<H: Fn(Position)->usize>(p: Position, cost_so_far: usize, heuristic: H) -> Self {
        AStarSearchNode {p, cost_so_far, heuristic_estimate: heuristic(p)}
    }

    pub fn estimated_cost(&self) -> usize {
        self.cost_so_far + self.heuristic_estimate
    }
}

impl PartialOrd for AStarSearchNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.estimated_cost().partial_cmp(&other.estimated_cost()) {
            None => None,
            Some(cmp) => Some(match cmp {
                Ordering::Less => Ordering::Greater,
                Ordering::Equal => cmp,
                Ordering::Greater => Ordering::Less
            })
        }
    }
}