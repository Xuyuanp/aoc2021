use std::num::ParseIntError;

enum Move {
    Vertical(i32),
    Horizental(i32),
}

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn execute(&mut self, mv: Move) {
        match mv {
            Move::Horizental(steps) => self.x += steps,
            Move::Vertical(steps) => self.y += steps,
        }
    }
}

fn parse_input(input: &Vec<String>) -> Result<Vec<Move>, ParseIntError> {
    input
        .iter()
        .map(|s| {
            let fields: Vec<&str> = s.splitn(2, " ").collect();
            let steps = fields[1].parse::<i32>()?;

            Ok(match fields[0] {
                "forward" => Move::Horizental(steps),
                "backward" => Move::Horizental(-steps),
                "up" => Move::Vertical(-steps),
                "down" => Move::Vertical(steps),
                _ => panic!("unknown move"),
            })
        })
        .collect()
}

pub fn part1(input: &Vec<String>) -> bool {
    let mut pos = Point { x: 0, y: 0 };
    for mv in parse_input(input).unwrap() {
        pos.execute(mv);
    }
    pos.x * pos.y == 1746616
}

pub fn part2(_input: &Vec<String>) -> bool {
    false
}
