use std::{env, io};
use std::str::FromStr;
use advent_code_lib::{all_lines, Position, RowMajorPositionIterator};
use hash_histogram::HashHistogram;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: day5.txt filename");
    } else {
        let segments = all_lines(args[1].as_str())?
            .map(|line| line.parse::<LineSegment>())
            .collect::<Result<Vec<LineSegment>, io::Error>>()?;
        let (max_x, max_y) = dimension(&segments);
        let (width, height) = (max_x + 1, max_y + 1);
        let mut counts = HashHistogram::new();
        for segment in segments.iter() {
            for point in RowMajorPositionIterator::new(width, height)
                .filter(|p| segment.contains(*p)) {
                counts.bump(&point);
            }
        }
        //print_diagram(&counts, width, height);
        let score_part_1 = counts.iter()
            .filter(|(_, count)| **count >= 2)
            .count();
        println!("Part 1 score: {}", score_part_1);
    }
    Ok(())
}

fn dimension(segments: &Vec<LineSegment>) -> (usize, usize) {
    segments.iter().fold((0, 0), max_from)
}

fn print_diagram(counts: &HashHistogram<Position>, width: usize, height: usize) {
    for row in 0..height {
        print!("{}: ", row);
        for col in 0..width {
            let c = counts.count(&Position::from((col as isize, row as isize)));
            if c > 0 {
                print!("{}", c);
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn max_from(acc: (usize, usize), seg: &LineSegment) -> (usize, usize) {
    let points = [
        (seg.start.col as usize, seg.start.row as usize),
        (seg.end.col as usize, seg.end.row as usize),
        acc];
    (points.iter().copied().map(|(x, _)| x).max().unwrap(),
     points.iter().copied().map(|(_, y)| y).max().unwrap())
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct LineSegment {
    start: Position,
    end: Position
}

impl LineSegment {
    fn contains(&self, p: Position) -> bool {
        self.start.col == self.end.col && p.col == self.start.col && between(self.start.row, self.end.row, p.row)||
            self.start.row == self.end.row && p.row == self.start.row && between(self.start.col, self.end.col, p.col)
    }
}

fn between(b1: isize, b2: isize, value: isize) -> bool {
    if b1 < b2 {
        value >= b1 && value <= b2
    } else {
        value >= b2 && value <= b1
    }
}

impl FromStr for LineSegment {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: Remove the unwrap() later.
        let parsed = s.split(" -> ")
            .map(|s| s.parse::<Position>())
            .collect::<Result<Vec<Position>, io::Error>>()
            .unwrap();
        Ok(LineSegment { start: parsed[0], end: parsed[1]})
    }
}