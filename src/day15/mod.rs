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

fn get_lowest_risk(matrix: &mut [[u8; N]; N]) -> usize {
    let mut heap = BinaryHeap::new();

    let start = (0 as usize, 0 as usize);
    heap.push((Reverse(0), start));

    while let Some((Reverse(risk), (x, y))) = heap.pop() {
        if (x, y) == (N - 1, N - 1) {
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
                if i < N && j < N && matrix[i][j] > 0 {
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

pub fn part2(_input: &Vec<String>) -> bool {
    unimplemented!();
}
