use std::io;
use advent_code_lib::file2nums;

fn main() -> io::Result<()> {
    let depths = file2nums("in/day1.txt")?;
    let mut count = 0;
    for i in 0..depths.len() - 1 {
        if depths[i] < depths[i + 1] {
            count += 1;
        }
    }
    println!("{}", count);
    Ok(())
}
