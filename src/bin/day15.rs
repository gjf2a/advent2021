use std::cmp::Ordering;
use std::io;
use advent_code_lib::{advent_main, nums2map, Position, search, SearchQueue, map_width_height, path_back_from, breadth_first_search};
use std::collections::{HashMap, BinaryHeap};
use bare_metal_modulo::{MNum, ModNumC};
use common_macros::b_tree_map;

fn main() -> io::Result<()> {
    advent_main(&["(1|2)"], &[], |args| {
        let part = args[2].as_str();
        let mut map = RiskMap::new(args[1].as_str())?;
        if part == "2" {
            map = map.expand(5);
            println!("Expanded!");
        }
        let path = map.a_star_search();
        let cost: u128 = path.iter().skip(1).map(|(_, cost)| *cost).sum();
        println!("Part {} score: {}", part, cost);
        Ok(())
    })
}

struct RiskMap {
    risks: HashMap<Position, ModNumC<u32, 10>>,
    width: usize,
    height: usize
}

impl RiskMap {
    fn new(filename: &str) -> io::Result<Self> {
        let risks = nums2map(filename)?;
        Ok(Self::from(risks))
    }

    fn from(risks: HashMap<Position, ModNumC<u32, 10>>) -> Self {
        let (width, height) = map_width_height(&risks);
        RiskMap {risks, width, height}
    }

    // This was cool. Too bad I misunderstood the instructions.
    fn expand(&self, expansion_factor: isize) -> Self {
        let mut expanded_risks = HashMap::new();
        for (p, risk) in self.risks.iter() {
            breadth_first_search(&(*p, *risk), |node, queue| {
                let (successor, s_risk) = node;
                expanded_risks.insert(*successor, *s_risk);
                for neighbor in successor.manhattan_neighbors() {
                    if neighbor.col >= p.col && neighbor.row >= p.row &&
                        neighbor.col < p.col + expansion_factor &&
                        neighbor.row < p.row + expansion_factor {
                        let mut neighbor_risk = *s_risk + 1;
                        if neighbor_risk == 0 {
                            neighbor_risk += 1;
                        }
                        queue.enqueue(&(neighbor, neighbor_risk));
                    }
                }
            });
        }
        Self::from(expanded_risks)
    }

    fn risk(&self, p: Position) -> Option<u128> {
        self.risks.get(&p).map(|risk| risk.a() as u128)
    }

    fn a_star_search(&self) -> Vec<(Position, u128)> {
        let mut open_list: BinaryHeap<AStarSearchNode> = BinaryHeap::new();
        let start = Position::new();
        let mut parent_map = b_tree_map! {start => None};
        let goal = Position::from(((self.width - 1) as isize, (self.height - 1) as isize));
        open_list.enqueue(&AStarSearchNode::new(start, 0, |p| manhattan_cost_estimate(p, goal)));
        let mut goal_node = None;
        search(open_list, |node, queue| {
            if node.p == goal {
                goal_node = Some(node.clone());
            } else {
                for neighbor in node.p.manhattan_neighbors() {
                    if let Some(risk) = self.risk(neighbor) {
                        if !parent_map.contains_key(&neighbor) {
                            let neighbor_node = AStarSearchNode::new(neighbor, node.cost_so_far + risk, |p| manhattan_cost_estimate(p, goal));
                            parent_map.insert(neighbor, Some(node.p));
                            queue.enqueue(&neighbor_node);
                        }
                    }
                }
            }
        });
        path_back_from(&goal, &parent_map).iter().map(|node| (*node, self.risk(*node).unwrap())).collect()
    }
}


pub fn manhattan_cost_estimate(p: Position, goal: Position) -> u128 {
    let manhattan = goal - p;
    (manhattan.col + manhattan.row) as u128
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, Debug)]
pub struct AStarSearchNode {
    p: Position,
    cost_so_far: u128,
    heuristic_estimate: u128
}

impl AStarSearchNode {
    pub fn new<H: Fn(Position)->u128>(p: Position, cost_so_far: u128, heuristic: H) -> Self {
        AStarSearchNode {p, cost_so_far, heuristic_estimate: heuristic(p)}
    }

    pub fn estimated_cost(&self) -> u128 {
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