use std::cmp::{max, min};
use std::collections::HashSet;
use std::{io, mem};
use std::fmt::{Display, Formatter};
use advent_code_lib::{advent_main, all_lines, Position, RowMajorPositionIterator};
use bits::BitArray;

const ON:  char = '#';
const OFF: char = '.';

fn main() -> io::Result<()> {
    advent_main(&[], &[], |args| {
        let mut lines = all_lines(args[1].as_str())?;
        let algorithm = read_enhancement_algorithm(lines.next().unwrap().as_str());
        lines.next();
        let image = read_image(&mut lines);
        let mut enhancer = ImageEnhancer::new(image, algorithm);
        for i in 0..=2 {
            let image = enhancer.next().unwrap();
            //println!("{}", image);
            println!("{}: {}", i, image.num_lit());
        }
        Ok(())
    })
}

fn read_enhancement_algorithm(line: &str) -> BitArray {
    let mut result = BitArray::new();
    for code in line.chars() {
        result.add(code == ON);
    }
    result
}

fn read_image<I: Iterator<Item=String>>(lines: &mut I) -> Image {
    let mut image = Image::new();
    for (row, line) in lines.enumerate() {
        for (col, bit) in line.chars().enumerate() {
            image.set(col as isize, row as isize, bit == ON);
        }
    }
    image
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Image {
    bits: HashSet<Position>,
    min_row: isize,
    min_col: isize,
    max_row: isize,
    max_col: isize
}

impl Image {
    fn new() -> Self {Image {bits: HashSet::new(), min_row: 0, min_col: 0, max_row: 0, max_col: 0}}

    fn lowest_pos(&self) -> Position {
        Position::from((self.min_col, self.min_row))
    }

    fn next_pos(&self, p: Position) -> Option<Position> {
        let mut updated = Position::from((p.col + 1, p.row));
        if updated.col > self.max_col {
            updated.col = self.min_col;
            updated.row += 1;
            if updated.row > self.max_row {
                return None
            }
        }
        Some(updated)
    }

    fn set(&mut self, col: isize, row: isize, value: bool) {
        let p = Position::from((col, row));
        if value {
            self.bits.insert(p);
        } else {
            self.bits.remove(&p);
        }
        self.min_row = min(row - 1, self.min_row);
        self.min_col = min(col - 1, self.min_col);
        self.max_row = max(row + 1, self.max_row);
        self.max_col = max(col + 1, self.max_col);
    }

    fn on(&self, p: Position) -> bool {
        self.bits.contains(&p)
    }

    fn neighborhood(&self, p: Position) -> u64 {
        let mut result = 0;
        let baseline = p - Position::from((1, 1));
        for pixel in RowMajorPositionIterator::new(3, 3) {
            let pixel = pixel + baseline;
            result *= 2;
            if self.on(pixel) {
                result += 1;
            }
        }
        result
    }

    fn num_lit(&self) -> usize {
        self.bits.len()
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut pixel = self.lowest_pos();
        loop {
            write!(f, "{}", if self.on(pixel) {ON} else {OFF})?;
            match self.next_pos(pixel) {
                None => return Ok(()),
                Some(next) => {
                    if next.col == self.min_col {writeln!(f)?;}
                    pixel = next;
                }
            }

        }
    }
}

struct ImageEnhancer {
    image: Image,
    algorithm: BitArray
}

impl ImageEnhancer {
    fn new(image: Image, algorithm: BitArray) -> Self {
        ImageEnhancer {image, algorithm}
    }
}

impl Iterator for ImageEnhancer {
    type Item = Image;

    fn next(&mut self) -> Option<Self::Item> {
        let mut updated = Image::new();
        let mut pixel = self.image.lowest_pos();
        loop {
            updated.set(pixel.col, pixel.row, self.algorithm.is_set(self.image.neighborhood(pixel)));
            match self.image.next_pos(pixel) {
                None => break,
                Some(next) => {pixel = next;}
            }
        }
        mem::swap(&mut self.image, &mut updated);
        Some(updated)
    }
}

#[cfg(test)]
mod tests {
    use advent_code_lib::Position;
    use crate::{read_enhancement_algorithm, read_image};

    const TEST_ALGORITHM: &'static str = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#";
    const TEST_IMAGE: &'static str = "#..#.
#....
##..#
..#..
..###";

    #[test]
    fn test1() {
        let algorithm = read_enhancement_algorithm(TEST_ALGORITHM);
        let image = read_image(&mut TEST_IMAGE.split_whitespace().map(|s| s.to_string()));
        assert_eq!(image.neighborhood(Position::from((2, 2))), 34);
    }
}