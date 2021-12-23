use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::io;
use std::str::FromStr;
use advent_code_lib::{advent_main, all_lines, make_io_error, Position};
use bare_metal_modulo::{MNum, ModNumC};

const ENERGY_BASE: u128 = 10;
const MIN_AMPHIPOD: char = 'A';
const MAX_AMPHIPOD: char = 'D';
const NUM_AMPHIPOD_TYPES: usize = MAX_AMPHIPOD as usize - MIN_AMPHIPOD as usize + 1;
const EMPTY_SQUARE: char = '.';
const WALL_SQUARE: char = '#';
const IGNORE_SQUARES: [char; 2] = [WALL_SQUARE, ' '];

fn main() -> io::Result<()> {
    advent_main(&[], &[], |args| {
        Ok(())
    })
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
struct Amphipod {
    abcd: ModNumC<u32, NUM_AMPHIPOD_TYPES>
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct AmphipodMap {
    space2amphipod: HashMap<Position, Option<Amphipod>>,
    width: isize,
    height: isize
}

impl Amphipod {
    fn new(code: char) -> io::Result<Amphipod> {
        if code < MIN_AMPHIPOD || code > MAX_AMPHIPOD {
            make_io_error(format!("Illegal Amphipod: {}", code).as_str())
        } else {
            Ok(Amphipod {abcd: ModNumC::new(code as u32 - MIN_AMPHIPOD as u32)})
        }
    }

    fn step_energy(&self) -> u128 {ENERGY_BASE.pow(self.abcd.a())}
}

impl From<Amphipod> for char {
    fn from(amphipod: Amphipod) -> Self {
        (amphipod.abcd.a() as u8 + MIN_AMPHIPOD as u8) as char
    }
}

impl AmphipodMap {
    fn from_file(filename: &str) -> io::Result<Self> {
        Self::from_iter(all_lines(filename)?)
    }

    fn from_iter<I: Iterator<Item=String>>(lines: I) -> io::Result<Self> {
        let mut space2amphipod = HashMap::new();
        for (row, line) in lines.enumerate() {
            for (col, hall_char) in line.chars().enumerate() {
                decode_square(hall_char, Position::from((col as isize, row as isize)), &mut space2amphipod)?;
            }
        }
        let width = space2amphipod.keys().map(|k| k.col).max().unwrap() + 1;
        let height = space2amphipod.keys().map(|k| k.row).max().unwrap() + 1;
        Ok(AmphipodMap {space2amphipod, width, height})
    }
}

impl Display for Amphipod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}

impl Display for AmphipodMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                let p = Position::from((col, row));
                let c = self.space2amphipod.get(&p).map_or(WALL_SQUARE, |s| s.map_or(EMPTY_SQUARE, |a| char::from(a)));
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn decode_square(square: char, p: Position, space2amphipod: &mut HashMap<Position, Option<Amphipod>>) -> io::Result<()> {
    if square == EMPTY_SQUARE {
        space2amphipod.insert(p, None);
    } else if !IGNORE_SQUARES.contains(&square) {
        space2amphipod.insert(p, Some(Amphipod::new(square)?));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{Amphipod, AmphipodMap};

    #[test]
    fn test_read_amphipod() {
        for (code, energy) in [('A', 1), ('B', 10), ('C', 100), ('D', 1000)] {
            let amphipod = Amphipod::new(code).unwrap();
            assert_eq!(amphipod.step_energy(), energy);
            println!("{}", amphipod);
        }
    }

    #[test]
    fn test_read_map() {
        let map = AmphipodMap::from_file("ex/day23.txt").unwrap();
        println!("{}", map);
        let map_str = format!("{}", map);
        let map2 = AmphipodMap::from_iter(map_str.split_whitespace().map(|s| s.to_string())).unwrap();
        assert_eq!(map, map2);
    }
}