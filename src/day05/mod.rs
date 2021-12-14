use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point(i32, i32);

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let seps: Vec<&str> = s.split(",").collect();
        let x = seps[0].parse()?;
        let y = seps[1].parse()?;

        Ok(Self(x, y))
    }
}

impl Point {
    fn cover(&self, other: &Point) -> Vec<Self> {
        let delta = |a, b| -> i32 {
            if a == b {
                0
            } else {
                let d: i32 = a - b;
                -(d.abs() / d)
            }
        };
        let dx = delta(self.0, other.0);
        let dy = delta(self.1, other.1);

        let dist = (self.0 - other.0).abs().max((self.1 - other.1).abs());
        (0..dist + 1)
            .map(|i| Point(self.0 + i * dx, self.1 + i * dy))
            .collect()
    }
}

struct Line {
    start: Point,
    end: Point,
}

impl FromStr for Line {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let seps: Vec<&str> = s.split(" -> ").collect();
        let start = seps[0].parse()?;
        let end = seps[1].parse()?;
        Ok(Self { start, end })
    }
}

impl Line {
    fn is_vertical(&self) -> bool {
        return self.start.1 == self.end.1;
    }

    fn is_horizental(&self) -> bool {
        return self.start.0 == self.end.0;
    }

    fn is_diagonal(&self) -> bool {
        return (self.start.0 - self.end.0).abs() == (self.start.1 - self.end.1).abs();
    }

    fn cover(&self) -> Vec<Point> {
        self.start.cover(&self.end)
    }
}

pub fn part1(input: &Vec<String>) -> bool {
    let all_points = input
        .iter()
        .map(|s| s.parse::<Line>().unwrap())
        .filter(|line| line.is_vertical() || line.is_horizental())
        .map(|line| line.cover())
        .flatten()
        .fold(HashMap::new(), |mut counter, p| {
            *counter.entry(p).or_insert(0) += 1;
            counter
        })
        .iter()
        .filter(|ent| ent.1 >= &2)
        .count();
    all_points == 5698
}

pub fn part2(input: &Vec<String>) -> bool {
    let all_points = input
        .iter()
        .map(|s| s.parse::<Line>().unwrap())
        .filter(|line| line.is_horizental() || line.is_vertical() || line.is_diagonal())
        .map(|line| line.cover())
        .flatten()
        .fold(HashMap::new(), |mut counter, p| {
            *counter.entry(p).or_insert(0) += 1;
            counter
        })
        .iter()
        .filter(|ent| ent.1 >= &2)
        .count();
    all_points == 15463
}
