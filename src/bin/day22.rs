use std::cmp::{max, min};
use std::fmt::{Display, Formatter};
use std::io;
use std::str::FromStr;
use advent_code_lib::{advent_main, all_lines, make_io_error};

const PART_1_MAX: isize = 50;
const ON: &'static str = "on";
const OFF: &'static str = "off";
const DIMENSIONS: usize = 3;

fn main() -> io::Result<()> {
    advent_main(&[], &[], |args| {
        let actions = AllActions::from_file(args[1].as_str())?;
        let part1 = actions.part1();
        println!("{}", part1);
        Ok(())
    })
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct AllActions {
    actions: Vec<CuboidAction>
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct CuboidAction {
    action: CubeState, region: Cuboid
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum CubeState {On, Off}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Cuboid {
    ranges: [RangeDim; DIMENSIONS]
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct RangeDim {
    start: isize, end: isize
}

impl RangeDim {
    fn contains(&self, value: isize) -> bool {self.start <= value && value <= self.end}

    fn envelops(&self, other: &RangeDim) -> bool {
        self.contains(other.start) && self.contains(other.end)
    }

    fn intersection(&self, other: &RangeDim) -> Option<RangeDim> {
        if other.end < self.start || self.end < other.start {
            None
        } else {
            Some(RangeDim {start: max(self.start, other.start), end: min(self.end, other.end)})
        }
    }
}

impl Cuboid {
    fn from_iter<I:Iterator<Item=RangeDim>>(mut iter: I) -> Self {
        let skeleton = [RangeDim {start: 0, end: 0}; DIMENSIONS];
        Self {ranges: skeleton.map(|_| iter.next().unwrap())}
    }

    fn envelops(&self, other: &Cuboid) -> bool {
        self.ranges.iter().zip(other.ranges.iter()).all(|(a, b)| a.envelops(b))
    }

    fn intersection<I: Iterator<Item=RangeDim> + FromIterator<RangeDim>>(&self, other: &Cuboid) -> Option<Self> {
        self.ranges.iter().zip(other.ranges.iter()).map(|(a, b)| a.intersection(b))
            .collect::<Option<I>>()
            .map(|iter| Self::from_iter(iter))
    }
}

impl AllActions {
    fn from_file(filename: &str) -> io::Result<Self> {
        Ok(AllActions {actions: all_lines(filename)?.map(|line| line.parse::<CuboidAction>().unwrap()).collect()})
    }

    fn part1(&self) -> Self {
        let checker_range = RangeDim {start: -PART_1_MAX, end: PART_1_MAX};
        let checker_cube = Cuboid {ranges: [checker_range; DIMENSIONS]};
        AllActions {actions: self.actions.iter()
            .filter(|cuboid| checker_cube.envelops(&cuboid.region))
            .copied().collect()}
    }
}

impl FromStr for CuboidAction {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let action = parts.next().unwrap().parse::<CubeState>()?;
        let region = parts.next().unwrap().parse::<Cuboid>()?;
        Ok(CuboidAction {action, region})
    }
}

impl FromStr for Cuboid {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let range_iter = s.split(',')
            .map(|s| s.split('=').skip(1).next().unwrap().parse().unwrap());
        Ok(Self::from_iter(range_iter))
    }
}

impl FromStr for RangeDim {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<isize> = s.split("..").map(|p| p.parse().unwrap()).collect();
        Ok(RangeDim {start: parts[0], end: parts[1]})
    }
}

impl FromStr for CubeState {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            ON => Ok(CubeState::On),
            OFF => Ok(CubeState::Off),
            other => make_io_error(format!("Unrecognized state: \"{}\"", other).as_str())
        }
    }
}

impl Display for CubeState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {CubeState::On => ON, CubeState::Off => OFF})
    }
}

impl Display for RangeDim {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}

impl Display for CuboidAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.action, self.region)
    }
}

impl Display for Cuboid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "x={},y={},z={}", self.ranges[0], self.ranges[1], self.ranges[2])
    }
}

impl Display for AllActions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for action in self.actions.iter() {
            writeln!(f, "{}", action)?;
        }
        Ok(())
    }
}