use std::cmp::max;
use std::io;
use std::str::FromStr;
use advent_code_lib::{advent_main, all_lines};

fn main() -> io::Result<()> {
    advent_main(&[], &[], |args| {
        let line = all_lines(args[1].as_str())?.next().unwrap();
        let zone: TargetZone = line.parse().unwrap();
        let highest = zone.find_best_launch();
        println!("Part 1: {}", highest);
        Ok(())
    })
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct TargetZone {
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize
}

impl FromStr for TargetZone {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums = extract_nums_from(s);
        Ok(TargetZone {min_x: nums[0], max_x: nums[1], min_y: nums[2], max_y: nums[3]})
    }
}

impl TargetZone {
    fn within(&self, x: isize, y: isize) -> bool {
        self.min_x <= x && self.max_x >= x && self.min_y <= y && self.max_y >= y
    }

    fn below(&self, y: isize) -> bool {
        y < self.min_y
    }

    fn find_best_launch(&self) -> isize {
        ((find_dx_from(self.min_x) as isize)..(self.max_x + 1))
            .map(|dx| self.best_height_using(dx))
            .max().unwrap()
    }

    fn best_height_using(&self, dx: isize) -> isize {
        let mut highest = 0;
        let max_x = find_max_x_from(dx);
        for dy in self.min_y.. {
            let (points, height) = self.simulate(dx, dy);
            if let Some(height) = height {
                if highest < height {
                    highest = height;
                }
            } else {
                let (end_x, _) = points.last().unwrap();
                if *end_x == max_x {
                    break;
                }
            }
        }
        highest
    }

    fn simulate(&self, mut dx: isize, mut dy: isize) -> (Vec<(isize, isize)>, Option<isize>) {
        let mut points = vec![(0, 0)];
        let mut max_y = 0;
        loop {
            let (x, y) = points[points.len() - 1];
            max_y = max(max_y, y);
            if self.within(x, y) {
                return (points, Some(max_y));
            }
            if self.below(y) {
                return (points, None);
            }
            points.push((x + dx, y + dy));
            dx = if dx > 0 {dx - 1} else if x < 0 {dx + 1} else {0};
            dy -= 1;
        }
    }
}

fn extract_nums_from(input: &str) -> Vec<isize> {
    let mut nums = Vec::new();
    let mut current_num = String::new();
    let mut negative = false;
    for c in input.chars() {
        if c.is_digit(10) {
            current_num.push(c);
        } else if c == '-' {
            negative = true;
        } else {
            if !current_num.is_empty() {
                nums.push(add_num(current_num.as_str(), negative));
                current_num = String::new();
                negative = false;
            }
        }
    }
    if !current_num.is_empty() {
        nums.push(add_num(current_num.as_str(), negative));
    }
    nums
}

fn add_num(num: &str, negative: bool) -> isize {
    let num: isize = num.parse().unwrap();
    if negative {-num} else {num}
}

fn find_dx_from(target_x: isize) -> f64 {
    (((1 + 8 * target_x) as f64).sqrt() - 1.0) / 2.0
}

fn find_max_x_from(dx: isize) -> isize {
    dx * (dx + 1) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn experiment() {
        for x in 20..=30 {
            println!("fdx for {}: {} ({})", x, find_dx_from(x), find_dx_from(x) as isize);
        }
    }

    #[test]
    fn num_from_test() {
        assert_eq!(extract_nums_from(EXAMPLE), vec![20, 30, -10, -5]);
    }

    #[test]
    fn test_max_x() {
        for (dx, mdx) in [(5, 15), (6, 21), (7, 28), (8, 36)] {
            assert_eq!(find_max_x_from(dx), mdx);
        }
    }

    #[test]
    fn test_simulate() {
        let target: TargetZone = EXAMPLE.parse().unwrap();
        for ((dx, dy), outcome) in [((7, 2), Some(3)), ((6, 3), Some(6)), ((9, 0), Some(0)), ((17, -4), None), ((6, 9), Some(45))] {
            assert_eq!(target.simulate(dx, dy).1, outcome);
        }
    }

    #[test]
    fn test_find() {
        let target: TargetZone = EXAMPLE.parse().unwrap();
        let best = target.find_best_launch();
        assert_eq!(best, 45);
    }
}

// Looks like some kind of constraint satisfaction problem.
//
// Given a target zone and an initial position of (0, 0), maximize y while landing
// in the target zone.
//
// Find dx and dy such that:
// * y is maximized
// * There exists an (x, y) point within the target zone
//
// Break this into pieces:
// * How do we set dx to ensure landing in the target zone?
// * How do we set dy to ensure landing in the target zone?
// * How do we maximize y?
//
// Note: My puzzle input has positive x and negative y.

// 1. Setting dx to ensure landing in the target x zone. (min_x, max_x)
// min_x <= dx + (dx - 1) + (dx - 2) + (dx - 3) + ... + 2 + 1 <= max_x
// min_x <= dx (dx + 1) / 2 <= max_x
// dx (dx + 1) / 2 = c
// dx^2 + dx - 2c = 0
// dx = -1 +/- sqrt(1 + 8c) / 2
// We know dx has to be positive, so...
// dx = (sqrt(1 + 8c) - 1) / 2
// Solving the inequality gives min and max values for dx
// From my experiment above:
// * This does give legit values for dx
// * But it misses a lot, because the summation is not always complete!
// * It does give a legitimate minimum dx; below that, it will never reach.
//
// 2. Setting dy to ensure landing in the target y zone. (min_y, max_y)
// * use the dx values derived in (1) to constrain what we do here.
// * Given a specific dx value:
//   * This represents a time budget for dy.
//   * y has to get in the target range within the time implied by dx
//   * It can't be so big that it overshoots.
//   * Min value for dy when it hits the target zone is -(min_y - max_y)
//   * Reaching min value implies maximum buildup, hence maximum height
//
// for each possible dx value
//   Calculate target dy value
//   Work backwards to find maximum height
//   Work further backwards to find initial (dx, dy)