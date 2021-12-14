use std::collections::HashSet;
use std::io;
use advent_code_lib::{all_lines, advent_main, Position, RowMajorPositionIterator};

fn main() -> io::Result<()> {
    advent_main(&[], &["-show-start"], |args| {
        let (points, instructions) = parse_input(args[1].as_str())?;
        if args.len() > 2 {
            println!("Starting position");
            visualize(&points);
        }
        println!("Part 1 score: {}", one_fold_count(&points, &instructions));
        visualize_folded(&points, &instructions);
        Ok(())
    })
}

fn parse_input(filename: &str) -> io::Result<(HashSet<Position>, Vec<FoldInstruction>)> {
    let mut lines = all_lines(filename)?;
    let points: HashSet<Position> = lines.by_ref().take_while(|line| line.len() > 0).map(|line| line.parse().unwrap()).collect();
    let instructions: Vec<FoldInstruction> = lines.map(|line| FoldInstruction::from(line.as_str())).collect();
    Ok((points, instructions))
}

fn one_fold_count(points: &HashSet<Position>, instructions: &Vec<FoldInstruction>) -> usize {
    instructions[0].folded_points(points.iter()).len()
}

fn fold_all(points: &HashSet<Position>, instructions: &Vec<FoldInstruction>) -> HashSet<Position> {
    instructions.iter().fold(points.clone(), |prev, instr| instr.folded_points(prev.iter()))
}

fn visualize_folded(points: &HashSet<Position>, instructions: &Vec<FoldInstruction>) {
    visualize(&fold_all(points, instructions));
}

fn visualize(points: &HashSet<Position>) {
    let width = 1 + points.iter().map(|p| p.col as usize).max().unwrap();
    let height = 1 + points.iter().map(|p| p.row as usize).max().unwrap();
    for p in RowMajorPositionIterator::new(width, height) {
        if p.col == 0 && p.row > 0 {
            println!();
        }
        print!("{}", if points.contains(&p) {'#'} else {'.'});
    }
    println!();
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

    fn remapped_value(fold_point: isize, original: isize) -> isize {
        if original > fold_point {
            2 * fold_point - original
        } else {
            original
        }
    }

    fn folded_point(&self, p: Position) -> Position {
        Position::from(match self {
            FoldInstruction::Horizontal(y_fold) =>
                (p.col, FoldInstruction::remapped_value(*y_fold, p.row)),
            FoldInstruction::Vertical(x_fold) =>
                (FoldInstruction::remapped_value(*x_fold, p.col), p.row)
        })
    }

    fn folded_points<'a, I: Iterator<Item=&'a Position>>(&'a self, points: I) -> HashSet<Position> {
        points.map(|p| self.folded_point(*p)).collect()
    }
}


#[cfg(test)]
mod tests {
    use advent_code_lib::Position;
    use crate::FoldInstruction;

    #[test]
    fn test() {
        for (old, folder, folded) in [
            ((6, 10), FoldInstruction::Horizontal(7), (6, 4))
        ] {
            assert_eq!(Position::from(folded), folder.folded_point(Position::from(old)));
        }
    }
}