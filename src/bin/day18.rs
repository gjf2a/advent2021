use std::fmt::{Display, Formatter};
use std::io;
use std::ops::Add;
use std::str::{Chars, FromStr};
use advent_code_lib::{advent_main, all_lines, assert_io_error, assert_token, make_io_error};

const SHOW: &'static str = "-show";

fn main() -> io::Result<()> {
    advent_main(&[], &[SHOW], |args| {
        let total = all_lines(args[1].as_str())?
            .map(|line| line.parse::<SailfishNumber>().unwrap())
            .inspect(|sn| if args.contains(&SHOW.to_string()) {println!("{}", sn);})
            .reduce(|a, b| &a + &b)
            .unwrap();
        println!("total: {}", total);
        println!("Part 1: {}", total.magnitude());
        Ok(())
    })
}

#[derive(Clone, Eq, PartialEq, Debug)]
enum SailfishNumber {
    Num(u32),
    Pair(Box<SailfishNumber>, Box<SailfishNumber>)
}

impl SailfishNumber {
    fn magnitude(&self) -> u32 {
        match self {
            SailfishNumber::Num(n) => *n,
            SailfishNumber::Pair(sn1, sn2) => 3 * sn1.magnitude() + 2 * sn2.magnitude()
        }
    }

    fn split(&self) -> (SailfishNumber, bool) {
        self.split_leftmost()
    }

    fn split_leftmost(&self) -> (SailfishNumber, bool) {
        match self {
            SailfishNumber::Num(num) => {
                if *num > 9 {
                    let low_half = num / 2;
                    let high_half = low_half + num % 2;
                    (SailfishNumber::Pair(Box::new(SailfishNumber::Num(low_half)), Box::new(SailfishNumber::Num(high_half))), true)
                } else {
                    (self.clone(), false)
                }
            }
            SailfishNumber::Pair(left, right) => {
                let (updated_left, did_split) = left.split_leftmost();
                if did_split {
                    (SailfishNumber::Pair(Box::new(updated_left), right.clone()), true)
                } else {
                    let (updated_right, did_split) = right.split_leftmost();
                    (SailfishNumber::Pair(Box::new(updated_left), Box::new(updated_right)), did_split)
                }
            }
        }
    }

    fn reduced(&self) -> Self {
        let mut result = self.clone();
        //println!("Reducing {}", result);
        loop {
            let (exploded, explode_needed, _, _) = result.exploded(0);
            //println!("Exploded {}", exploded);
            if explode_needed {
                result = exploded;
            } else {
                let (split, split_needed) = exploded.split();
                //println!("Split {}", split);
                result = split;
                if !split_needed {
                    //println!("Done");
                    return result;
                }
            }
        }
    }

    fn unwrap_num(&self) -> u32 {
        match self {
            SailfishNumber::Num(num) => *num,
            _ => panic!("This should never happen!")
        }
    }

    fn is_num(&self) -> bool {
        match self {
            SailfishNumber::Num(_) => true,
            SailfishNumber::Pair(_, _) => false
        }
    }

    fn explodable(&self) -> bool {
        match self {
            SailfishNumber::Num(_) => false,
            SailfishNumber::Pair(a, b) => {
                a.is_num() && b.is_num()
            }
        }
    }

    fn exploded(&self, depth: usize) -> (SailfishNumber, bool, Option<u32>, Option<u32>) {
        match self {
            SailfishNumber::Num(_) => (self.clone(), false, None, None),
            SailfishNumber::Pair(sn1, sn2) => {
                if depth >= 4 && self.explodable() {
                    (SailfishNumber::Num(0), true, Some(sn1.unwrap_num()), Some(sn2.unwrap_num()))
                } else {
                    let (left, did_explode, left_num, right_num) = sn1.exploded(depth + 1);
                    if did_explode {
                        let right = if let Some(right_num) = right_num {
                            Box::new(sn2.leftmost_added(right_num))
                        } else {
                            sn2.clone()
                        };
                        (SailfishNumber::Pair(Box::new(left), right), true, left_num, None)
                    } else {
                        let (right, did_explode, left_num, right_num) = sn2.exploded(depth + 1);
                        if did_explode {
                            let thing = if let Some(left_num) = left_num {
                                Box::new(sn1.rightmost_added(left_num))
                            } else {
                                sn1.clone()
                            };
                            (SailfishNumber::Pair(thing, Box::new(right)), true, None, right_num)
                        } else {
                            (self.clone(), false, None, None)
                        }
                    }
                }
            }
        }
    }

    fn rightmost_added(&self, value: u32) -> Self {
        match self {
            SailfishNumber::Num(num) => SailfishNumber::Num(*num + value),
            SailfishNumber::Pair(left, right) =>
                SailfishNumber::Pair(left.clone(), Box::new(right.rightmost_added(value)))
        }
    }

    fn leftmost_added(&self, value: u32) -> Self {
        match self {
            SailfishNumber::Num(num) => SailfishNumber::Num(*num + value),
            SailfishNumber::Pair(left, right) =>
                SailfishNumber::Pair(Box::new(left.leftmost_added(value)), right.clone())
        }
    }
}

impl Add for &SailfishNumber {
    type Output = SailfishNumber;

    fn add(self, rhs: Self) -> Self::Output {
        SailfishNumber::Pair(Box::new(self.clone()), Box::new(rhs.clone())).reduced()
    }
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
        for line in all_lines("ex/day18_2.txt").unwrap() {
            let parsed: SailfishNumber = line.parse().unwrap();
            let unparsed = format!("{}", parsed);
            assert_eq!(unparsed, line);
        }
    }

    #[test]
    fn expand_test() {
        for (before, after) in [
            ("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"),
            ("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]"),
            ("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"),
            ("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"),
            ("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[7,0]]]]")
        ] {
            let start: SailfishNumber = before.parse().unwrap();
            let exploded = start.exploded(0);
            let end = format!("{}", exploded.0);
            assert_eq!(end.as_str(), after);
        }
    }

    #[test]
    fn add_test_1() {
        let one: SailfishNumber = "[[[[4,3],4],4],[7,[[8,4],9]]]".parse().unwrap();
        let two: SailfishNumber = "[1,1]".parse().unwrap();
        let sum = &one + &two;
        let sum_str = format!("{}", sum);
        assert_eq!(sum_str.as_str(), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
    }

    #[test]
    fn add_test_2() {
        let mut sum: SailfishNumber = "[1,1]".parse().unwrap();
        for i in 2..=6 {
            sum = &sum + &format!("[{},{}]", i, i).parse::<SailfishNumber>().unwrap();
            println!("{}: {}", i, sum);
        }
        let result: SailfishNumber = "[[[[5,0],[7,4]],[5,5]],[6,6]]".parse().unwrap();
        assert_eq!(sum, result);
    }

    #[test]
    fn add_test_3() {
        let one: SailfishNumber = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]".parse().unwrap();
        let two: SailfishNumber = "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]".parse().unwrap();
        let sum = &one + &two;
        let sum_str = format!("{}", sum);
        assert_eq!(sum_str.as_str(), "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]");
    }
}