use std::io;
use std::collections::HashMap;
use advent_code_lib::{first_line_only_numbers, advent_main};

const FIRST: usize = 9;
const REST: usize = 7;

fn main() -> io::Result<()> {
    advent_main(&["duration"], &["table"], |args| {
        let fish_counters = first_line_only_numbers::<usize>(args[1].as_str())?;
        let duration: usize = args[2].parse().unwrap();
        let mut table = HashMap::new();
        let total = fish_counters.iter()
            .map(|f| total_fish_at(duration + FIRST - f - 1, &mut table))
            .sum::<u128>();
        if args.len() > 3 {display(&table);}
        println!("Total fish: {}", total);
        Ok(())
    })
}

fn total_fish_at(lifetime: usize, table: &mut HashMap<usize, u128>) -> u128 {
    1 + (FIRST..=lifetime)
        .step_by(REST)
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

fn display(table: &HashMap<usize, u128>) {
    let mut pairs = table.iter().collect::<Vec<_>>();
    pairs.sort();
    println!("{:?}", pairs);
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
            let total = total_fish_at(i, &mut table);
            println!("i: {} goal: {} total: {}", i, goal, total);
            assert_eq!(total, *goal);
        }
    }
}