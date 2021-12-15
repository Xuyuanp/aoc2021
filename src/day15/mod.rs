use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::convert::TryInto;

const N: usize = 100;

fn parse_input(input: &Vec<String>) -> [[u8; N]; N] {
    let matrix: [[u8; N]; N] = input
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| c as u8 - b'0')
                .collect::<Vec<u8>>()
                .try_into()
                .unwrap()
        })
        .collect::<Vec<[u8; N]>>()
        .try_into()
        .unwrap();
    matrix
}

fn get_lowest_risk<const H: usize, const W: usize>(matrix: &mut [[u8; H]; W]) -> usize {
    let mut heap = BinaryHeap::new();

    let start = (0 as usize, 0 as usize);
    heap.push((Reverse(0), start));

    while let Some((Reverse(risk), (x, y))) = heap.pop() {
        if (x, y) == (H - 1, W - 1) {
            return risk;
        }
        // FIXME: how to find all neighbors?
        for neighbor in [
            (x.checked_sub(1), Some(y)),
            (Some(x + 1), Some(y)),
            (Some(x), y.checked_sub(1)),
            (Some(x), Some(y + 1)),
        ] {
            if let (Some(i), Some(j)) = neighbor {
                if i < H && j < W && matrix[i][j] > 0 {
                    let new_risk = risk + (matrix[i][j] as usize);
                    heap.push((Reverse(new_risk), (i, j)));
                    matrix[i][j] = 0;
                }
            }
        }
    }
    unreachable!();
}

pub fn part1(input: &Vec<String>) -> bool {
    let mut matrix = parse_input(input);
    let res = get_lowest_risk(&mut matrix);

    res == 745
}

pub fn part2(input: &Vec<String>) -> bool {
    let matrix = parse_input(input);
    const SCALE: usize = 5;
    let mut full_matrix = [[0; N * SCALE]; N * SCALE];
    for i in 0..SCALE {
        for j in 0..SCALE {
            for x in 0..N {
                for y in 0..N {
                    let delta = (i + j) % 9;
                    let ori = matrix[x][y] as usize;
                    let new = (ori + delta - 1) % 9 + 1;
                    full_matrix[i * N + x][j * N + y] = new as u8;
                }
            }
        }
    }
    let res = get_lowest_risk(&mut full_matrix);
    res == 3002
}
