use std::cmp::{max, min};
use std::collections::HashMap;
use std::{io, iter, mem};
use std::fmt::{Display, Formatter};
use advent_code_lib::{advent_main, all_lines, OffsetRowMajorPositionIterator, Position, RowMajorPositionIterator};
use bits::BitArray;

const ON:  char = '#';
const OFF: char = '.';
const PART_1_ITER: usize = 2;
const PART_2_ITER: usize = 50;
const BORDER: isize = 1;
const SHOW: &'static str = "-show";

fn main() -> io::Result<()> {
    advent_main(&["(1|2)"], &[SHOW], |args| {
        let mut lines = all_lines(args[1].as_str())?;
        let algorithm = read_enhancement_algorithm(lines.next().unwrap().as_str());
        lines.next();
        let image = read_image(&mut lines);
        let part = args[2].as_str();
        let iterations = 1 + if part == "1" {PART_1_ITER} else {PART_2_ITER};

        let lit = ImageEnhancer::new(image, algorithm)
            .take(iterations).enumerate()
            .inspect(|(i, image)| {
                if args.contains(&SHOW.to_string()) {
                    println!("After step {}", i);
                    println!("{}", image);
                    println!("{:?}", image.num_lit());
                }
            })
            .last().map(|(_, image)| image.num_lit().unwrap());
        println!("Part {}: {}", part, lit.unwrap());
        Ok(())
    })
}

fn code2pixel(c: char) -> bool {
    c == ON
}

fn pixel2code(pixel: bool) -> char {
    if pixel {ON} else {OFF}
}

fn read_enhancement_algorithm(line: &str) -> BitArray {
    let mut result = BitArray::new();
    for code in line.chars() {
        result.add(code2pixel(code));
    }
    assert_eq!(result.len(), 512);
    result
}

fn read_image<I: Iterator<Item=String>>(lines: &mut I) -> Image {
    let mut image = Image::new();
    for (row, line) in lines.enumerate() {
        for (col, code) in line.chars().enumerate() {
            image.set(col as isize, row as isize, code2pixel(code));
        }
    }
    image
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Image {
    bits: HashMap<Position,bool>,
    infinite_expanse: bool,
    min_row: isize,
    min_col: isize,
    max_row: isize,
    max_col: isize
}

impl Image {
    fn new() -> Self {Image {bits: HashMap::new(), infinite_expanse: false, min_row: 0, min_col: 0, max_row: 0, max_col: 0}}

    fn lowest_pos(&self) -> Position {
        Position::from((self.min_col, self.min_row))
    }

    fn width(&self) -> usize {
        (self.max_col - self.min_col + 1) as usize
    }

    fn set(&mut self, col: isize, row: isize, value: bool) {
        let p = Position::from((col, row));
        self.bits.insert(p, value);
        self.min_row = min(row, self.min_row);
        self.min_col = min(col, self.min_col);
        self.max_row = max(row, self.max_row);
        self.max_col = max(col, self.max_col);
    }

    fn on(&self, p: Position) -> bool {
        self.bits.get(&p).map_or(self.infinite_expanse, |b| *b)
    }

    fn encoded(&self, p: Position) -> char {
        pixel2code(self.on(p))
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

    fn num_lit(&self) -> Option<usize> {
        if self.infinite_expanse {
            None
        } else {
            Some(self.bits.values().filter(|v| **v).count())
        }
    }

    fn infinite_row(&self) -> String {
        iter::repeat(self.infinite_char()).take(self.width() + 2).collect()
    }

    fn infinite_char(&self) -> char {
        pixel2code(self.infinite_expanse)
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}", self.infinite_row(), self.infinite_char())?;
        for pixel in OffsetRowMajorPositionIterator::new(self.min_col, self.min_row, self.max_col, self.max_row) {
            write!(f, "{}", self.encoded(pixel))?;
            if pixel.col == self.max_col {
                writeln!(f, "{}", self.infinite_char())?;
                if pixel.row < self.max_row {
                    write!(f, "{}", self.infinite_char())?;
                }
            }
        }
        writeln!(f, "{}", self.infinite_row())?;
        Ok(())
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
        for pixel in OffsetRowMajorPositionIterator::new(self.image.min_col - BORDER,
                                                         self.image.min_row - BORDER,
                                                         self.image.max_col + BORDER,
                                                         self.image.max_row + BORDER) {
            updated.set(pixel.col, pixel.row, self.algorithm.is_set(self.image.neighborhood(pixel)));
        }
        updated.infinite_expanse = updated.on(updated.lowest_pos());
        mem::swap(&mut self.image, &mut updated);
        Some(updated)
    }
}

#[cfg(test)]
mod tests {
    use advent_code_lib::Position;
    use crate::read_image;

    const TEST_IMAGE: &'static str = "#..#.
#....
##..#
..#..
..###";

    #[test]
    fn test1() {
        let image = read_image(&mut TEST_IMAGE.split_whitespace().map(|s| s.to_string()));
        assert_eq!(image.neighborhood(Position::from((2, 2))), 34);
    }
}