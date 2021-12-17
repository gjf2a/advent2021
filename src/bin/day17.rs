use std::io;
use std::str::FromStr;
use advent_code_lib::advent_main;

fn main() -> io::Result<()> {
    advent_main(&[], &[], |args| {
        Ok(())
    })
}

struct TargetZone {
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize
}

impl FromStr for TargetZone {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut top = s.split_whitespace();
        let x_str = top.skip(1).by_ref().next().unwrap();
        let y_str = top.next().unwrap();

    }
}

fn range2values(range: &str) -> (isize, isize) {
    let start = 1 + range.chars().enumerate().find(|(i, c)| *c == '=').map(|(i,_)| i).unwrap();

}

impl TargetZone {

}

fn simulate_until(dx: isize, dy: isize, )

fn find_dx_from(target_x: isize) -> f64 {
    (((1 + 8 * target_x) as f64).sqrt() - 1.0) / 2.0
}

#[cfg(test)]
mod tests {
    use crate::find_dx_from;

    #[test]
    fn experiment() {
        for x in 20..=30 {
            println!("fdx for {}: {}", x, find_dx_from(x));
        }
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