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

pub fn part2(_input: &Vec<String>) -> bool {
    false
}
