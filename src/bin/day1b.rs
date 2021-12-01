use std::io;
use advent_code_lib::file2nums;

fn window_total(depths: &Vec<isize>, i: usize, window_width: usize) -> isize {
    (i..i+window_width).map(|j| depths[j]).sum()
}

fn main() -> io::Result<()> {
    let depths = file2nums("in/day1.txt")?;
    let mut count = 0;
    let window_width = 3;
    for i in 0..depths.len() - window_width {
        if window_total(&depths, i, window_width) < window_total(&depths, i + 1, window_width) {
            count += 1;
        }
    }
    println!("{}", count);
    Ok(())
}