use std::{env, io};
use advent_code_lib::all_lines;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: day7 filename");
    } else {
        let mut positions = all_lines(args[1].as_str())?
            .next().unwrap().split(',').map(|s| s.parse().unwrap())
            .collect::<Vec<isize>>();
        positions.sort();
        report(1, &positions, part1_fuel_used_position);
        report(2, &positions, part2_fuel_used_position);
    }
    Ok(())
}

fn report(part: usize, positions: &Vec<isize>, fuel_used_position: fn(&Vec<isize>) -> (isize,isize)) {
    let (fuel_used, position) = fuel_used_position(positions);
    println!("Part {}: position: {} fuel used: {}", part, position, fuel_used);
}

fn part1_fuel_used_position(positions: &Vec<isize>) -> (isize, isize) {
    let median = positions[positions.len() / 2];
    (positions.iter().map(|p| part1_fuel_used(median, *p)).sum::<isize>(), median)
}

fn part2_fuel_used_position(positions: &Vec<isize>) -> (isize, isize) {
    (positions[0]..positions[positions.len() - 1]).map(|p| (part2_fuel_for(p, positions), p)).min().unwrap()
}

fn part2_fuel_for(position: isize, positions: &Vec<isize>) -> isize {
    positions.iter().map(|p| part2_fuel_used(position, *p)).sum()
}

fn part2_fuel_used(p1: isize, p2: isize) -> isize {
    let distance = part1_fuel_used(p1, p2);
    distance * (distance + 1) / 2
}

fn part1_fuel_used(p1: isize, p2: isize) -> isize {
    (p1 - p2).abs()
}