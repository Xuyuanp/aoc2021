use std::collections::HashSet;
use std::convert::TryInto;

#[derive(Debug)]
struct Image {
    lights: HashSet<(i32, i32)>,
    width: usize,
    height: usize,
    default_on: bool,
}

impl Image {
    fn new(width: usize, height: usize, default_on: bool) -> Self {
        Self {
            width,
            height,
            default_on,
            lights: HashSet::new(),
        }
    }

    fn from(matrix: &Vec<Vec<char>>) -> Self {
        let (height, width) = (matrix.len(), matrix[0].len());
        let mut lights = HashSet::new();
        for i in 0..height {
            for j in 0..width {
                if matrix[i][j] == '#' {
                    lights.insert((i as i32, j as i32));
                }
            }
        }
        Self {
            width,
            height,
            lights,
            default_on: false,
        }
    }

    fn collect_bits(&self, x: i32, y: i32) -> usize {
        let (w, h) = (self.width as i32, self.height as i32);
        (-1..2)
            .map(move |dx| (-1..2).map(move |dy| (x + dx, y + dy)))
            .flatten()
            .map(|(nx, ny)| {
                if nx < 0 || nx >= h || ny < 0 || ny >= w {
                    if self.default_on {
                        1
                    } else {
                        0
                    }
                } else {
                    if self.lights.contains(&(nx, ny)) {
                        1
                    } else {
                        0
                    }
                }
            })
            .fold(0, |acc, b| acc << 1 | b)
    }

    fn enhance(&self, trans: &[char; 512]) -> Self {
        let mut new = Self::new(self.width + 2, self.height + 2, !self.default_on);

        let (w, h) = (self.width as i32, self.height as i32);
        (-1..h + 1)
            .enumerate()
            .map(move |x| (-1..w + 1).enumerate().map(move |y| (x, y)))
            .flatten()
            .for_each(|p| {
                let ((new_x, old_x), (new_y, old_y)) = p;
                let idx = self.collect_bits(old_x, old_y);
                if trans[idx] == '#' {
                    new.lights.insert((new_x as i32, new_y as i32));
                }
            });

        new
    }
}

pub fn part1(input: &Vec<String>) -> bool {
    let trans: [char; 512] = input[0].chars().collect::<Vec<_>>().try_into().unwrap();
    let matrix: Vec<Vec<char>> = input[2..]
        .iter()
        .map(|row| row.chars().collect::<Vec<_>>())
        .collect();
    let mut image = Image::from(&matrix);

    for _ in 0..2 {
        image = image.enhance(&trans);
    }

    let res = image.lights.len();
    res == 5306
}

pub fn part2(input: &Vec<String>) -> bool {
    let trans: [char; 512] = input[0].chars().collect::<Vec<_>>().try_into().unwrap();
    let matrix: Vec<Vec<char>> = input[2..]
        .iter()
        .map(|row| row.chars().collect::<Vec<_>>())
        .collect();
    let mut image = Image::from(&matrix);

    for _ in 0..50 {
        image = image.enhance(&trans);
    }

    let res = image.lights.len();
    res == 17497
}
