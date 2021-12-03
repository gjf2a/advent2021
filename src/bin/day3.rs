use std::{env, io};
use advent_code_lib::all_lines;
use bits::BitArray;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: day3 filename");
    } else {
        let binary_nums: Vec<BitArray> = all_lines(args[1].as_str())?.map(|s| s.parse().unwrap()).collect();
        let width = binary_nums[0].len();
        let majority_min = binary_nums.len() / 2 + 1;
        let gamma: BitArray = (0..width)
            .map(|i| binary_nums.iter().filter(|b| b.is_set(i)).count() >= majority_min)
            .collect();
        let epsilon = !&gamma;
        let gamma_rate = u64::try_from(&gamma).unwrap();
        let epsilon_rate = u64::try_from(&epsilon).unwrap();
        println!("gamma: {} ({})", gamma, gamma_rate);
        println!("epsilon: {} ({})", epsilon, epsilon_rate);
        println!("product: {}", gamma_rate * epsilon_rate);
    }
    Ok(())
}