use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::io;
use std::num::ParseIntError;
use std::ops::{Add, Mul, Neg, Sub};
use std::str::{FromStr, Split};
use advent_code_lib::{advent_main, ExNihilo, make_inner_io_error, MultiLineObjects};
use bare_metal_modulo::{MNum, ModNumC};

const MIN_OVERLAPPING_POINTS: usize = 12;

fn main() -> io::Result<()> {
    advent_main(&[], &[], |args| {
        let scanners = Scanners::from_file(args[1].as_str())?;
        println!("Part 1: {}", scanners.find_all_beacons().len());
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
                scanner.add_beacon(line.parse().unwrap());
            }
        })?.objects()})
    }

    fn find_all_beacons(&self) -> Vec<Point3> {
        let mut result = Vec::new();
        for i in 0..self.scanners.len() {
            for j in (i+1)..self.scanners.len() {
                println!("Checking {} vs {}", i, j);
                println!("{:?}", self.scanners[i].overlap_with(&self.scanners[j]))
            }
        }
        result
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
    beacons: Vec<Point3>,
    offsets2triples: HashMap<Point3,Vec<(Transform, Point3, Point3)>>,
    beacons2beacons: HashMap<Point3,HashMap<Point3,HashMap<Transform, Point3>>>
}

impl Scanner {
    fn add_beacon(&mut self, beacon: Point3) {
        self.beacons2beacons.insert(beacon, HashMap::new());
        let mut beacons = self.beacons.iter().copied().collect::<Vec<_>>();
        for other in beacons {
            self.add_offset_between(other, beacon);
            self.add_offset_between(beacon, other);
        }
        self.beacons.push(beacon);
    }

    fn add_offset_between(&mut self, beacon1: Point3, beacon2: Point3) {
        let mut combos = HashMap::new();
        let offset = beacon2 - beacon1;
        for (version, transform) in offset.transforms() {
            let triple = (transform, beacon1, beacon2);
            combos.insert(transform, version);
            match self.offsets2triples.get_mut(&version) {
                None => {self.offsets2triples.insert(version, vec![triple]);}
                Some(triples) => {triples.push(triple);}
            }
        }
        self.beacons2beacons.get_mut(&beacon1).unwrap().insert(beacon2, combos);
    }

    fn overlap_with(&self, other: &Scanner) -> Option<Vec<Point3>> {
        let common_offsets = self.offsets2triples.keys()
            .filter(|offset| other.offsets2triples.contains_key(*offset))
            .copied()
            .collect::<Vec<_>>();
        println!("# common offsets: {}", common_offsets.len());

        for offset in common_offsets.iter() {
            for (transform, beacon1, beacon2) in self.offsets2triples.get(offset).unwrap() {
                for (other_transform, other1, other2) in other.offsets2triples.get(offset).unwrap() {

                }
            }
        }
        None
    }

    fn overlapping_points(&self, beacon1: Point3, beacon2: Point3, transform: Transform, other: &Scanner, other1: Point3, other2: Point3, other_transform: Transform) -> Option<Vec<Point3>> {
        let mut result = vec![beacon1, beacon2];
        let mut current_start = beacon2;
        let mut current_other = other2;
        for _ in result.len()..MIN_OVERLAPPING_POINTS {
            for (candidate, possibilities) in self.beacons2beacons.get(&current_start).unwrap().iter() {
                let candidate_offset = possibilities.get(&transform).unwrap();
                if let Some(next_other) = other.offsets2triples.get(candidate_offset)
                    .and_then(|possibilities| possibilities.iter()
                        .find(|(otransform, obeacon1, obeacon2)| otransform == other_transform && obeacon1 == current_other))
                    .map(|(_,_,next_other)| *next_other) {

                }
            }
        }
        Some(result)
    }
}

impl ExNihilo for Scanner {
    fn create() -> Self {Scanner {beacons: Vec::new(), offsets2triples: HashMap::new(), beacons2beacons: HashMap::new()}}
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Point3 {
    x: isize, y: isize, z: isize
}

impl Point3 {
    fn abs(&self) -> Self {
        Point3 {x: self.x.abs(), y: self.y.abs(), z: self.z.abs()}
    }

    fn signum(&self) -> Self {
        Point3 {x: self.x.signum(), y: self.y.signum(), z: self.z.signum()}
    }

    fn rotated90(&self) -> Self {
        Point3 {x: self.x, y: -self.z, z: self.y}
    }

    fn flipped(&self) -> Self {
        Point3 {x: -self.x, y: -self.y, z: self.z}
    }

    fn advanced(&self) -> Self {
        Point3 {x: self.y, y: self.z, z: self.x}
    }

    fn transforms(&self) -> TransformIterator {
        TransformIterator {transform: Transform::new(), point: *self, done: false}
    }
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

impl Add for Point3 {
    type Output = Point3;
    fn add(self, rhs: Self) -> Self::Output {
        Point3 {x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z}
    }
}

impl Neg for Point3 {
    type Output = Point3;
    fn neg(self) -> Self::Output {Point3 {x: -self.x, y: -self.y, z: -self.z}}
}

impl Sub for Point3 {
    type Output = Point3;
    fn sub(self, rhs: Self) -> Self::Output {self + (-rhs)}
}

// Hadamard Product
impl Mul for Point3 {
    type Output = Point3;

    fn mul(self, rhs: Self) -> Self::Output {
        Point3 {x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z}
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
struct Transform {
    axes: ModNumC<u8, 3>,
    flips: ModNumC<u8, 2>,
    rotations90: ModNumC<u8, 4>
}

impl Add for Transform {
    type Output = Transform;

    fn add(self, rhs: Self) -> Self::Output {
        Transform {axes: self.axes + rhs.axes, flips: self.flips + rhs.flips, rotations90: self.rotations90 + rhs.rotations90}
    }
}

impl Neg for Transform {
    type Output = Transform;

    fn neg(self) -> Self::Output {
        Transform {axes: -self.axes, flips: -self.flips, rotations90: -self.rotations90}
    }
}

impl Transform {
    fn new() -> Self {
        Transform {axes: ModNumC::new(0), flips: ModNumC::new(0), rotations90: ModNumC::new(0)}
    }

    fn transformed(&self, p: Point3) -> Point3 {
        let mut result = p;
        for _ in 0..self.axes.a() {result = result.advanced();}
        for _ in 0..self.flips.a() {result = result.flipped();}
        for _ in 0..self.rotations90.a() {result = result.rotated90();}
        result
    }
}

struct TransformIterator {
    transform: Transform,
    point: Point3,
    done: bool
}

impl Iterator for TransformIterator {
    type Item = (Point3, Transform);

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        } else {
            let result = (self.point, self.transform);
            self.point = self.point.rotated90();
            self.transform.rotations90 += 1;
            if self.transform.rotations90 == 0 {
                self.point = self.point.flipped();
                self.transform.flips += 1;
                if self.transform.flips == 0 {
                    self.point = self.point.advanced();
                    self.transform.axes += 1;
                    self.done = self.transform.axes == 0;
                }
            }
            Some(result)
        }
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
    use super::*;

    #[test]
    fn test_point_io() {
        for point_str in ["-892,524,684", "-876,649,763"] {
            let point: Point3 = point_str.parse().unwrap();
            assert_eq!(point_str, format!("{}", point).as_str());
        }
    }

    #[test]
    fn test_transform_iterator() {
        let start = Point3 {x: 1, y: 2, z: 3};
        for ((point, transform), expected) in start.transforms()
            .zip([
                ( 1,  2,  3), ( 1, -3,  2), ( 1, -2, -3), ( 1,  3, -2),
                (-1, -2,  3), (-1, -3, -2), (-1,  2, -3), (-1,  3,  2),
                ( 2,  3,  1), ( 2, -1,  3), ( 2, -3, -1), ( 2,  1, -3),
                (-2, -3,  1), (-2, -1, -3), (-2,  3, -1), (-2,  1,  3),
                ( 3,  1,  2), ( 3, -2,  1), ( 3, -1, -2), ( 3,  2, -1),
                (-3, -1,  2), (-3, -2, -1), (-3,  1, -2), (-3,  2,  1)
            ].iter()) {
            let (x, y, z) = *expected;
            let expected = Point3 {x, y, z};
            assert_eq!(point, expected);
            assert_eq!(point, transform.transformed(start));
            /*
            // This doesn't work.
            // Maybe I won't need it.

            let inverse = -transform;
            assert_eq!(transform + inverse, Transform::new());
            println!("point:     {}", point);
            println!("transform: {:?}", transform);
            println!("inverse:   {:?}", inverse);
            assert_eq!(inverse.transformed(point), start);

             */
        }
    }

    #[test]
    fn experiments() {
        let s1a = Point3 {x: 0, y: 0, z: 0};
        let s1b = Point3 {x: 3, y: 2, z: 1};

        let s2a = Point3 {x: 1, y: 1, z: 1};
        let s2b = Point3 {x: -2, y: 3, z: 2};

        let offset1 = s1b - s1a;
        let offset2 = s2b - s2a;
        let dir1 = offset1.signum();
        let dir2 = offset2.signum();
        for (offset, sign, test_offset, test_sign) in [("3,2,1", "1,1,1", offset1, dir1), ("-3,2,1", "-1,1,1", offset2, dir2)] {
            let offset: Point3 = offset.parse().unwrap();
            let sign: Point3 = sign.parse().unwrap();
            assert_eq!(offset, test_offset);
            assert_eq!(sign, test_sign);
        }
    }
}

// Finding the offsets
//
// Perform a depth-first search as follows:
//
// Set up a stack of points from self, initialized by an arbitrary point
// If the stack contains 12 points, end the search
// Successors:
// * For every offset that is present in other:
//   * For every point from the offset:
//