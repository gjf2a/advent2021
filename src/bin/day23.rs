use std::cmp::{max, min};
use std::fmt::{Display, Formatter};
use std::io;
use advent_code_lib::{advent_main, all_lines, AStarCost, AStarNode, best_first_search, ContinueSearch, DirType, make_io_error, ManhattanDir, SearchQueue};
use bare_metal_modulo::{MNum, ModNumC};
use itertools::Itertools;
use enum_iterator::all;

const ENERGY_BASE: u128 = 10;
const MIN_AMPHIPOD: char = 'A';
const MAX_AMPHIPOD: char = 'D';
const NUM_AMPHIPOD_TYPES: usize = MAX_AMPHIPOD as usize - MIN_AMPHIPOD as usize + 1;
const NUM_AMPHIPODS: usize = NUM_AMPHIPOD_TYPES * 2;
const HALLWAY_SPOTS: usize = 11;
const DEPTH: usize = 3;
const ROOM_COLUMNS: [usize; NUM_AMPHIPOD_TYPES] = [2, 4, 6, 8];

fn main() -> io::Result<()> {
    advent_main(&[], &[], |args| {
        println!("Part 1: {}", part1(&AmphipodMap::from_file(args[1].as_str())?));
        Ok(())
    })
}

fn part1(map: &AmphipodMap) -> u128 {
    //let mut max_iterations = 20;
    let start_node = AStarNode::new(map.clone(), AStarCost::new(0, map.distance_to_goal()));
    best_first_search(&start_node, |node, queue| {
        println!("node cost: {} (estimated total: {}) home: {}", node.cost_so_far(), node.total_estimate(), node.item().num_home());
        println!("{}", node.item());
        //max_iterations -= 1;
        //if max_iterations == 0 {return ContinueSearch::No;}
        if node.item().all_home() {
            ContinueSearch::No
        } else {
            for successor in node.item().successors() {
                let cost_so_far = successor.energy_used;
                let estimate_to_goal = successor.distance_to_goal();
                queue.enqueue(&AStarNode::new(successor, AStarCost::new(cost_so_far, estimate_to_goal)));
            }
            ContinueSearch::Yes
        }
    }).cost().unwrap()
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
struct Amphipod {
    abcd: ModNumC<usize, NUM_AMPHIPOD_TYPES>,
    state: AmphipodState,
    column: ModNumC<usize, HALLWAY_SPOTS>,
    row: ModNumC<usize, DEPTH>
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
struct AmphipodMap {
    amphipods: Vec<Amphipod>,
    energy_used: u128
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
enum AmphipodState {
    Starting, FirstMove, Waiting, SecondMove, Finished
}

impl Amphipod {
    fn new(code: char, column: usize, row: usize) -> io::Result<Amphipod> {
        if code < MIN_AMPHIPOD || code > MAX_AMPHIPOD {
            make_io_error(format!("Illegal Amphipod: {}", code).as_str())
        } else {
            Ok(Amphipod {abcd: ModNumC::new(code as usize - MIN_AMPHIPOD as usize),
                state: AmphipodState::Starting,
                column: ModNumC::new(column), row: ModNumC::new(row)})
        }
    }

    fn step_energy(&self) -> u128 {ENERGY_BASE.pow(self.abcd.a() as u32)}

    fn room_column(&self) -> usize {ROOM_COLUMNS[self.abcd.a()]}

    fn displacement(&self) -> usize {distance(self.column.a(), self.room_column())}

    fn aligned_room(&self) -> bool {ROOM_COLUMNS.contains(&self.column.a())}

    fn at_home(&self) -> bool {self.displacement() == 0 && self.column > 0}

    fn can_go(&self, d: ManhattanDir) -> bool {
        match d {
            ManhattanDir::N => self.row > 0 && self.aligned_room(),
            ManhattanDir::E => self.row == 0 && self.column + 1 < HALLWAY_SPOTS,
            ManhattanDir::S => self.row + 1 < DEPTH && self.aligned_room(),
            ManhattanDir::W => self.row == 0 && self.column > 0
        }
    }

    fn go(&mut self, d: ManhattanDir) {
        let (dx, dy) = d.offset();
        self.column = ModNumC::new((self.column.a() as isize + dx) as usize);
        self.row = ModNumC::new((self.row.a() as isize + dy) as usize);
    }
}

impl From<Amphipod> for char {
    fn from(amphipod: Amphipod) -> Self {(amphipod.abcd.a() as u8 + MIN_AMPHIPOD as u8) as char}
}

fn distance(a: usize, b: usize) -> usize {
    max(a, b) - min(a, b)
}

impl AmphipodMap {
    fn from_file(filename: &str) -> io::Result<Self> {
        Ok(Self::from_iter(all_lines(filename)?))
    }

    fn from_iter<I: Iterator<Item=String>>(mut lines: I) -> Self {
        lines.next();
        let mut amphipods = lines.next().unwrap().chars().skip(1)
            .take(HALLWAY_SPOTS).enumerate()
            .filter_map(|(col, c)| Amphipod::new(c, col, 0).ok())
            .collect_vec();
        let row2 = lines.next().unwrap();
        let row3 = lines.next().unwrap();
        for (col, (amp2, amp3)) in row2.chars()
            .zip(row3.chars()).skip(1).enumerate()
            .filter(|(col, _)| ROOM_COLUMNS.contains(col)) {
            let entries = [amp2, amp3];
            for row in 0..entries.len() {
                if let Ok(amphipod) = Amphipod::new(entries[row], col, row + 1) {
                    amphipods.push(amphipod);
                }
            }
        }
        AmphipodMap {amphipods, energy_used: 0}
    }

    fn check_invariants(&self) {
        assert_eq!(self.amphipods.len(), NUM_AMPHIPODS);

        for i in 0..self.amphipods.len() {
            for j in (i+1)..self.amphipods.len() {
                assert!(self.amphipods[i].row != self.amphipods[j].row || self.amphipods[i].column != self.amphipods[j].column);
            }
        }

        assert!(self.amphipods.iter().all(|a| a.row == 0 || (a.row <= 2 && ROOM_COLUMNS.contains(&a.column.a()))));
    }

    fn all_home(&self) -> bool {
        self.amphipods.iter().all(|amp| amp.at_home())
    }

    fn num_home(&self) -> usize {
        self.amphipods.iter().filter(|a| a.at_home()).count()
    }

    fn room_ready(&self, room: ModNumC<usize,NUM_AMPHIPOD_TYPES>) -> bool {
        self.amphipods.iter().all(|a| a.row == 0 ||
            (a.abcd != room && a.column != ROOM_COLUMNS[room.a()]))
    }

    fn distance_to_goal(&self) -> u128 {
        self.amphipods.iter()
            .map(|a| if a.at_home() {0} else {a.step_energy() * (a.row + a.displacement() + 1).a() as u128})
            .sum()
    }

    fn single_mover_only(&self) -> Option<usize> {
        self.amphipods.iter().enumerate()
            .find(|(_, a)| (a.aligned_room() && a.row == 0) ||
                a.state == AmphipodState::SecondMove || a.state == AmphipodState::FirstMove)
            .map(|(i, _)| i)
    }

    fn can_go(&self, i: usize, d: ManhattanDir) -> bool {
        self.can_occupy(i, d) &&
            self.single_mover_only().map_or(true, |m| m == i) &&
            (self.amphipods[i].state != AmphipodState::SecondMove || self.room_ready(self.amphipods[i].abcd))
    }

    fn can_occupy(&self, i: usize, d: ManhattanDir) -> bool {
        let mut neighbor = self.amphipods[i].clone();
        if neighbor.can_go(d) {
            neighbor.go(d);
            self.amphipods.iter().all(|a| *a != neighbor)
        } else {
            false
        }
    }

    fn legal_moves_for(&self, i: usize) -> Vec<AmphipodMap> {
        all::<ManhattanDir>()
            .filter(|d| self.can_go(i, *d))
            .map(|d| {
                let mut copy = self.clone();
                copy.amphipods[i].go(d);
                copy.energy_used += copy.amphipods[i].step_energy();
                for (j, a) in copy.amphipods.iter_mut().enumerate() {
                    a.state = if i == j {
                        match a.state {
                            AmphipodState::Starting => AmphipodState::FirstMove,
                            AmphipodState::Waiting => AmphipodState::SecondMove,
                            AmphipodState::SecondMove => if a.at_home() {AmphipodState::Finished} else {AmphipodState::SecondMove},
                            other => other
                        }
                    } else {
                        match a.state {
                            AmphipodState::FirstMove => AmphipodState::Waiting,
                            AmphipodState::SecondMove => panic!("This should never happen: {:?}, moving {}", self.amphipods.iter().map(|a| a.state).collect_vec(), i),
                            other => other
                        }
                    }
                }
                copy.check_invariants();
                copy
            })
            .collect()
    }

    fn successors(&self) -> Vec<AmphipodMap> {
        let mut successors = Vec::new();
        for i in 0..self.amphipods.len() {
            successors.append(&mut self.legal_moves_for(i));
        }
        successors
    }
}

impl Display for Amphipod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}

impl Display for AmphipodMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", (0..(2+HALLWAY_SPOTS)).map(|_| '#').collect::<String>())?;
        writeln!(f, "#{}#", (0..HALLWAY_SPOTS)
            .map(|i| self.amphipods.iter().find(|a| a.row == 0 && a.column == i)
                .map_or('.', |a| char::from(*a))).collect::<String>())?;
        for row in 1..=2 {
            let fix = if row == 1 {"##"} else {"  "};
            let roomed = (0..NUM_AMPHIPOD_TYPES)
                .map(|i| self.amphipods.iter().find(|a| a.row == row && a.column == ROOM_COLUMNS[i])
                    .map_or(".#".to_string(), |a| format!("{}#", char::from(*a))))
                .collect::<String>();
            writeln!(f, "{}#{}{}", fix, roomed, fix)?;
        }
        writeln!(f, "  {}  ", (0..(HALLWAY_SPOTS - 2)).map(|_| '#').collect::<String>())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{Amphipod, AmphipodMap};

    #[test]
    fn test_read_amphipod() {
        for (code, energy) in [('A', 1), ('B', 10), ('C', 100), ('D', 1000)] {
            let amphipod = Amphipod::new(code, 0, 0).unwrap();
            assert_eq!(amphipod.step_energy(), energy);
        }
    }

    #[test]
    fn test_read_map() {
        let map = AmphipodMap::from_file("ex/day23.txt").unwrap();
        let map_str = format!("{}", map);
        let map2 = AmphipodMap::from_iter(map_str.split('\n').map(|s| s.to_string()));
        assert_eq!(map, map2);
    }
}