use std::fmt::Display;

mod macros;

pub enum Part {
    One,
    Two,
}

impl Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &Self::One => f.write_str("part1"),
            &Self::Two => f.write_str("part2"),
        }
    }
}

pub struct Solution {
    pub part1: fn(nput: &Vec<String>) -> bool,
    pub part2: fn(nput: &Vec<String>) -> bool,
}

impl Solution {
    pub fn run(&self, part: Part, input: &Vec<String>) -> bool {
        match part {
            Part::One => (self.part1)(input),
            Part::Two => (self.part2)(input),
        }
    }
}

crate::aoc!(day01, day02, day03, day04, day05, day06);
