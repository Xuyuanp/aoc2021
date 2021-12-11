use std::collections::VecDeque;
use std::convert::TryInto;

fn parse_input(input: &Vec<String>) -> [[u32; 10]; 10] {
    let zero = '0' as u32;
    input
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| c as u32 - zero)
                .collect::<Vec<u32>>()
                .try_into()
                .unwrap()
        })
        .collect::<Vec<[u32; 10]>>()
        .try_into()
        .unwrap()
}

pub fn part1(input: &Vec<String>) -> bool {
    let mut matrix = parse_input(input);

    let flashes = (0..100)
        .map(|_step| {
            let mut queue: VecDeque<(usize, usize)> = (0..10 as usize)
                .map(move |i| (0..10 as usize).map(move |j| (i, j)))
                .flatten()
                .filter(|p| {
                    let (i, j) = (p.0, p.1);
                    matrix[i][j] = (matrix[i][j] + 1) % 10;
                    matrix[i][j] == 0
                })
                .collect();

            let mut flashes = 0;
            while let Some((i, j)) = queue.pop_front() {
                flashes += 1;

                (-1..2)
                    .map(move |dx| (-1..2).map(move |dy| (dx, dy)))
                    .flatten()
                    .filter(|d| d.0 != 0 || d.1 != 0)
                    .map(|d| (i as i32 + d.0, j as i32 + d.1))
                    .filter(|p| p.0 >= 0 && p.0 < 10 && p.1 >= 0 && p.1 < 10)
                    .for_each(|p| {
                        let (x, y) = (p.0 as usize, p.1 as usize);
                        match matrix[x][y] {
                            0 => {}
                            9 => {
                                matrix[x][y] = 0;
                                queue.push_back((x, y));
                            }
                            _ => {
                                matrix[x][y] += 1;
                            }
                        }
                    });
            }

            flashes
        })
        .sum::<usize>();

    flashes == 1679
}

pub fn part2(_input: &Vec<String>) -> bool {
    unimplemented!();
}
