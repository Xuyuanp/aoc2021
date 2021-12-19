use std::collections::{HashMap, HashSet};
use std::num::ParseIntError;
use std::ops::Sub;
use std::str::FromStr;

#[derive(Debug)]
enum ParseError {
    WrongDim(usize),
    ParseIntError(ParseIntError),
}

impl From<ParseIntError> for ParseError {
    fn from(e: ParseIntError) -> Self {
        Self::ParseIntError(e)
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point(i32, i32, i32);

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Scanner {
    id: u32,
    position: Option<Point>,
    beacons: Vec<Point>,
}

impl FromStr for Point {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let seps = s.split(",").collect::<Vec<_>>();
        if seps.len() != 3 {
            return Err(ParseError::WrongDim(seps.len()));
        }

        let (x, y, z) = (seps[0].parse()?, seps[1].parse()?, seps[2].parse()?);

        Ok(Self(x, y, z))
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Point {
    fn manhattan_dist(&self, other: &Self) -> u32 {

        (self.0-other.0).abs() as u32 + (self.1-other.1).abs() as u32 + (self.2-other.2).abs() as u32
        // self.0.abs_diff(other.0) + self.1.abs_diff(other.1) + self.2.abs_diff(other.2)
    }
}

fn parse_input(input: &Vec<String>) -> Result<Vec<Scanner>, ParseError> {
    let mut scanners = Vec::new();

    let mut lines = input.into_iter();

    let mut idx = 0;
    while let Some(_) = lines.next() {
        let mut beacons = Vec::new();
        while let Some(line) = lines.next() {
            match line.as_str() {
                "" => break,
                _ => beacons.push(line.parse()?),
            }
        }
        scanners.push(Scanner {
            id: idx,
            position: match idx {
                0 => Some(Point(0, 0, 0)),
                _ => None,
            },
            beacons,
        });
        idx += 1;
    }

    Ok(scanners)
}

fn guess_one(
    s: &Scanner,
    known: &Scanner,
    delta: (i32, i32, i32),
    dir: (usize, usize, usize),
) -> Option<Scanner> {
    let (dx, dy, dz) = delta;
    let (i, j, k) = dir;
    let mut counter = HashMap::new();
    for b1 in known.beacons.iter() {
        for b2 in s.beacons.iter() {
            let axises = [b2.0, b2.1, b2.2];
            let (x, y, z) = (axises[i], axises[j], axises[k]);
            let b2 = Point(dx * x, dy * y, dz * z);
            let delta = *b1 - b2;
            *counter.entry(delta).or_insert(0) += 1;
        }
    }
    if let Some(ent) = counter.iter().filter(|ent| ent.1 >= &12).next() {
        let Point(x, y, z) = ent.0;
        return Some(Scanner {
            id: s.id,
            position: Some(*ent.0),
            beacons: s
                .beacons
                .iter()
                .map(|p| {
                    let axises = [p.0, p.1, p.2];
                    Point(dx * axises[i] + x, dy * axises[j] + y, dz * axises[k] + z)
                })
                .collect(),
        });
    }
    None
}

fn guess(s: &Scanner, knowns: &Vec<Scanner>) -> Option<Scanner> {
    for s1 in knowns.iter() {
        for dx in [-1, 1] {
            for dy in [-1, 1] {
                for dz in [-1, 1] {
                    for (i, j, k) in [
                        (0, 1, 2),
                        (0, 2, 1),
                        (1, 0, 2),
                        (1, 2, 0),
                        (2, 0, 1),
                        (2, 1, 0),
                    ] {
                        if let Some(s) = guess_one(s, s1, (dx, dy, dz), (i, j, k)) {
                            return Some(s);
                        }
                    }
                }
            }
        }
    }
    None
}

fn resolve(scanners: &Vec<Scanner>) -> Vec<Scanner> {
    let mut knowns = vec![scanners[0].clone()];

    let mut unknowns: Vec<Scanner> = scanners[1..].iter().map(|s| s.clone()).collect();

    while unknowns.len() > 0 {
        let mut new_unknowns = Vec::new();

        let length = unknowns.len();

        for s in unknowns {
            if let Some(new) = guess(&s, &knowns) {
                knowns.push(new);
            } else {
                new_unknowns.push(s);
            }
        }
        assert_ne!(new_unknowns.len(), length);

        unknowns = new_unknowns;
    }
    knowns
}

pub fn part1(input: &Vec<String>) -> bool {
    let scanners = parse_input(input).unwrap();

    let knowns = resolve(&scanners);

    let res = knowns
        .iter()
        .fold(HashSet::new(), |mut set, s| {
            for b in s.beacons.iter() {
                set.insert(b);
            }
            set
        })
        .len();

    res == 472
}

pub fn part2(input: &Vec<String>) -> bool {
    let scanners = parse_input(input).unwrap();
    let knowns = resolve(&scanners);

    let mut res = u32::MIN;
    for i in 0..knowns.len() {
        for j in 0..knowns.len() {
            if i == j {
                continue;
            }
            let pos1 = knowns[i].position.unwrap();
            let pos2 = knowns[j].position.unwrap();
            let dist = pos1.manhattan_dist(&pos2);
            res = res.max(dist);
        }
    }

    res == 12092
}
