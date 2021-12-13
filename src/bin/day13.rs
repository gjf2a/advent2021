use std::collections::HashSet;
use std::io;
use advent_code_lib::{all_lines, generic_main, Position};

fn main() -> io::Result<()> {
    generic_main("day13", &[], &[], |args| {
        let (points, instructions) = parse_input(args[1].as_str())?;
        println!("{:?}", points);
        println!("{:?}", instructions);
        Ok(())
    })
}

fn parse_input(filename: &str) -> io::Result<(HashSet<Position>, Vec<FoldInstruction>)> {
    let mut lines = all_lines(filename)?;
    let points: HashSet<Position> = lines.by_ref().take_while(|line| line.len() > 0).map(|line| line.parse().unwrap()).collect();
    let instructions: Vec<FoldInstruction> = lines.map(|line| FoldInstruction::from(line.as_str())).collect();
    Ok((points, instructions))
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum FoldInstruction {
    Horizontal(isize),
    Vertical(isize)
}

impl FoldInstruction {
    fn from(line: &str) -> Self {
        let mut parts = line.split_whitespace().skip(2).next().unwrap().split('=');
        let x_or_y = parts.next().unwrap();
        let fold_spot = parts.next().unwrap().parse().unwrap();
        match x_or_y {
            "x" => FoldInstruction::Vertical(fold_spot),
            "y" => FoldInstruction::Horizontal(fold_spot),
            bad => panic!("Unknown pattern: {}", bad)
        }
    }
}