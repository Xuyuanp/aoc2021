use std::num::ParseIntError;

enum Move {
    Depth(i32),
    Horizental(i32),
}

struct Point {
    horizental: i32,
    depth: i32,
}

struct AimedPoint {
    horizental: i32,
    depth: i32,
    aim: i32,
}

impl Point {
    fn new() -> Self {
        Point {
            horizental: 0,
            depth: 0,
        }
    }
    fn execute(&mut self, mv: Move) {
        match mv {
            Move::Horizental(steps) => self.horizental += steps,
            Move::Depth(steps) => self.depth += steps,
        }
    }
}

impl AimedPoint {
    fn new() -> Self {
        Self {
            horizental: 0,
            depth: 0,
            aim: 0,
        }
    }
    fn execute(&mut self, mv: Move) {
        match mv {
            Move::Horizental(steps) => {
                self.horizental += steps;
                self.depth += self.aim * steps;
            }
            Move::Depth(steps) => self.aim += steps,
        }
    }
}

// possible to return an iterator?
fn parse_input(input: &Vec<String>) -> Result<Vec<Move>, ParseIntError> {
    input
        .iter()
        .map(|s| {
            let fields: Vec<&str> = s.splitn(2, " ").collect();
            let steps = fields[1].parse::<i32>()?;

            Ok(match fields[0] {
                "forward" => Move::Horizental(steps),
                "up" => Move::Depth(-steps),
                "down" => Move::Depth(steps),
                _ => panic!("unknown move"),
            })
        })
        .collect()
}

pub fn part1(input: &Vec<String>) -> bool {
    let mut pos = Point::new();
    for mv in parse_input(input).unwrap() {
        pos.execute(mv);
    }
    pos.horizental * pos.depth == 1746616
}

pub fn part2(input: &Vec<String>) -> bool {
    let mut pos = AimedPoint::new();
    for mv in parse_input(input).unwrap() {
        pos.execute(mv);
    }
    pos.horizental * pos.depth == 1741971043
}
