use std::{env, io};
use std::collections::HashMap;
use advent_code_lib::all_lines;

const START: isize = 9;
const RESET: isize = 7;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: day6 filename duration");
    } else {
        let lines = all_lines(args[1].as_str())?.next().unwrap();
        let duration = args[2].parse::<isize>().unwrap();
        let mut table = HashMap::new();
        let total = lines.split(',')
            .map(|s| s.parse().unwrap())
            .map(|f: isize| total_fish_at(duration + START - f - 1, &mut table))
            .sum::<u128>();
        println!("Total fish: {}", total);
    }
    Ok(())
}

fn total_fish_at(lifetime: isize, table: &mut HashMap<isize, u128>) -> u128 {
    1 + (START..=lifetime)
        .step_by(RESET as usize)
        .map(|i| {
            let spawn_lifetime = lifetime - i;
            if let Some(count) = table.get(&spawn_lifetime) {
                *count
            } else {
                let result = total_fish_at(spawn_lifetime, table);
                table.insert(spawn_lifetime, result);
                result
            }

        })
        .sum::<u128>()
}

#[cfg(test)]
mod tests {
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
    // 18: 4, 6, 6, 8 (4)

    use super::*;

    #[test]
    fn test() {
        let mut table = HashMap::new();
        for (i, goal) in [1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 3, 3, 4].iter().enumerate() {
            let total = total_fish_at(i as isize, &mut table);
            println!("i: {} goal: {} total: {}", i, goal, total);
            assert_eq!(total, *goal);
        }
    }
}