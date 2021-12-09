use std::convert::TryInto;

pub fn part1(input: &Vec<String>) -> bool {
    let matrix: [[char; 100]; 100] = input
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>().try_into().unwrap())
        .collect::<Vec<[char; 100]>>()
        .try_into()
        .unwrap();

    let mut res = 0;
    let base = '0';
    for i in 0..100 as i32 {
        for j in 0..100 {
            let curr = matrix[i as usize][j as usize];

            let is_low_point = [(0, 1), (0, -1), (1, 0), (-1, 0)]
                .iter()
                .map(|(dx, dy)| (i + dx, j + dy))
                .filter(|(x, y)| x >= &0 && x < &100 && y >= &0 && y < &100)
                .map(|(x, y)| matrix[x as usize][y as usize])
                .filter(|v| v <= &curr)
                .count()
                == 0;

            if is_low_point {
                res += (curr as u32 - base as u32) + 1;
                println!("matrix[{}][{}] = {}", i, j, curr);
            }
        }
    }
    res == 633
}

pub fn part2(_input: &Vec<String>) -> bool {
    false
}
