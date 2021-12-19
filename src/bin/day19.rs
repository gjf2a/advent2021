use std::fmt::{Display, Formatter};
use std::io;
use std::num::ParseIntError;
use std::str::{FromStr, Split};
use advent_code_lib::{advent_main, ExNihilo, make_inner_io_error, MultiLineObjects};

fn main() -> io::Result<()> {
    advent_main(&[], &[], |args| {
        let scanners = Scanners::from_file(args[1].as_str())?;
        println!("{}", scanners);
        Ok(())
    })
}

#[derive(Clone, Debug)]
struct Scanners {
    scanners: Vec<Scanner>
}

impl Scanners {
    fn from_file(filename: &str) -> io::Result<Self> {
        Ok(Scanners {scanners: MultiLineObjects::from_file(filename, |scanner: &mut Scanner, line| {
            if !line.contains("scanner") {
                scanner.beacons.push(line.parse().unwrap());
            }
        })?.objects()})
    }
}

impl Display for Scanners {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (i, scanner) in self.scanners.iter().enumerate() {
            writeln!(f, "--- scanner {} ---", i)?;
            for beacon in scanner.beacons.iter() {
                writeln!(f, "{}", beacon)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Scanner {
    beacons: Vec<Point3>
}

impl ExNihilo for Scanner {
    fn create() -> Self {Scanner {beacons: Vec::new()}}
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Point3 {
    x: isize, y: isize, z: isize
}

impl FromStr for Point3 {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nums = s.split(',');
        let x = next_value_from(&mut nums, "x")?;
        let y = next_value_from(&mut nums, "y")?;
        let z = next_value_from(&mut nums, "z")?;
        Ok(Point3 {x, y, z})
    }
}

fn next_value_from(nums: &mut Split<char>, id: &str) -> io::Result<isize> {
    let error_msg_1 = make_inner_io_error(format!("No {} value", id).as_str());
    nums.next()
        .ok_or(error_msg_1)?
        .parse()
        .map_err(|e: ParseIntError| make_inner_io_error(format!("Error parsing {} value: {}", id, e.to_string()).as_str()))
}

impl Display for Point3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use crate::Point3;

    #[test]
    fn test_point_io() {
        for point_str in ["-892,524,684", "-876,649,763"] {
            let point: Point3 = point_str.parse().unwrap();
            assert_eq!(point_str, format!("{}", point).as_str());
        }
    }
}