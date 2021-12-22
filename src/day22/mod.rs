use std::cmp::{max, min};
use std::{collections::HashSet, num::ParseIntError, str::FromStr};

#[derive(Debug)]
struct Step {
    on: bool,
    region: Region,
}

type Range = std::ops::RangeInclusive<isize>;

#[derive(Debug)]
struct Region {
    x: Range,
    y: Range,
    z: Range,
}

#[derive(Debug)]
struct Point(isize, isize, isize);

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

pub fn part2(_input: &Vec<String>) -> bool {
    unimplemented!()
}
