use std::io;
use std::str::FromStr;
use advent_code_lib::{all_lines, advent_main, Position};
use hash_histogram::HashHistogram;

const MIN_OVERLAP: usize = 2;

fn main() -> io::Result<()> {
    advent_main(&["(1|2)"], &["show"], |args| {
        let (segments, with_diagonals, show) = segments_diagonals_show(&args)?;
        let counts = count_intersections(&segments, with_diagonals);
        if show {print_diagram(&counts, &segments);}
        println!("Score: {}", score(&counts));
        Ok(())
    })
}

fn segments_diagonals_show(args: &Vec<String>) -> io::Result<(Vec<LineSegment>, bool, bool)> {
    Ok((all_lines(args[1].as_str())?
        .map(|line| line.parse::<LineSegment>())
        .collect::<Result<Vec<LineSegment>, io::Error>>()?,
        args[2] == "2",
        args.len() == 4))
}

fn count_intersections(segments: &Vec<LineSegment>, with_diagonals: bool) -> HashHistogram<Position> {
    let mut counts = HashHistogram::new();
    for segment in segments.iter() {
        for p in segment.points(with_diagonals) {
            counts.bump(&p);
        }
    }
    counts
}

fn score(counts: &HashHistogram<Position>) -> usize {
    counts.iter()
        .filter(|(_, count)| **count >= MIN_OVERLAP)
        .count()
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct LineSegment {
    start: Position,
    end: Position
}

impl LineSegment {
    pub fn points(&self, with_diagonals: bool) -> LineSegmentPoints {
        if with_diagonals || self.start.row == self.end.row || self.start.col == self.end.col {
            LineSegmentPoints::from(self.start, self.end)
        } else {
            LineSegmentPoints::empty()
        }
    }
}

fn find_offset(start: isize, end: isize) -> isize {
    if start < end {1} else if start > end {-1} else {0}
}

pub struct LineSegmentPoints {
    d: Position,
    current: Position,
    last: Position,
    active: bool
}

impl LineSegmentPoints {
    fn from(start: Position, end: Position) -> Self {
        LineSegmentPoints {
            d: Position::from((find_offset(start.col, end.col),
                               find_offset(start.row, end.row))),
            current: start, last: end, active: true }
    }

    fn empty() -> Self {
        LineSegmentPoints {d: Position::from((0, 0)), current: Position::from((0, 0)),
            last: Position::from((0, 0)), active: false}
    }
}

impl Iterator for LineSegmentPoints {
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
        let parsed = s.split(" -> ")
            .map(|s| s.parse::<Position>())
            .collect::<Result<Vec<Position>, io::Error>>()?;
        Ok(LineSegment { start: parsed[0], end: parsed[1]})
    }
}

fn print_diagram(counts: &HashHistogram<Position>, segments: &Vec<LineSegment>) {
    let (max_x, max_y) = dimension(&segments);
    let (width, height) = (max_x + 1, max_y + 1);
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

fn dimension(segments: &Vec<LineSegment>) -> (usize, usize) {
    segments.iter().fold((0, 0), max_from)
}

fn max_from(acc: (usize, usize), seg: &LineSegment) -> (usize, usize) {
    let points = [
        (seg.start.col as usize, seg.start.row as usize),
        (seg.end.col as usize, seg.end.row as usize),
        acc];
    (points.iter().copied().map(|(x, _)| x).max().unwrap(),
     points.iter().copied().map(|(_, y)| y).max().unwrap())
}