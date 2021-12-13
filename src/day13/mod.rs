use std::collections::{BTreeMap, HashSet};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
struct Point(u32, u32);

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
    fn foldx(&self, x: u32) -> Self {
        if self.0 < x {
            Self(self.0, self.1)
        } else {
            Self(x - (self.0 - x), self.1)
        }
    }
    fn foldy(&self, y: u32) -> Self {
        if self.1 < y {
            Self(self.0, self.1)
        } else {
            Self(self.0, y - (self.1 - y))
        }
    }

    fn fold(&self, fold: &Fold) -> Self {
        match *fold {
            Fold::X(x) => self.foldx(x),
            Fold::Y(y) => self.foldy(y),
        }
    }
}

#[derive(Debug)]
enum Fold {
    X(u32),
    Y(u32),
}

impl FromStr for Fold {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let seps: Vec<&str> = s.split("=").collect();
        assert_eq!(2, seps.len());

        match seps[0] {
            "x" => Ok(Fold::X(seps[1].parse()?)),
            "y" => Ok(Fold::Y(seps[1].parse()?)),
            _ => panic!("unreachable!"),
        }
    }
}

impl Fold {
    fn fold(&self, points: &Vec<Point>) -> Vec<Point> {
        let set: HashSet<Point> = points.iter().map(|p| p.fold(self)).collect();
        set.into_iter().collect()
    }
}

fn parse_input(input: &Vec<String>) -> (Vec<Point>, Vec<Fold>) {
    let mut points = Vec::new();
    let mut folds = Vec::new();
    let mut is_points = true;
    for line in input {
        if line.is_empty() {
            is_points = false;
        } else if is_points {
            points.push(line.parse().unwrap());
        } else {
            folds.push(
                line.split_whitespace()
                    .nth_back(0)
                    .unwrap()
                    .parse()
                    .unwrap(),
            );
        }
    }

    (points, folds)
}

pub fn part1(input: &Vec<String>) -> bool {
    let (points, folds) = parse_input(input);
    let first = &folds[0];
    let res = first.fold(&points).len();
    res == 708
}

pub fn part2(input: &Vec<String>) -> bool {
    let (points, folds) = parse_input(input);
    let res = folds.iter().fold(points, |pts, fold| fold.fold(&pts));

    let mut screen: BTreeMap<u32, Vec<u32>> = BTreeMap::new();
    for point in res.iter() {
        screen.entry(point.1).or_insert(vec![]).push(point.0);
    }

    let rows = vec![
        "####.###..#....#..#.###..###..####.#..#",
        "#....#..#.#....#..#.#..#.#..#.#....#..#",
        "###..###..#....#..#.###..#..#.###..####",
        "#....#..#.#....#..#.#..#.###..#....#..#",
        "#....#..#.#....#..#.#..#.#.#..#....#..#",
        "####.###..####..##..###..#..#.#....#..#",
    ];

    for (y, xs) in screen.iter_mut() {
        xs.sort();
        let mut row = ['.'; 39];
        for x in xs {
            row[*x as usize] = '#';
        }
        assert_eq!(row.iter().collect::<String>(), rows[*y as usize]);
    }

    true
}
