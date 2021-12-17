#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy)]
struct Area(i32, i32, i32, i32);

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn default() -> Self {
        Self { x: 0, y: 0 }
    }

    fn in_area(&self, target: &Area) -> bool {
        self.x >= target.0 && self.x <= target.1 && self.y >= target.3 && self.y <= target.2
    }
}

#[derive(Debug)]
struct Velocity {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Probe {
    position: Point,
    velocity: Velocity,
    target: Area,
}

impl Iterator for Probe {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position.in_area(&self.target) {
            return None;
        }
        let new_pos = Point::new(
            self.position.x + self.velocity.x,
            self.position.y + self.velocity.y,
        );

        if new_pos.y < self.target.3 {
            return None;
        }

        if (self.velocity.x == 0 && new_pos.x < self.target.0) || new_pos.x > self.target.1 {
            return None;
        }

        self.position = new_pos;

        self.velocity.x = 0.max(self.velocity.x - 1);
        self.velocity.y -= 1;

        Some(self.position)
    }
}

pub fn part1(_input: &Vec<String>) -> bool {
    let area = Area(144, 178, -76, -100); // skip pasing the stupid input
    let mut res = 0;
    for vx in 1..area.1 + 1 {
        for vy in area.3..-area.3 + 1 {
            let probe = Probe {
                position: Point::default(),
                velocity: Velocity { x: vx, y: vy },
                target: area,
            };

            let mut highest = 0;
            if let Some(last) = probe
                .map(|p| {
                    highest = highest.max(p.y);
                    p
                })
                .last()
            {
                if last.in_area(&area) {
                    res = res.max(highest);
                }
            }
        }
    }

    res == 4950
}

pub fn part2(_input: &Vec<String>) -> bool {
    let area = Area(144, 178, -76, -100); // skip pasing the stupid input
    let mut res = 0;
    for vx in 1..area.1 + 1 {
        for vy in area.3..-area.3 + 1 {
            let probe = Probe {
                position: Point::default(),
                velocity: Velocity { x: vx, y: vy },
                target: area,
            };

            if let Some(last) = probe.last() {
                if last.in_area(&area) {
                    res += 1
                }
            }
        }
    }

    res == 1477
}
