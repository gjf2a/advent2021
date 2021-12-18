use std::fmt::{Display, Formatter};
use std::io;
use std::str::{Chars, FromStr};
use advent_code_lib::{advent_main, assert_io_error, assert_token, make_io_error};

fn main() -> io::Result<()> {
    advent_main(&[], &[], |args| {
        Ok(())
    })
}

enum SailfishNumber {
    Num(u32),
    Pair(Box<SailfishNumber>, Box<SailfishNumber>)
}

impl FromStr for SailfishNumber {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parser(&mut s.trim().chars())
    }
}

fn parser(chars: &mut Chars) -> io::Result<SailfishNumber> {
    if let Some(c) = chars.next() {
        Ok(if c.is_digit(10) {
            SailfishNumber::Num(c.to_digit(10).unwrap())
        } else {
            assert_io_error(c == '[', "No opening bracket")?;
            let left = Box::new(parser(chars)?);
            assert_token(chars.next(), ',')?;
            let right = Box::new(parser(chars)?);
            assert_token(chars.next(), ']')?;
            SailfishNumber::Pair(left, right)
        })
    } else {
        make_io_error("Out of chars!")
    }
}

impl Display for SailfishNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SailfishNumber::Num(num) => {write!(f, "{}", num)}
            SailfishNumber::Pair(sn1, sn2) => {
                write!(f, "[")?;
                write!(f, "{}", sn1)?;
                write!(f, ",")?;
                write!(f, "{}", sn2)?;
                write!(f, "]")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use advent_code_lib::all_lines;
    use super::*;

    #[test]
    fn parse_test() {
        for line in all_lines("ex/day18.txt").unwrap() {
            let parsed: SailfishNumber = line.parse().unwrap();
            let unparsed = format!("{}", parsed);
            assert_eq!(unparsed, line);
        }
    }
}