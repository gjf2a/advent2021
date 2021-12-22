use std::fmt::{Display, Formatter};
use std::io;
use std::str::FromStr;
use advent_code_lib::{advent_main, all_lines, make_io_error};

const PART_1_MAX: isize = 50;
const ON: &'static str = "on";
const OFF: &'static str = "off";

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
enum CubeState {On, Off}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct RangeDim {
    start: isize, end: isize
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct CuboidAction {
    action: CubeState, x: RangeDim, y: RangeDim, z: RangeDim
}

impl RangeDim {
    fn contains(&self, value: isize) -> bool {self.start <= value && value <= self.end}

    fn envelops(&self, other: &RangeDim) -> bool {
        self.contains(other.start) && self.contains(other.end)
    }
}

impl CuboidAction {
    fn envelops(&self, other: &CuboidAction) -> bool {
        self.x.envelops(&other.x) && self.y.envelops(&other.y) && self.z.envelops(&other.z)
    }
}

impl AllActions {
    fn from_file(filename: &str) -> io::Result<Self> {
        Ok(AllActions {actions: all_lines(filename)?.map(|line| line.parse::<CuboidAction>().unwrap()).collect()})
    }

    fn part1(&self) -> Self {
        let checker_range = RangeDim {start: -PART_1_MAX, end: PART_1_MAX};
        let checker_cube = CuboidAction {action: CubeState::On, x: checker_range, y: checker_range, z: checker_range};
        AllActions {actions: self.actions.iter().filter(|cuboid| checker_cube.envelops(*cuboid)).copied().collect()}
    }
}

impl FromStr for CuboidAction {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts1 = s.split_whitespace();
        let action = parts1.next().unwrap().parse::<CubeState>()?;
        let ranges: Vec<RangeDim> = parts1.next().unwrap().split(',')
            .map(|s| s.split('=').skip(1).next().unwrap().parse::<RangeDim>().unwrap())
            .collect();
        Ok(CuboidAction {action, x: ranges[0], y: ranges[1], z: ranges[2]})
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
        write!(f, "{} x={},y={},z={}", self.action, self.x, self.y, self.z)
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