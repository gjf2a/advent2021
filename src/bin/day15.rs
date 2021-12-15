use std::cmp::Ordering;
use std::io;
use advent_code_lib::{advent_main, nums2map, Position, search, SearchQueue, map_width_height, RowMajorPositionIterator, ManhattanDir, DirType};
use std::collections::{HashMap, BinaryHeap};
use std::fmt::{Display, Formatter};
use bare_metal_modulo::{MNum, ModNumC};
use common_macros::b_tree_map;

const EXPANSION_FACTOR: usize = 5;

fn main() -> io::Result<()> {
    advent_main(&["(1|2)"], &["-show"], |args| {
        let part = args[2].as_str();
        let mut map = RiskMap::new(args[1].as_str())?;
        if part == "2" {
            map = map.expand(EXPANSION_FACTOR);
        }
        if args.len() >= 4 {
            println!("{}", map);
        }
        println!("Part {} score: {}", part, map.a_star_search());
        Ok(())
    })
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Risk {
    risk: ModNumC<u128, 9>
}

impl Risk {
    fn from(risk: ModNumC<u32, 10>) -> Self {
        Risk {risk: ModNumC::new(risk.a() as u128 - 1)}
    }

    fn risk(&self) -> u128 {1 + self.risk.a()}

    fn bump(&self) -> Self {Risk {risk: self.risk + 1}}
}

#[derive(Clone)]
struct RiskMap {
    risks: HashMap<Position, Risk>,
    width: usize,
    height: usize
}

impl RiskMap {
    fn new(filename: &str) -> io::Result<Self> {
        Ok(Self::from(nums2map(filename)?.iter()
            .map(|(p,r)| (*p, Risk::from(*r)))
            .collect()))
    }

    fn from(risks: HashMap<Position, Risk>) -> Self {
        let (width, height) = map_width_height(&risks);
        RiskMap {risks, width, height}
    }

    fn expand(&self, expansion_factor: usize) -> Self {
        let mut expanded_risks = self.risks.clone();
        for offset in RowMajorPositionIterator::new(expansion_factor, expansion_factor).skip(1) {
            let prev_dir = if offset.col == 0 {ManhattanDir::N} else {ManhattanDir::W};
            let prev_offset = prev_dir.next(offset);
            let prev_points = self.points_at(&prev_offset);
            for (old_point, new_point) in prev_points.zip(self.points_at(&offset)) {
                expanded_risks.insert(new_point, expanded_risks.get(&old_point).unwrap().bump());
            }
        }
        RiskMap::from(expanded_risks)
    }

    fn risk(&self, p: Position) -> Option<u128> {
        self.risks.get(&p).map(|r| r.risk())
    }

    fn a_star_search(&self) -> u128 {
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
        goal_node.unwrap().cost_so_far
    }

    fn points_at<'a>(&'a self, offset: &'a Position) -> impl Iterator<Item=Position> + 'a {
        self.risks.iter().map(|(p, _)|
            Position::from((p.col + offset.col * self.width as isize,
                            p.row + offset.row * self.height as isize)))
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
        self.cost_so_far //+ self.heuristic_estimate
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

impl Display for RiskMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for p in RowMajorPositionIterator::new(self.width, self.height) {
            if p.col == 0 && p.row > 0 {writeln!(f)?;}
            write!(f, "{}", self.risks.get(&p).unwrap().risk())?
        }
        Ok(())
    }
}