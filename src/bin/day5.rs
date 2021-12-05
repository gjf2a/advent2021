use std::{env, io, iter};
use std::cmp::{max, min};
use std::str::FromStr;
use advent_code_lib::{all_lines, Position};
use hash_histogram::HashHistogram;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: day5.txt filename (1|2) [show]");
    } else {
        let with_diagonals = args[2] == "2";
        let show = args.len() == 4;
        let segments = all_lines(args[1].as_str())?
            .map(|line| line.parse::<LineSegment>())
            .collect::<Result<Vec<LineSegment>, io::Error>>()?;
        let (max_x, max_y) = dimension(&segments);
        let (width, height) = (max_x + 1, max_y + 1);
        let mut counts = HashHistogram::new();
        for segment in segments.iter() {
            for p in segment.points(with_diagonals) {
                counts.bump(&p);
            }
        }
        if show {print_diagram(&counts, width, height);}
        let score_part_1 = counts.iter()
            .filter(|(_, count)| **count >= 2)
            .count();
        println!("Score: {}", score_part_1);
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
pub struct LineSegment {
    start: Position,
    end: Position
}

impl LineSegment {
    // Returning an Iterator: https://stackoverflow.com/questions/27535289/what-is-the-correct-way-to-return-an-iterator-or-any-other-trait
    pub fn points<'a>(&'a self, with_diagonals: bool) -> Box<dyn Iterator<Item = Position> + 'a> {
        if self.start.row == self.end.row {
            let start = min(self.start.col, self.end.col);
            let end = max(self.start.col, self.end.col);
            Box::new((start..=end).map(|x| Position::from((x, self.start.row))))
        } else if self.start.col == self.end.col {
            let start = min(self.start.row, self.end.row);
            let end = max(self.start.row, self.end.row);
            Box::new((start..=end).map(|y| Position::from((self.start.col, y))))
        } else if with_diagonals && (self.start.col - self.end.col).abs() == (self.start.row - self.end.row).abs() {
            Box::new(DiagonalIterator::from(self.start, self.end))
        } else {
            Box::new(iter::empty::<Position>())
        }
    }
}

fn find_offset(start: isize, end: isize) -> isize {
    if start < end {1} else {-1}
}

struct DiagonalIterator {
    d: Position,
    current: Position,
    last: Position,
    active: bool
}

impl DiagonalIterator {
    fn from(start: Position, end: Position) -> Self {
        DiagonalIterator {d: Position::from((find_offset(start.col, end.col), find_offset(start.row, end.row))),
            current: start, last: end, active: true }
    }
}

impl Iterator for DiagonalIterator {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        if self.active {
            let result = self.current;
            self.active = self.current != self.last;
            self.current += self.d;
            Some(result)
        } else {
            None
        }
    }
}

impl FromStr for LineSegment {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Consider removing the unwrap() later.
        let parsed = s.split(" -> ")
            .map(|s| s.parse::<Position>())
            .collect::<Result<Vec<Position>, io::Error>>()
            .unwrap();
        Ok(LineSegment { start: parsed[0], end: parsed[1]})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagonal() {
        println!("{:?}", LineSegment::from_str("1,1 -> 3,3").unwrap().points(true).collect::<Vec<Position>>());
    }
}