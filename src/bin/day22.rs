use std::cmp::{max, min};
use std::fmt::{Display, Formatter};
use std::io;
use std::str::FromStr;
use advent_code_lib::{advent_main, all_lines, combinations_of, make_inner_io_error, make_io_error};
use itertools::Itertools;

const PART_1_MAX: isize = 50;
const ON: &'static str = "on";
const OFF: &'static str = "off";
const DIMENSIONS: usize = 3;

fn main() -> io::Result<()> {
    advent_main(&[], &[], |args| {
        let actions = AllActions::from_file(args[1].as_str())?;
        let part1 = actions.part1();
        println!("Part 1: {}", part1.total_on());
        println!("Part 2: {}", actions.total_on());
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

impl AllActions {
    fn from_file(filename: &str) -> io::Result<Self> {
        Ok(AllActions {actions: all_lines(filename)?.map(|line| line.parse::<CuboidAction>().unwrap()).collect()})
    }

    fn part1(&self) -> Self {
        let checker_range = RangeDim::from(-PART_1_MAX, PART_1_MAX).unwrap();
        let checker_cube = Cuboid {ranges: [checker_range; DIMENSIONS]};
        AllActions {actions: self.actions.iter()
            .filter(|cuboid| checker_cube.envelops(&cuboid.region))
            .copied().collect()}
    }

    fn total_on(&self) -> usize {
        //println!("total_on");
        let mut activated = Vec::new();
        for action in self.actions.iter() {
            activated = action.apply_to(&activated);
            /*println!("activated");
            for cube in activated.iter() {
                println!("{}: {}", cube.num_cubes(), cube);
            }*/
        }
        //println!("done");
        activated.iter().map(|cuboid| cuboid.num_cubes()).sum()
    }
}

impl CuboidAction {
    fn apply_to(&self, activated: &Vec<Cuboid>) -> Vec<Cuboid> {
        let mut result = Vec::new();
        for cuboid in activated.iter() {
            match self.region.intersection(cuboid) {
                None => {result.push(cuboid.clone())}
                Some(intersection) => {
                    if let Some(broken_pieces) = cuboid.break_out(&intersection) {
                        for piece in broken_pieces {
                            if piece != intersection {
                                result.push(piece);
                            }
                        }
                    }
                }
            }
        }
        if self.action == CubeState::On {
            result.push(self.region.clone());
        }
        result
    }
}

impl Cuboid {
    fn from_iter<I:Iterator<Item=RangeDim>>(mut iter: I) -> Self {
        let skeleton = [RangeDim {start: 0, end: 0}; DIMENSIONS];
        Self {ranges: skeleton.map(|_| iter.next().unwrap())}
    }

    fn num_cubes(&self) -> usize {
        self.ranges.iter().map(|range| range.span()).product()
    }

    fn envelops(&self, other: &Cuboid) -> bool {
        self.ranges.iter().zip(other.ranges.iter()).all(|(a, b)| a.envelops(b))
    }

    fn break_out(&self, piece: &Cuboid) -> Option<Vec<Cuboid>> {
        //println!("Breaking out piece: {} from self: {}", piece, self);
        let break_outs = self.ranges.iter()
            .zip(piece.ranges.iter())
            .filter_map(|(a, b)| a.break_out(b))
            .collect_vec();
        //println!("break_outs:");
        /*for break_out in break_outs.iter() {
            println!("break_out: {:?}", break_out);
        }*/
        let num_break_outs = break_outs.iter().map(|v| v.len()).collect_vec();
        if break_outs.len() == DIMENSIONS {
            let mut result = Vec::new();
            combinations_of(num_break_outs.iter().copied().max().unwrap(),
                            &mut |indices: &[usize; DIMENSIONS]| {
                                if indices.iter().zip(num_break_outs.iter()).all(|(i, len)| *i < *len) {
                                    let mut ranges = [RangeDim { start: 0, end: 0 }; DIMENSIONS];
                                    for i in 0..DIMENSIONS {
                                        ranges[i] = break_outs[i][indices[i]];
                                    }
                                    result.push(Cuboid {ranges});
                                }
                            });
            Some(result)
        } else  {
            None
        }
    }

    fn intersection(&self, other: &Cuboid) -> Option<Self> {
        let ranges = self.ranges.iter()
            .zip(other.ranges.iter())
            .filter_map(|(a, b)| a.intersection(b))
            .collect_vec();
        if ranges.len() == DIMENSIONS {
            Some(Self::from_iter(ranges.iter().copied()))
        } else {
            None
        }
    }
}

impl RangeDim {
    fn from(start: isize, end: isize) -> Option<Self> {
        if start <= end {
            Some(RangeDim {start, end})
        } else {
            None
        }
    }

    fn span(&self) -> usize {
        (self.end - self.start + 1) as usize
    }

    fn contains(&self, value: isize) -> bool {self.start <= value && value <= self.end}

    fn envelops(&self, other: &RangeDim) -> bool {
        self.contains(other.start) && self.contains(other.end)
    }

    fn break_out(&self, piece: &RangeDim) -> Option<Vec<RangeDim>> {
        self.intersection(piece)
            //.filter(|intersection| *intersection != *self)
            .map(|intersection| {
                let mut result = vec![intersection];
                if intersection.start > self.start {
                    result.push(RangeDim::from(self.start, intersection.start - 1).unwrap());
                }
                if intersection.end < self.end {
                    result.push(RangeDim::from(intersection.end + 1, self.end).unwrap());
                }
                result
            })
    }

    fn intersection(&self, other: &RangeDim) -> Option<RangeDim> {
        RangeDim::from(max(self.start, other.start), min(self.end, other.end))
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
        Self::from(parts[0], parts[1])
            .ok_or(make_inner_io_error(format!("RangeDim ({} {}) didn't parse", parts[0], parts[1]).as_str()))
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

#[cfg(test)]
mod tests {
    use super::*;

    const CUBOIDS_1: [&'static str; 2] = ["x=10..12,y=10..12,z=10..12", "x=11..13,y=11..13,z=11..13"];

    #[test]
    fn test_simple_count() {
        for cuboid in CUBOIDS_1.iter() {
            let cuboid: Cuboid = cuboid.parse().unwrap();
            assert_eq!(cuboid.num_cubes(), 27);
        }
    }

    #[test]
    fn test_examples() {
        for (example, goal) in [("c", 46), ("a", 39), ("b", 590784)] {
            let actions = AllActions::from_file(format!("ex/day22{}.txt", example).as_str()).unwrap().part1();
            assert_eq!(actions.total_on(), goal);
        }
    }

    #[test]
    fn test_break_out_1() {
        let cuboids = CUBOIDS_1.iter().map(|s| s.parse::<Cuboid>().unwrap()).collect_vec();
        let intersection = cuboids[0].intersection(&cuboids[1]);
        println!("Intersection: {}", intersection.unwrap());
        let breaks = cuboids[0].break_out(&cuboids[1]).unwrap();
        for broken in breaks.iter() {
            println!("{}", broken);
        }
        let break_sum = breaks.iter().map(|b| b.num_cubes()).sum::<usize>();
        assert_eq!(break_sum, cuboids[0].num_cubes());
    }

    #[test]
    fn test_break_out_2() {
        let one = "x=10..10,y=11..12,z=11..12".parse::<Cuboid>().unwrap();
        let two = "x=9..11,y=9..11,z=9..11".parse::<Cuboid>().unwrap();
        let intersection = one.intersection(&two).unwrap();
        println!("Intersection: {}", intersection);
        let broken = one.break_out(&two).unwrap();
        println!("Broken out:");
        for cuboid in broken {
            println!("{}", cuboid);
        }
    }
}