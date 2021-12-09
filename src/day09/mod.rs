use std::collections::{HashSet, VecDeque};
use std::convert::TryInto;

pub fn part1(input: &Vec<String>) -> bool {
    let matrix: [[char; 100]; 100] = input
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>().try_into().unwrap())
        .collect::<Vec<[char; 100]>>()
        .try_into()
        .unwrap();

    let base = '0';
    let res = (0..100)
        .map(move |i| (0..100).map(move |j| (i, j)))
        .flatten()
        .filter(|(i, j)| {
            let curr = matrix[*i as usize][*j as usize];

            [(0, 1), (0, -1), (1, 0), (-1, 0)]
                .iter()
                .map(|(dx, dy)| (i + dx, j + dy))
                .filter(|(x, y)| x >= &0 && x < &100 && y >= &0 && y < &100)
                .map(|(x, y)| matrix[x as usize][y as usize])
                .filter(|v| v <= &curr)
                .count()
                == 0
        })
        .map(|(i, j)| {
            let curr = matrix[i as usize][j as usize];
            (curr as u32 - base as u32) + 1
        })
        .sum::<u32>();

    res == 633
}

pub fn part2(input: &Vec<String>) -> bool {
    let matrix: [[char; 100]; 100] = input
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>().try_into().unwrap())
        .collect::<Vec<[char; 100]>>()
        .try_into()
        .unwrap();

    let mut basins = (0..100)
        .map(move |i| (0..100).map(move |j| (i, j)))
        .flatten()
        .filter(|(i, j)| {
            let curr = matrix[*i as usize][*j as usize];

            [(0, 1), (0, -1), (1, 0), (-1, 0)]
                .iter()
                .map(|(dx, dy)| (*i as i32 + dx, *j as i32 + dy))
                .filter(|(x, y)| x >= &0 && x < &100 && y >= &0 && y < &100)
                .map(|(x, y)| matrix[x as usize][y as usize])
                .filter(|v| v <= &curr)
                .count()
                == 0
        })
        .map(|(i, j)| {
            let mut visited = HashSet::new();
            let mut queue = VecDeque::new();

            queue.push_back((i as i32, j as i32));

            let mut size = 0;

            while let Some((i, j)) = queue.pop_front() {
                size += 1;
                visited.insert((i, j));
                let curr = matrix[i as usize][j as usize];

                [(0, 1), (0, -1), (1, 0), (-1, 0)]
                    .iter()
                    .map(|(dx, dy)| (i as i32 + dx, j as i32 + dy))
                    .filter(|(x, y)| {
                        x >= &0
                            && x < &100
                            && y >= &0
                            && y < &100
                            && matrix[*x as usize][*y as usize] > curr
                            && matrix[*x as usize][*y as usize] < '9'
                            && visited.insert((*x, *y))
                    })
                    .for_each(|(x, y)| {
                        queue.push_back((x, y));
                    });
            }

            size
        })
        .collect::<Vec<i32>>();

    basins.sort_by_key(|v| -v);

    let res = basins.iter().take(3).fold(1, |acc, v| acc * v);

    res == 1050192
}
