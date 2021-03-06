use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
struct Step {
    on: bool,
    region: Region,
}

type Range = std::ops::RangeInclusive<isize>;

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
struct Region {
    x: Range,
    y: Range,
    z: Range,
}

impl FromStr for Region {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ranges: Vec<_> = s.split(",").collect();
        let mut xyz = ranges.iter().map(|r| {
            let s = r.split("=").last().unwrap();
            let seps: Vec<_> = s.split("..").collect();

            let start = seps[0].parse()?;
            let end = seps[1].parse()?;

            Ok(start..=end)
        });
        let x = xyz.next().unwrap()?;
        let y = xyz.next().unwrap()?;
        let z = xyz.next().unwrap()?;

        Ok(Self { x, y, z })
    }
}

impl Region {
    fn count(&self) -> usize {
        let x = (self.x.end() - self.x.start() + 1) as usize;
        let y = (self.y.end() - self.y.start() + 1) as usize;
        let z = (self.z.end() - self.z.start() + 1) as usize;
        x * y * z
    }
}

impl FromStr for Step {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let seps = s.split(" ").collect::<Vec<_>>();
        let on = seps[0] == "on";
        let region = seps[1].parse()?;

        Ok(Self { on, region })
    }
}

fn parse_input(input: &Vec<String>) -> Result<Vec<Step>, ParseIntError> {
    input.iter().map(|line| line.parse()).collect()
}

pub fn part1(input: &Vec<String>) -> bool {
    let steps = parse_input(input).unwrap();

    let mut reactor = HashSet::new();

    steps.iter().for_each(|s| {
        for x in max(-50, *s.region.x.start())..=min(50, *s.region.x.end()) {
            for y in max(-50, *s.region.y.start())..=min(50, *s.region.y.end()) {
                for z in max(-50, *s.region.z.start())..=min(50, *s.region.z.end()) {
                    if s.on {
                        reactor.insert((x, y, z));
                    } else {
                        reactor.remove(&(x, y, z));
                    }
                }
            }
        }
    });
    let res = reactor.len();

    res == 580810
}

fn intersaction_range(r1: &Range, r2: &Range) -> Option<Range> {
    let start = max(r1.start(), r2.start());
    let end = min(r1.end(), r2.end());
    if start > end {
        None
    } else {
        Some(*start..=*end)
    }
}

fn intersaction(r1: &Region, r2: &Region) -> Option<Region> {
    let x = intersaction_range(&r1.x, &r2.x)?;
    let y = intersaction_range(&r1.y, &r2.y)?;
    let z = intersaction_range(&r1.z, &r2.z)?;
    Some(Region { x, y, z })
}

pub fn part2(input: &Vec<String>) -> bool {
    let steps = parse_input(input).unwrap();

    let mut counter = HashMap::new();

    for step in steps {
        let mut new_counter = counter.clone();

        for (region, cnt) in counter {
            if let Some(sub) = intersaction(&region, &step.region) {
                *new_counter.entry(sub).or_insert(0) -= cnt;
            }
        }

        if step.on {
            *new_counter.entry(step.region).or_insert(0) += 1;
        }

        counter = new_counter;
    }

    let res: i64 = counter.iter().map(|(r, c)| r.count() as i64 * c).sum();

    res == 1265621119006734
}
