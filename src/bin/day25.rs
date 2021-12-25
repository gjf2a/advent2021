use std::cmp::max;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::io;
use advent_code_lib::{advent_main, all_lines, make_io_error, Position, RowMajorPositionIterator};
use itertools::Itertools;

fn main() -> io::Result<()> {
    advent_main(&[], &[], |args| {
        println!("Part 1: {}", part1(args[1].as_str())?);
        Ok(())
    })
}

fn part1(filename: &str) -> io::Result<usize> {
    Ok(Cucumbers::from_file(filename)?.iter().count())
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Cucumber {
    East, South, Empty
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Cucumbers {
    grid: HashMap<Position, Cucumber>,
    width: isize,
    height: isize
}

struct CIterator {
    value: Option<Cucumbers>
}

impl Cucumbers {
    fn from_file(filename: &str) -> io::Result<Self> {
        Self::from_iter(all_lines(filename)?)
    }

    fn from_iter<I: Iterator<Item=String>>(iter: I) -> io::Result<Self> {
        let mut grid = HashMap::new();
        let mut width = 0;
        let mut height = 0;
        for (row, line) in iter.enumerate() {
            let row = row as isize;
            height = max(height, row);
            for (col, c) in line.chars().enumerate() {
                let col = col as isize;
                width = max(width, col);
                grid.insert(Position::from((col, row)), match c {
                    '>' => Cucumber::East,
                    'v' => Cucumber::South,
                    '.' => Cucumber::Empty,
                    _ => make_io_error("Not a cucumber")?
                });
            }
        }
        width += 1;
        height += 1;
        Ok(Cucumbers {grid, width, height})
    }

    fn iter(&self) -> CIterator {
        CIterator {value: Some(self.clone())}
    }

    fn move_all(&mut self, dir: Cucumber) {
        let movers = self.grid.iter()
            .filter(|(_, c)| **c == dir)
            .filter_map(|(p, c)| c.next_spot(*p, self).map(|n| (*c, *p, n)))
            .collect_vec();
        for (mover, start, end) in movers {
            self.grid.insert(end, mover);
            self.grid.insert(start, Cucumber::Empty);
        }
    }
}

impl Cucumber {
    fn next_spot(&self, p: Position, grid: &Cucumbers) -> Option<Position> {
        match self {
            Cucumber::Empty => None,
            Cucumber::East => Some(Position::from(((p.col + 1) % grid.width, p.row))),
            Cucumber::South => Some(Position::from((p.col, (p.row + 1) % grid.height)))
        }.filter(|n| grid.grid.get(n).map_or(false, |c| *c == Cucumber::Empty))
    }
}

impl Iterator for CIterator {
    type Item = Cucumbers;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.value.clone();
        self.value = self.value.as_ref().and_then(|current| {
            let mut future = current.clone();
            future.move_all(Cucumber::East);
            future.move_all(Cucumber::South);
            if future == *current {None} else {Some(future)}
        });
        result
    }
}

impl Display for Cucumbers {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for p in RowMajorPositionIterator::new(self.width as usize, self.height as usize) {
            if p.col == 0 && p.row > 0 {
                writeln!(f)?;
            }
            write!(f, "{}", self.grid.get(&p).unwrap())?;
        }
        writeln!(f)
    }
}

impl Display for Cucumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {Cucumber::East => '>', Cucumber::South => 'v', Cucumber::Empty => '.'})
    }
}

#[cfg(test)]
mod tests {
    use crate::{Cucumbers, part1};

    #[test]
    fn test_io() {
        let grid = Cucumbers::from_file("ex/day25.txt").unwrap();
        let copy = Cucumbers::from_iter(format!("{}", grid).split_whitespace().map(|line| line.to_owned())).unwrap();
        assert_eq!(grid.width, copy.width);
        assert_eq!(grid.height, copy.height);
        assert_eq!(grid, copy);
    }

    #[test]
    fn test_one_iteration() {
        let grid = Cucumbers::from_file("ex/day25.txt").unwrap();
        let next = Cucumbers::from_file("ex/day25_1.txt").unwrap();
        assert_eq!(grid.iter().skip(1).next().unwrap(), next);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(58, part1("ex/day25.txt").unwrap());
    }
}