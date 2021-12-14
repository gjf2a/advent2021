use std::io;
use advent_code_lib::{all_lines, advent_main};
use bits::BitArray;
use num::BigUint;

fn main() -> io::Result<()> {
    advent_main(&[], &[], |args| {
        let binary_nums: Vec<BitArray> = all_lines(args[1].as_str())?.map(|s| s.parse().unwrap()).collect();
        part1(&binary_nums)?;
        part2(&binary_nums)
    })
}

fn part1(binary_nums: &Vec<BitArray>) -> io::Result<()> {
    let gamma = find_gamma(binary_nums);
    show_results(&gamma, "gamma", &!&gamma, "epsilon")
}

fn find_gamma(binary_nums: &Vec<BitArray>) -> BitArray {
    let width = binary_nums[0].len();
    (0..width)
        .map(|i| one_most_common_at(i, binary_nums))
        .collect()
}

fn one_most_common_at(index: u64, binary_nums: &Vec<BitArray>) -> bool {
    let one_count = binary_nums.iter().filter(|b| b.is_set(index)).count();
    one_count >= binary_nums.len() - one_count
}

fn part2(binary_nums: &Vec<BitArray>) -> io::Result<()> {
    let oxygen_generator_rating = filter_using(false, binary_nums);
    let co2_scrubber_rating = filter_using(true, binary_nums);
    show_results(&oxygen_generator_rating, "oxygen generator rating", &co2_scrubber_rating, "CO2 scrubber rating")
}

fn filter_using(least_common: bool, binary_nums: &Vec<BitArray>) -> BitArray {
    let mut binary_nums = binary_nums.clone();
    let mut index = binary_nums[0].len();
    while binary_nums.len() > 1 {
        index -= 1;
        let target = one_most_common_at(index, &binary_nums) != least_common;
        binary_nums.retain(|b| b.is_set(index) == target);
    }
    binary_nums[0].clone()
}

fn show_results(num1: &BitArray, num1name: &str, num2: &BitArray, num2name: &str) -> io::Result<()> {
    let mut product = BigUint::from(1 as usize);
    for (num, name) in [(num1, num1name), (num2, num2name)] {
        let rate = BigUint::from(num);
        println!("{}: {} ({})", name, num, rate);
        product *= rate;
    }
    println!("product: {}", product);
    Ok(())
}