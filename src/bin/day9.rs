use advent_code_lib::{breadth_first_search, simpler_main, Position, SearchQueue, ContinueSearch, GridDigitWorld};
use bare_metal_modulo::{MNum, ModNumC};

const MIN_SAFE_HEIGHT: u8 = 9;
const NUM_LARGEST_BASINS: usize = 3;

fn main() -> anyhow::Result<()> {
    simpler_main(|filename| {
        let heights = HeightMap::from(filename)?;
        println!("Part 1: {}", heights.risk_level_sum());
        println!("Part 2: {}", heights.largest_basin_product());
        Ok(())
    })
}

struct HeightMap {
    heights: GridDigitWorld
}

impl HeightMap {
    fn from(filename: &str) -> anyhow::Result<Self> {
        Ok(HeightMap {heights: GridDigitWorld::from_digit_file(filename)?})
    }

    fn risk_level_sum(&self) -> u8 {
        self.low_points().map(|(_, h)| h.a() + 1).sum()
    }

    fn largest_basin_product(&self) -> usize {
        let mut basin_sizes: Vec<usize> = self.all_basin_sizes().collect();
        basin_sizes.sort();
        (1..=NUM_LARGEST_BASINS).map(|i| basin_sizes[basin_sizes.len() - i]).product()
    }

    fn low_points(&self) -> impl Iterator<Item=(Position,ModNumC<u8, 10>)> + '_ {
        self.heights.position_value_iter()
            .filter(|(p, h)| self.adjacent_location_heights(*p).all(|nh| nh > **h))
            .map(|(p, h)| (*p, *h))
    }

    fn basin_size_for(&self, p: &Position) -> usize {
        breadth_first_search(p,
                             |c, q| {
                                 for n in c.manhattan_neighbors()
                                     .filter(|n| self.heights.value(*n)
                                         .map_or(false, |h| h < MIN_SAFE_HEIGHT)) {
                                     q.enqueue(&n);
                                 }
                                 ContinueSearch::Yes
                             })
            .len()
    }

    fn all_basin_sizes(&self) -> impl Iterator<Item=usize> + '_ {
        self.low_points().map(|(low, _)| self.basin_size_for(&low))
    }

    fn adjacent_location_heights<'a>(&'a self, p: &'a Position) -> impl Iterator<Item=ModNumC<u8, 10>> + 'a {
        p.manhattan_neighbors().filter_map(|n| self.heights.value(n))
    }
}