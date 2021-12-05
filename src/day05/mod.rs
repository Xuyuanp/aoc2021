use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point(i32, i32);

impl Point {
    fn new(s: &str) -> Self {
        let seps: Vec<&str> = s.split(",").collect();
        let x = seps[0].parse().unwrap();
        let y = seps[1].parse().unwrap();

        Self(x, y)
    }

    fn cover(&self, other: &Point) -> Vec<Self> {
        if self.0 == other.0 {
            if self.1 > other.1 {
                return other.cover(self);
            }
            (self.1..other.1 + 1).map(|y| Point(self.0, y)).collect()
        } else if self.1 == other.1 {
            if self.0 > other.0 {
                return other.cover(self);
            }
            (self.0..other.0 + 1).map(|x| Point(x, self.1)).collect()
        } else {
            if self.0 > other.0 {
                return other.cover(self);
            }
            let dx = if self.0 < other.0 { 1 } else { -1 };
            let dy = if self.1 < other.1 { 1 } else { -1 };
            let dist = (self.0 - other.0).abs();
            (0..dist + 1)
                .map(|i| Point(self.0 + i * dx, self.1 + i * dy))
                .collect()
        }
    }
}

pub fn part1(input: &Vec<String>) -> bool {
    let all_points = input
        .iter()
        .map(|s| {
            let seps: Vec<&str> = s.split(" -> ").collect();
            let start = Point::new(seps[0]);
            let end = Point::new(seps[1]);
            (start, end)
        })
        .filter(|(start, end)| start.0 == end.0 || start.1 == end.1)
        .map(|(start, end)| start.cover(&end))
        .flatten()
        .fold(HashMap::new(), |mut counter, p| {
            let x = counter.remove(&p).unwrap_or(0);
            counter.insert(p, x + 1);
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
        .map(|s| {
            let seps: Vec<&str> = s.split(" -> ").collect();
            let start = Point::new(seps[0]);
            let end = Point::new(seps[1]);
            (start, end)
        })
        .filter(|(start, end)| {
            start.0 == end.0
                || start.1 == end.1
                || (start.0 - end.0).abs() == (start.1 - end.1).abs()
        })
        .map(|(start, end)| start.cover(&end))
        .flatten()
        .fold(HashMap::new(), |mut counter, p| {
            let x = counter.remove(&p).unwrap_or(0);
            counter.insert(p, x + 1);
            counter
        })
        .iter()
        .filter(|ent| ent.1 >= &2)
        .count();
    all_points == 15463
}
