use std::cmp::Ordering;
use std::io;
use advent_code_lib::{advent_main, nums2map, Position, search, SearchQueue, map_width_height, RowMajorPositionIterator, ManhattanDir, DirType, path_back_from};
use std::collections::{HashMap, BinaryHeap, BTreeMap};
use std::fmt::{Display, Formatter};
use bare_metal_modulo::{MNum, ModNumC};
use common_macros::b_tree_map;

const EXPANSION_FACTOR: usize = 5;
const SHOW_GRID: &'static str = "-grid";
const PATH_LEN: &'static str = "-len";
const PATH: &'static str = "-path";

fn main() -> io::Result<()> {
    advent_main(&["(1|2)"], &[SHOW_GRID, PATH_LEN, PATH], |args| {
        let part = args[2].as_str();
        let mut map = RiskMap::new(args[1].as_str())?;
        if part == "2" {
            map = map.expand(EXPANSION_FACTOR);
        }
        if args.contains(&SHOW_GRID.to_string()) {println!("{}", map);}
        let ((goal, cost), parent_map) = map.dijkstra();
        if args.contains(&PATH.to_string()) {print_path(goal, &parent_map);}
        if args.contains(&PATH_LEN.to_string()) {path_len_only(goal, &parent_map);}
        println!("Part {} score: {}", part, cost);
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

    fn dijkstra(&self) -> ((Position, u128), BTreeMap<Position,Option<Position>>) {
        let mut open_list: BinaryHeap<DijkstraNode> = BinaryHeap::new();
        let start = Position::new();
        let mut parent_map = b_tree_map! {start => None};
        let goal = Position::from(((self.width - 1) as isize, (self.height - 1) as isize));
        open_list.enqueue(&DijkstraNode::new(start, 0));
        let mut goal_node = None;
        search(open_list, |node, queue| {
            if node.p == goal {
                goal_node = Some(node.clone());
            } else {
                for neighbor in node.p.manhattan_neighbors() {
                    if let Some(risk) = self.risk(neighbor) {
                        if !parent_map.contains_key(&neighbor) {
                            let neighbor_node = DijkstraNode::new(neighbor, node.cost_so_far + risk);
                            parent_map.insert(neighbor, Some(node.p));
                            queue.enqueue(&neighbor_node);
                        }
                    }
                }
            }
        });
        let goal_node = goal_node.unwrap();
        ((goal_node.p, goal_node.cost_so_far), parent_map)
    }

    fn points_at<'a>(&'a self, offset: &'a Position) -> impl Iterator<Item=Position> + 'a {
        self.risks.iter().map(|(p, _)|
            Position::from((p.col + offset.col * self.width as isize,
                            p.row + offset.row * self.height as isize)))
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, Debug)]
struct DijkstraNode {
    p: Position,
    cost_so_far: u128
}

impl DijkstraNode {
    pub fn new(p: Position, cost_so_far: u128) -> Self {
        DijkstraNode {p, cost_so_far}
    }
}

impl PartialOrd for DijkstraNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.cost_so_far.partial_cmp(&other.cost_so_far).map(|ord| ord.reverse())
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

fn path(goal: Position, parent_map: &BTreeMap<Position, Option<Position>>) -> Vec<(isize, isize)> {
    path_back_from(&goal, parent_map).iter().map(|p| (p.col, p.row)).collect()
}

fn print_path(goal: Position, parent_map: &BTreeMap<Position, Option<Position>>) {
    let path = path(goal, parent_map);
    println!("{:?}", path);
    println!("Length: {}", path.len());
}

fn path_len_only(goal: Position, parent_map: &BTreeMap<Position, Option<Position>>) {
    let path = path(goal, parent_map);
    println!("Length: {}", path.len());
}