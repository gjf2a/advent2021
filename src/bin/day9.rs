use std::io;
use std::collections::HashMap;
use advent_code_lib::{breadth_first_search, advent_main, Position, SearchQueue, nums2map, ContinueSearch};
use bare_metal_modulo::{MNum, ModNumC};

const MIN_SAFE_HEIGHT: u32 = 9;
const NUM_LARGEST_BASINS: usize = 3;

fn main() -> io::Result<()> {
    advent_main(&[], &[], |args| {
        let heights = HeightMap::from(args[1].as_str())?;
        println!("Part 1: {}", heights.risk_level_sum());
        println!("Part 2: {}", heights.largest_basin_product());
        Ok(())
    })
}

struct HeightMap {
    heights: HashMap<Position, ModNumC<u32, 10>>
}

impl HeightMap {
    fn from(filename: &str) -> io::Result<Self> {
        Ok(HeightMap {heights: nums2map(filename)?})
    }

    fn risk_level_sum(&self) -> u32 {
        self.low_points().map(|(_, h)| h.a() + 1).sum()
    }

    fn largest_basin_product(&self) -> usize {
        let mut basin_sizes: Vec<usize> = self.all_basin_sizes().collect();
        basin_sizes.sort();
        (1..=NUM_LARGEST_BASINS).map(|i| basin_sizes[basin_sizes.len() - i]).product()
    }

    fn low_points(&self) -> impl Iterator<Item=(Position,ModNumC<u32, 10>)> + '_ {
        self.heights.iter()
            .filter(|(p, h)| self.adjacent_location_heights(*p).all(|nh| nh > **h))
            .map(|(p, h)| (*p, *h))
    }

    fn basin_size_for(&self, p: &Position) -> usize {
        breadth_first_search(p,
                             |c, q| {
                                 for n in c.manhattan_neighbors()
                                     .filter(|n| self.heights.get(n)
                                         .map_or(false, |h| *h < MIN_SAFE_HEIGHT)) {
                                     q.enqueue(&n);
                                 }
                                 ContinueSearch::Yes
                             })
            .len()
    }

    fn all_basin_sizes(&self) -> impl Iterator<Item=usize> + '_ {
        self.low_points().map(|(low, _)| self.basin_size_for(&low))
    }

    fn adjacent_location_heights<'a>(&'a self, p: &'a Position) -> impl Iterator<Item=ModNumC<u32, 10>> + 'a {
        p.manhattan_neighbors().filter_map(|n| self.heights.get(&n).copied())
    }
}