use std::{env, io};
use advent_code_lib::all_lines;

const START: isize = 8;
const RESET: isize = 6;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: day6 filename duration");
    } else {
        let lines = all_lines(args[1].as_str())?.next().unwrap();
        let duration = args[2].parse::<isize>().unwrap();
        let total = lines.split(',')
            .map(|s| s.parse().unwrap())
            .map(|f: isize| total_fish_at(duration + START - f))
            .sum::<u128>();
        println!("Part 1 solution: {}", total);
    }
    Ok(())
}


// 0: 8 (1)
// 1: 7 (1)
// 2: 6 (1)
// 3: 5 (1)
// 4: 4 (1)
// 5: 3 (1)
// 6: 2 (1)
// 7: 1 (1)
// 8: 0 (1)
// 9: 6, 8 (2)
// 10: 5, 7 (2)
// 11: 4, 6 (2)
// 12: 3, 5 (2)
// 13: 2, 4 (2)
// 14: 1, 3 (2)
// 15: 0, 2 (2)
// 16: 6, 1, 8 (3)
// 17: 5, 0, 7 (3)
// 18: 4, 6, 6 (4)

// Basic spawning: 1 + (timestamp - 2) % 7
// Recursive spawning: At (timestamp - 2) % 7 == 0, recursion(

fn total_fish_at(timestamp: isize) -> u128 {
    let mut total = 1;
    for i in (0..=timestamp).rev() {
        if (i - (START - RESET)) % (RESET + 1) == 0 {
            total += total_fish_at(i)
        }
    }
    total
}

mod tests {
    use super::*;

    #[test]
    fn base_test() {
        for (i, goal) in [1, 1].iter().enumerate() {
            assert_eq!(total_fish_at(i as isize), *goal);
        }
    }

    #[test]
    fn failing_test() {
        for (i, goal) in [1, 1, 1].iter().enumerate() {
            assert_eq!(total_fish_at(i as isize), *goal);
        }
    }

    #[test]
    fn test() {
        for (i, goal) in [1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 3, 3, 4].iter().enumerate() {
            assert_eq!(total_fish_at(i as isize), *goal);
        }
    }
}