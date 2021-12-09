use std::{env, io};
use std::collections::{HashMap, HashSet, VecDeque};
use advent_code_lib::{all_lines, Position};

const MIN_SAFE_HEIGHT: u32 = 9;
const NUM_LARGEST_BASINS: usize = 3;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: day9 filename");
    } else {
        let heights = HeightMap::from(args[1].as_str())?;
        println!("Part 1: {}", heights.risk_level_sum());
        println!("Part 2: {}", heights.largest_basin_product());
    }
    Ok(())
}

struct HeightMap {
    heights: HashMap<Position, u32>
}

impl HeightMap {
    fn from(filename: &str) -> io::Result<Self> {
        let mut heights = HashMap::new();
        for (row, line) in all_lines(filename)?.enumerate() {
            for (col, height_char) in line.chars().enumerate() {
                heights.insert(Position::from((col as isize, row as isize)), height_char.to_digit(10).unwrap());
            }
        }
        Ok(HeightMap {heights})
    }

    fn risk_level_sum(&self) -> u32 {
        self.low_points().map(|(_, h)| h + 1).sum()
    }

    fn largest_basin_product(&self) -> usize {
        let mut basin_sizes: Vec<usize> = self.all_basin_sizes().collect();
        basin_sizes.sort();
        (1..=NUM_LARGEST_BASINS).map(|i| basin_sizes[basin_sizes.len() - i]).product()
    }

    fn low_points(&self) -> impl Iterator<Item=(Position,u32)> + '_ {
        self.heights.iter()
            .filter(|(p, h)| self.adjacent_location_heights(*p).all(|nh| nh > **h))
            .map(|(p, h)| (*p, *h))
    }

    fn basin_size_for(&self, p: &Position) -> usize {
        let mut open_list = VecDeque::new();
        let mut visited = HashSet::new();
        open_list.push_back(*p);
        loop {
            match open_list.pop_front() {
                None => break,
                Some(candidate) => {
                    if !visited.contains(&candidate) {
                        visited.insert(candidate);
                        for neighbor in candidate.manhattan_neighbors()
                            .filter(|n| self.heights.get(n).map_or(false, |h| *h < MIN_SAFE_HEIGHT)) {
                            open_list.push_back(neighbor);
                        }
                    }
                }
            }
        }
        visited.len()
    }

    fn all_basin_sizes(&self) -> impl Iterator<Item=usize> + '_ {
        self.low_points().map(|(low, _)| self.basin_size_for(&low))
    }

    fn adjacent_location_heights<'a>(&'a self, p: &'a Position) -> impl Iterator<Item=u32> + 'a {
        p.manhattan_neighbors().filter_map(|n| self.heights.get(&n).copied())
    }
}