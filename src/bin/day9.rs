use std::{env, io};
use std::collections::HashMap;
use advent_code_lib::{all_lines, Position};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: day9 filename");
    } else {
        let heights = HeightMap::from(args[1].as_str())?;
        println!("Part 1: {}", heights.risk_level_sum());
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

    fn low_points(&self) -> impl Iterator<Item=(Position,u32)> + '_ {
        self.heights.iter()
            .filter(|(p, h)| self.adjacent_location_heights(*p).all(|nh| nh > **h))
            .map(|(p, h)| (*p, *h))
    }

    fn risk_level_sum(&self) -> u32 {
        self.low_points().map(|(_, h)| h + 1).sum()
    }

    fn adjacent_location_heights<'a>(&'a self, p: &'a Position) -> impl Iterator<Item=u32> + 'a {
        p.manhattan_neighbors().filter_map(|n| self.heights.get(&n).copied())
    }
}