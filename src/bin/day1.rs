use std::io;
use advent_code_lib::{advent_main, file2nums};

fn main() -> io::Result<()> {
    advent_main(&["window_width"], &[], |args| {
        let depths = file2nums(format!("{}", args[1]).as_str())?;
        let mut count = 0;
        let window_width = args[2].parse::<usize>().unwrap();
        for i in 0..depths.len() - window_width {
            if window_total(&depths, i, window_width) < window_total(&depths, i + 1, window_width) {
                count += 1;
            }
        }
        println!("{}", count);
        Ok(())
    })
}

fn window_total(depths: &Vec<isize>, i: usize, window_width: usize) -> isize {
    (i..i+window_width).map(|j| depths[j]).sum()
}