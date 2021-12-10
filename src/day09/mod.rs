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
                .map(|(dx, dy)| (*i + dx, *j + dy))
                .filter(|(x, y)| {
                    matrix
                        .get(*x as usize)
                        .and_then(|row| row.get(*y as usize))
                        .map_or(false, |v| v <= &curr)
                })
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
                .map(|(dx, dy)| (*i + dx, *j + dy))
                .filter(|(x, y)| {
                    matrix
                        .get(*x as usize)
                        .and_then(|row| row.get(*y as usize))
                        .map_or(false, |v| v <= &curr)
                })
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
                    .map(|(dx, dy)| (i + dx, j + dy))
                    .filter(|(x, y)| {
                        matrix
                            .get(*x as usize)
                            .and_then(|row| row.get(*y as usize))
                            .map_or(false, |v| v < &'9' && v > &curr && visited.insert((*x, *y)))
                    })
                    .for_each(|(x, y)| {
                        queue.push_back((x, y));
                    });
            }

            size
        })
        .collect::<Vec<i32>>();

    basins.select_nth_unstable_by_key(3, |v| -v);
    let res = basins.iter().take(3).fold(1, |acc, v| acc * v);

    res == 1050192
}
