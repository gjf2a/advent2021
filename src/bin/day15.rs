use std::cmp::Ordering;
use std::io;
use advent_code_lib::{advent_main, nums2map, Position, search, SearchQueue, map_width_height, RowMajorPositionIterator, ManhattanDir, DirType, ContinueSearch};
use std::collections::{HashMap, BinaryHeap};
use std::fmt::{Display, Formatter};
use bare_metal_modulo::{MNum, ModNumC};
use common_macros::b_tree_set;

const EXPANSION_FACTOR: usize = 5;
const SHOW_GRID: &'static str = "-grid";
const A_STAR: &'static str = "-a*";

fn main() -> io::Result<()> {
    advent_main(&["(1|2)"], &[SHOW_GRID, A_STAR], |args| {
        let part = args[2].as_str();
        let mut map = RiskMap::new(args[1].as_str())?;
        if part == "2" {
            map = map.expand(EXPANSION_FACTOR);
        }
        if args.contains(&SHOW_GRID.to_string()) { println!("{}", map); }
        let use_a_star = args.contains(&A_STAR.to_string());
        println!("Part {} score: {}", part, map.path_cost(use_a_star));
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

    fn bumped(&self) -> Self {Risk {risk: self.risk + 1}}
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
                expanded_risks.insert(new_point, expanded_risks.get(&old_point).unwrap().bumped());
            }
        }
        RiskMap::from(expanded_risks)
    }

    fn risk(&self, p: Position) -> Option<u128> {
        self.risks.get(&p).map(|r| r.risk())
    }

    fn path_cost(&self, use_a_star: bool) -> u128 {
        let goal = Position::from(((self.width - 1) as isize, (self.height - 1) as isize));
        let a_star_goal = if use_a_star {Some(goal)} else {None};
        let start_node = PriorityNode::new(Position::new(), 0, a_star_goal);
        let mut open_list: BinaryHeap<PriorityNode> = BinaryHeap::new();
        open_list.enqueue(&start_node);
        let mut visited = b_tree_set! {start_node.p};
        let mut cost_at_goal = None;
        search(open_list, |node, queue| {
            visited.insert(node.p);
            if node.p == goal {
                cost_at_goal = Some(node.cost_so_far);
                ContinueSearch::No
            } else {
                for neighbor in node.p.manhattan_neighbors() {
                    if let Some(risk) = self.risk(neighbor) {
                        if !visited.contains(&neighbor) {
                            if !use_a_star {visited.insert(neighbor);}
                            let neighbor_node = PriorityNode::new(neighbor, node.cost_so_far + risk, a_star_goal);
                            queue.enqueue(&neighbor_node);
                        }
                    }
                }
                ContinueSearch::Yes
            }
        });
        cost_at_goal.unwrap()
    }

    fn points_at<'a>(&'a self, offset: &'a Position) -> impl Iterator<Item=Position> + 'a {
        self.risks.iter().map(|(p, _)|
            Position::from((p.col + offset.col * self.width as isize,
                            p.row + offset.row * self.height as isize)))
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, Debug)]
struct PriorityNode {
    p: Position,
    cost_so_far: u128,
    goal: Option<Position>
}

impl PriorityNode {
    pub fn new(p: Position, cost_so_far: u128, goal: Option<Position>) -> Self {
        PriorityNode {p, cost_so_far, goal}
    }

    pub fn total_estimated(&self) -> u128 {
        self.cost_so_far + self.goal.map_or(0, |g| g.manhattan_distance(self.p) as u128)
    }
}

impl PartialOrd for PriorityNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.total_estimated().partial_cmp(&other.total_estimated()).map(|ord| ord.reverse())
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
